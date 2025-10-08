use std::borrow::Cow;
use std::iter::Peekable;
use std::str::CharIndices;

use thiserror::Error;

use crate::chunk::LineNum;

use super::token::{Token, TokenKind};

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("Invalid character '{c}'")]
    BadChar { line: LineNum, c: char },
    #[error("Could not parse number literal '{n}'")]
    BadNumber { line: LineNum, n: String },
    #[error("Unterminated string")]
    UnterminatedString { line: LineNum },
}

impl ScannerError {
    pub fn line(&self) -> LineNum {
        *match self {
            Self::BadChar { line, c: _ }
            | Self::BadNumber { line, n: _ }
            | Self::UnterminatedString { line } => line,
        }
    }
}

pub type ScannerResult<T> = Result<T, ScannerError>;

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf::phf_map! {
    "and" => TokenKind::And,
    "or" => TokenKind::Or,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "while" => TokenKind::While,
    "for" => TokenKind::For,
    "class" => TokenKind::Class,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "fun" => TokenKind::Fun,
    "return" => TokenKind::Return,
    "nil" => TokenKind::Nil,
    "print" => TokenKind::Print,
    "var" => TokenKind::Var,
};

pub struct Scanner<'a> {
    source: &'a str,
    source_iter: Peekable<CharIndices<'a>>,
    start: usize,
    current: usize,
    line: LineNum,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            source_iter: source.char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Helper method to keep `source_iter` and current in sync
    fn advance(&mut self) -> Option<(usize, char)> {
        let (i, ch) = self.source_iter.next()?;
        self.current = i + ch.len_utf8();
        Some((i, ch))
    }

    /// Consume characters in the source iterator until the predicate is false or the iterator is
    /// exhausted.
    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some((_, ch)) = self.source_iter.peek()
            && predicate(*ch)
        {
            self.advance();
        }
    }

    fn make_lexeme(&self) -> Option<&'a str> {
        self.source.get(self.start..self.current)
    }

    fn make_token(&self, kind: TokenKind<'a>) -> Option<Token<'a>> {
        Some(Token {
            kind,
            line: self.line,
            lexeme: self.make_lexeme()?,
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.source_iter.peek()
            && ch.is_whitespace()
        {
            // This additional logic prevents the usage of consume_while
            if *ch == '\n' {
                self.line += 1;
            }

            self.advance();
        }
    }

    fn match_next(
        &mut self,
        guess: char,
        yes: TokenKind<'a>,
        no: TokenKind<'a>,
    ) -> Option<Token<'a>> {
        let (_, ch) = self.source_iter.peek()?;
        if *ch == guess {
            self.advance();
            Some(self.make_token(yes)?)
        } else {
            Some(self.make_token(no)?)
        }
    }

    fn string(&mut self) -> Option<ScannerResult<Token<'a>>> {
        self.consume_while(|c| c != '"');

        // Consume the closing double quote, or err if the string is unclosed
        match self
            .advance()
            .ok_or(ScannerError::UnterminatedString { line: self.line })
        {
            Ok(_) => {}
            Err(e) => return Some(Err(e)),
        }

        let lexeme = self.make_lexeme()?;
        let s = Cow::Borrowed(lexeme);
        let token = self.make_token(TokenKind::String(s))?;
        Some(Ok(token))
    }

    fn number(&mut self) -> Option<ScannerResult<Token<'a>>> {
        self.consume_while(|c| c.is_numeric() || c == '.');

        let lexeme = self.make_lexeme()?;
        let Ok(n) = lexeme.parse() else {
            let error = ScannerError::BadNumber {
                line: self.line,
                n: lexeme.to_string(),
            };

            return Some(Err(error));
        };

        let token = self.make_token(TokenKind::Number(n))?;
        Some(Ok(token))
    }

    fn identifier(&mut self) -> Option<Token<'a>> {
        self.consume_while(|c| is_ident_char(c) || c.is_numeric());

        let lexeme = self.make_lexeme()?;
        if let Some(keyword) = KEYWORDS.get(lexeme) {
            self.make_token(keyword.clone())
        } else {
            self.make_token(TokenKind::Identifier(lexeme))
        }
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

impl<'a> Iterator for Scanner<'a> {
    type Item = ScannerResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use TokenKind as TK;

            self.skip_whitespace();
            self.start = self.current;

            let (_, ch) = self.advance()?;

            let result = match ch {
                '(' => Ok(self.make_token(TK::LParen)?),
                ')' => Ok(self.make_token(TK::RParen)?),
                '{' => Ok(self.make_token(TK::LCurly)?),
                '}' => Ok(self.make_token(TK::RCurly)?),
                ';' => Ok(self.make_token(TK::Semicolon)?),
                ',' => Ok(self.make_token(TK::Comma)?),
                '.' => Ok(self.make_token(TK::Dot)?),
                '+' => Ok(self.make_token(TK::Plus)?),
                '-' => Ok(self.make_token(TK::Minus)?),
                '*' => Ok(self.make_token(TK::Star)?),

                '/' => {
                    if self.source_iter.peek()?.1 == '/' {
                        // '//' is a comment, so skip the rest of the line
                        // The newline itself will be handled on the next loop by skip_whitespace. It
                        // will also handle the increment of self.line
                        self.consume_while(|c| c != '\n');
                        continue;
                    }

                    Ok(self.make_token(TK::Slash)?)
                }

                '!' => Ok(self.match_next('=', TK::BangEq, TK::Bang)?),
                '=' => Ok(self.match_next('=', TK::EqEq, TK::Eq)?),
                '<' => Ok(self.match_next('=', TK::LtEq, TK::Lt)?),
                '>' => Ok(self.match_next('=', TK::GtEq, TK::Gt)?),

                '"' => self.string()?,
                c if c.is_numeric() => self.number()?,
                c if is_ident_char(c) => Ok(self.identifier()?),

                _ => Err(ScannerError::BadChar {
                    line: self.line,
                    c: ch,
                }),
            };

            return Some(result);
        }
    }
}
