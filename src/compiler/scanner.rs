// TODO: Do i + ch.len_utf8() or source_iter.peek()? Or wrap char_indices in a custom iteator?
// TODO: Remove self.current?

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

// We need the first byte of the next character, not the current character, to use as an exclusive
// upper bound for lexeme generation. To make this a bit more convenient, we use this wrapper
// around the base CharIndices that performs the offset calculation automatically.
struct CharIndicesNext<'a> {
    inner: CharIndices<'a>,
}

impl<'a> CharIndicesNext<'a> {
    fn new(inner: CharIndices<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Iterator for CharIndicesNext<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, ch) = self.inner.next()?;
        Some((i + ch.len_utf8(), ch))
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    source_iter: Peekable<CharIndicesNext<'a>>,
    start: usize,
    current: usize,
    line: LineNum,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            // TODO: Make this postfix
            source_iter: CharIndicesNext::new(source.char_indices()).peekable(),
            start: 0,
            current: 0,
            line: 1,
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
        while let Some((i, ch)) = self.source_iter.peek()
            && ch.is_whitespace()
        {
            if *ch == '\n' {
                self.line += 1;
            }

            self.current = *i;
            self.source_iter.next();
        }
    }

    fn guess(&mut self, guess: char, yes: TokenKind<'a>, no: TokenKind<'a>) -> Option<Token<'a>> {
        let (_, ch) = self.source_iter.peek()?;
        if *ch == guess {
            self.source_iter.next();
            Some(self.make_token(yes)?)
        } else {
            Some(self.make_token(no)?)
        }
    }

    fn string(&mut self) -> InterpretResult<Token<'a>> {
        while let Some((i, ch)) = self.source_iter.next() {
            if ch == '"' {
                self.current = i;
                // TODO: Think about this unwrap
                let lexeme = self.make_lexeme().unwrap();
                let s = Cow::Borrowed(lexeme);
                let token = self.make_token(TokenKind::String(s)).unwrap();
                return Ok(token);
            }
        }

        // If we get here, the string was not closed
        Err(InterpretError::Compiler)
    }

    fn number(&mut self) -> InterpretResult<Token<'a>> {
        while let Some((i, ch)) = self.source_iter.peek()
            && (ch.is_numeric() || *ch == '.')
        {
            self.current = *i;
            self.source_iter.next();
        }

        // TODO: Think about this unwrap
        let lexeme = self.make_lexeme().unwrap();
        let n = lexeme.parse()?;
        let token = self.make_token(TokenKind::Number(n)).unwrap();
        Ok(token)
    }

    fn identifier(&mut self) -> Token<'a> {
        while let Some((i, ch)) = self.source_iter.peek()
            && (is_ident_char(*ch) || ch.is_numeric())
        {
            self.current = *i;
            self.source_iter.next();
        }

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
        let (i, ch) = self.source_iter.next()?;
        self.current = i;

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
