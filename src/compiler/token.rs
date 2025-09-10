use std::{borrow::Cow, fmt::Display};

use crate::chunk::LineNum;

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
    // Single character tokens
    LParen, RParen,
    LCurly, RCurly,
    Comma, Dot, Semicolon,
    Plus, Minus, Star, Slash,

    // One or two character tokens
    Bang, BangEq,
    Eq, EqEq,
    Gt, GtEq,
    Lt, LtEq,

    // Literals
    Identifier(&'a str),
    String(Cow<'a, str>),
    Number(f64),

    // Keywords
    And, Or, True, False,
    If, Else, While, For,
    Class, Super, This, Fun, Return,
    Nil, Print, Var,
}

impl Display for TokenKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::LCurly => write!(f, "LCurly"),
            TokenKind::RCurly => write!(f, "RCurly"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::Slash => write!(f, "Slash"),

            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::BangEq => write!(f, "BangEq"),
            TokenKind::Eq => write!(f, "Eq"),
            TokenKind::EqEq => write!(f, "EqEq"),
            TokenKind::Gt => write!(f, "Gt"),
            TokenKind::GtEq => write!(f, "GtEq"),
            TokenKind::Lt => write!(f, "Lt"),
            TokenKind::LtEq => write!(f, "LtEq"),

            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
            TokenKind::String(s) => write!(f, "String({s})"),
            TokenKind::Number(n) => write!(f, "Number({n})"),

            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::True => write!(f, "True"),
            TokenKind::False => write!(f, "False"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::While => write!(f, "While"),
            TokenKind::For => write!(f, "For"),
            TokenKind::Class => write!(f, "Class"),
            TokenKind::Super => write!(f, "Super"),
            TokenKind::This => write!(f, "This"),
            TokenKind::Fun => write!(f, "Func"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Nil => write!(f, "None"),
            TokenKind::Print => write!(f, "Print"),
            TokenKind::Var => write!(f, "Let"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub lexeme: &'a str,
    pub line: LineNum,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} '{}'", self.line, self.kind, self.lexeme)
    }
}
