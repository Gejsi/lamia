use nom::IResult;

pub mod error;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod syntax;
pub mod token;

pub use error::LexerError;

pub use identifier::{lex_identifier, Identifier};
pub use keyword::{lex_keyword, Keyword};
pub use literal::{lex_literal, Literal};
pub use operator::{lex_operator, Operator};
pub use syntax::{lex_colon, lex_comma, lex_semicolon};
pub use token::{Delimiter, Span, Token, TokenKind};

macro_rules! assert_lex_eq {
    ($fn: expr, $lit: expr) => {
        match $fn {
            Ok((span, rest)) => {
                assert_eq!((span.into_fragment(), rest), ("", $lit));
            }
            Err(err) => {
                assert!(false, "{err}");
            }
        }
    };
}

pub(crate) use assert_lex_eq;

pub fn lexer(i: Span) -> IResult<Span, &Token, LexerError> {
    todo!()
}
