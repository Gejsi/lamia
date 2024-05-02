use nom::{character::complete::char, combinator::value, IResult};

pub mod error;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod token;
pub mod syntax;

pub use error::LexerError;

pub use identifier::{lex_identifier, Identifier};
pub use keyword::{lex_keyword, Keyword};
pub use literal::{lex_literal, Literal};
pub use operator::{lex_operator, Operator};
pub use token::{Span, Token, TokenKind, Delimiter};
pub use syntax::{lex_colon, lex_semicolon, lex_comma};

pub fn lexer(i: Span) -> IResult<Span, &Token, LexerError> {
    todo!()
}