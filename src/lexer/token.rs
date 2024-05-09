use super::{Comment, Identifier, Keyword, Literal, Operator};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub position: Span<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind<'a> {
    /// Unknown token, not expected by the lexer, e.g. "№"
    Illegal,
    Eof,

    Identifier(Identifier<'a>),
    Literal(Literal),
    Operator(Operator),
    Keyword(Keyword),
    Comment(Comment<'a>),

    Assign {
        operator: Option<Operator>,
    },

    Comma,
    Semicolon,
    Colon,

    Group {
        delimiter: Delimiter,
        tokens: Vec<Token<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Delimiter {
    Paren,
    Square,
    Brace,
}
