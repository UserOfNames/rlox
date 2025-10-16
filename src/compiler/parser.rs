use super::scanner::{Scanner, ScannerError};
use super::token::Token;

#[derive(Debug)]
pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
    pub current: Token<'a>,
    pub previous: Token<'a>,
    pub erred: bool,
    pub panicking: bool,
}

impl<'a> Parser<'a> {
    pub fn new(mut scanner: Scanner<'a>) -> Self {
        // TODO: Proper error model
        let previous = scanner.next().unwrap().unwrap();
        let current = scanner.next().unwrap().unwrap();
        Self {
            scanner,
            current,
            previous,
            erred: false,
            panicking: false,
        }
    }

    fn advance(&mut self) -> Option<Token<'a>> {
        while let Some(token_res) = self.scanner.next() {
            match token_res {
                Ok(t) => return Some(t),
                Err(e) => self.report_err(&e),
            }
        }

        None
    }

    fn report_err(&mut self, error: &ScannerError) {
        self.erred = true;
        self.panicking = true;
        eprintln!("[Line {}] Syntax error: {}\n", error.line(), error);
    }
}
