use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;
use std::sync::LazyLock;

use crate::chunk::LineNum;
use crate::{InterpretError, InterpretResult};

use super::token::{Token, TokenKind};

// TODO: Not the best solution. The book recommends a trie; my understanding, from 30 seconds of
// googling, is something about 'perfect hashing.' I'll look into that later. In any case, wrapping
// a dynamic hashset for a set of values that are all known at compile time definitely sucks.
static KEYWORDS: LazyLock<HashMap<&'static str, TokenKind>> = LazyLock::new(|| {
    let mut hs = HashMap::new();
    hs.insert("and", TokenKind::And);
    hs.insert("or", TokenKind::Or);
    hs.insert("true", TokenKind::True);
    hs.insert("false", TokenKind::False);
    hs.insert("if", TokenKind::If);
    hs.insert("else", TokenKind::Else);
    hs.insert("while", TokenKind::While);
    hs.insert("for", TokenKind::For);
    hs.insert("class", TokenKind::Class);
    hs.insert("super", TokenKind::Super);
    hs.insert("this", TokenKind::This);
    hs.insert("fun", TokenKind::Fun);
    hs.insert("return", TokenKind::Return);
    hs.insert("nil", TokenKind::Nil);
    hs.insert("print", TokenKind::Print);
    hs.insert("var", TokenKind::Var);
    hs
});

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

    /// Helper method to keep source_iter and current in sync
    fn inner_next(&mut self) -> Option<(usize, char)> {
        let (i, ch) = self.source_iter.next()?;
        self.current = i + ch.len_utf8();
        Some((i, ch))
    }

    /// Consume characters in the source iterator until the predicate is false of the iterator is
    /// exhausted.
    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some((_, ch)) = self.source_iter.peek()
            && predicate(*ch)
        {
            self.inner_next();
        }
    }

    fn make_lexeme(&self) -> Option<&'a str> {
        self.source.get(self.start..self.current)
    }

    fn make_token(&self, kind: TokenKind<'a>) -> Option<Token<'a>> {
        Some(Token {
            kind,
            line: self.line,
            // TODO: Is passing this error necessary?
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

            self.inner_next();
        }
    }

    fn guess(&mut self, guess: char, yes: TokenKind<'a>, no: TokenKind<'a>) -> Option<Token<'a>> {
        let (_, ch) = self.source_iter.peek()?;
        if *ch == guess {
            self.inner_next();
            Some(self.make_token(yes)?)
        } else {
            Some(self.make_token(no)?)
        }
    }

    fn string(&mut self) -> InterpretResult<Token<'a>> {
        self.consume_while(|c| c != '"');

        // Consume the closing double quote. consume_while() already peeked at this value, so we
        // know it exists and is a double quote.
        self.inner_next().unwrap();

        let lexeme = self.make_lexeme().unwrap();
        let s = Cow::Borrowed(lexeme);
        let token = self.make_token(TokenKind::String(s)).unwrap();
        Ok(token)
    }

    fn number(&mut self) -> InterpretResult<Token<'a>> {
        self.consume_while(|c| c.is_numeric() || c == '.');

        // TODO: Think about this unwrap
        let lexeme = self.make_lexeme().unwrap();
        let n = lexeme.parse()?;
        let token = self.make_token(TokenKind::Number(n)).unwrap();
        Ok(token)
    }

    fn identifier(&mut self) -> Token<'a> {
        self.consume_while(|c| is_ident_char(c) || c.is_numeric());

        // TODO: Think about this unwrap
        let lexeme = self.make_lexeme().unwrap();
        if let Some(keyword) = KEYWORDS.get(lexeme) {
            // TODO: Think about this unwrap
            self.make_token(keyword.clone()).unwrap()
        } else {
            self.make_token(TokenKind::Identifier(lexeme)).unwrap()
        }
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

impl<'a> Iterator for Scanner<'a> {
    type Item = InterpretResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.start = self.current;
        let (_, ch) = self.inner_next()?;

        use TokenKind as TK;
        Some(match ch {
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
                // HACK: This is disgusting, fix this later
                if self.source_iter.peek()?.1 == '/' {
                    while let Some((_, ch)) = self.source_iter.next()
                        && ch != '\n'
                    {
                        self.current += 1;
                    }
                    self.current += 1;
                    self.line += 1;
                    self.next()?
                } else {
                    Ok(self.make_token(TK::Slash)?)
                }
            }

            '!' => Ok(self.guess('=', TK::BangEq, TK::Bang)?),
            '=' => Ok(self.guess('=', TK::EqEq, TK::Eq)?),
            '<' => Ok(self.guess('=', TK::LtEq, TK::Lt)?),
            '>' => Ok(self.guess('=', TK::GtEq, TK::Gt)?),

            '"' => self.string(),
            c if c.is_numeric() => self.number(),
            c if is_ident_char(c) => Ok(self.identifier()),

            _ => Err(InterpretError::Compiler),
        })
    }
}
