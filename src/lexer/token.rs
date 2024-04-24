use super::{Identifier, Keyword, Literal, Operator};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub literal: &'a str,
    pub source: TokenSource,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKind<'a> {
    /// Unknown token, not expected by the lexer, e.g. "№"
    Illegal,
    Eof,

    Identifier(Identifier<'a>),
    Literal(Literal<'a>),
    Operator(Operator),
    Keyword(Keyword),

    Assign {
        operator: Option<Operator>,
    },

    Comma,
    Semicolon,
    Colon,

    Group {
        delimiter: Delimiter,
        tokens: &'a [Token<'a>],
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Delimiter {
    Paren,
    Square,
    Brace,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TokenSource {
    pub position: (usize, usize),
}
