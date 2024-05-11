use nom::IResult;

pub mod comment;
pub mod error;
pub mod group;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod token;

pub use error::LexerError;

pub use comment::{lex_comment, Comment};
pub use group::{lex_group_brace, lex_group_paren, lex_group_square, Delimiter};
pub use identifier::{lex_identifier, Identifier};
pub use keyword::{lex_keyword, Keyword};
pub use literal::{lex_literal, Literal};
pub use operator::{lex_operator, Operator};
pub use punctuation::{lex_punctuation, Punctuation};
pub use token::{Span, Token, TokenKind};

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
