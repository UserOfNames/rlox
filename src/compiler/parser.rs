use super::scanner::Scanner;

#[derive(Debug)]
pub struct Parser {
    pub erred: bool,
    pub panicking: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            erred: false,
            panicking: false,
        }
    }
}
