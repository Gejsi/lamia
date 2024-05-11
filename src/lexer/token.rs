use super::{Comment, Delimiter, Identifier, Keyword, Literal, Operator, Punctuation};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub position: Span<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind<'a> {
    Identifier(Identifier<'a>),
    Comment(Comment<'a>),
    Literal(Literal),
    Operator(Operator),
    Keyword(Keyword),
    Punctuation(Punctuation),

    Assign {
        operator: Option<Operator>,
    },

    Group {
        delimiter: Delimiter,
        tokens: Vec<Token<'a>>,
    },

    /// Unknown token, not expected by the lexer, e.g. "№"
    Illegal,
    Eof,
}
