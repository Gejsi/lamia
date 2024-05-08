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

#[macro_export]
macro_rules! assert_lex_eq {
    ($fn: expr, $lit: expr) => {
        assert_eq!(
            $fn.map(|(span, rest)| { (span.into_fragment(), rest) }),
            Ok(("", $lit))
        );
    };
}

pub fn lexer(i: Span) -> IResult<Span, &Token, LexerError> {
    todo!()
}
