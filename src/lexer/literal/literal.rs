use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, one_of},
    combinator::{cut, map, value},
    sequence::{preceded, terminated},
    IResult,
};

use crate::lexer::LexerError;

use super::number::{lex_number, Number};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    String(&'a str),
    Bool(bool),
    Number(Number),
}

fn lex_bool(i: &str) -> IResult<&str, bool, LexerError> {
    alt((value(true, tag("true")), value(false, tag("false"))))(i)
}

fn lex_string(i: &str) -> IResult<&str, &str, LexerError> {
    preceded(
        char('\"'),
        cut(terminated(
            escaped(alphanumeric1, '\\', one_of("\"n\\")),
            char('\"'),
        )),
    )(i)
}

pub fn lex_literal(i: &str) -> IResult<&str, Literal, LexerError> {
    alt((
        map(lex_bool, Literal::Bool),
        map(lex_string, Literal::String),
        map(lex_number, Literal::Number),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::lexer::{lex_literal, Literal};

    #[test]
    fn match_bool() {
        assert_eq!(lex_literal("true"), Ok(("", Literal::Bool(true))));
        assert_eq!(lex_literal("false"), Ok(("", Literal::Bool(false))));
    }

    #[test]
    fn match_simple_string() {
        assert_eq!(lex_literal("\"test\""), Ok(("", Literal::String("test"))));
    }

    #[test]
    fn match_escaped_string() {
        assert_eq!(
            lex_literal("\"test\\\"\""),
            Ok(("", Literal::String("test\\\"")))
        );
    }

    #[test]
    fn match_newline_string() {
        assert_eq!(
            lex_literal("\"test\\n\""),
            Ok(("", Literal::String("test\\n")))
        );
    }
}
