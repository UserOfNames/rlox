use super::scanner::{Scanner, ScannerError};
use super::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
    pub current: Token<'a>,
    pub previous: Token<'a>,
    pub erred: bool,
    pub panicking: bool,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            current: Token::new_undefined(),
            previous: Token::new_undefined(),
            erred: false,
            panicking: false,
        }
    }

    fn advance(&mut self) -> Option<Token<'a>> {
        while let Some(token_res) = self.scanner.next() {
            match token_res {
                Ok(t) => return Some(t),
                Err(e) => self.report_err(&e, "Syntax error"),
            }
        }

        None
    }

    fn consume(&mut self, kind: &TokenKind, err_message: &str) {
        if matches!(&self.current.kind, kind) {
            self.advance();
        }
    }

    fn report_err(&mut self, error: &ScannerError, message: &str) {
        self.erred = true;
        self.panicking = true;
        eprintln!("[Line {}] {message}: {}\n", error.line(), error);
    }
}
