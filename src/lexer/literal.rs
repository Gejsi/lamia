use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    multi::many0,
    number::complete::double,
    sequence::{pair, preceded, terminated},
    IResult,
};

use super::LexerError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    String(&'a str),
    Bool(bool),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

fn lex_bool(i: &str) -> IResult<&str, bool, LexerError> {
    alt((value(true, tag("true")), value(false, tag("false"))))(i)
}

fn lex_integer(i: &str) -> IResult<&str, i64, LexerError> {
    map_res(
        recognize(pair(
            opt(char('-')),
            pair(digit1, many0(alt((digit1, tag("_"))))),
        )),
        |s: &str| s.replace("_", "").parse::<i64>(),
    )(i)
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
        map(lex_integer, Literal::Long),
        map(double, Literal::Double),
        map(lex_string, Literal::String),
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
    fn match_simple_long() {
        assert_eq!(lex_literal("123"), Ok(("", Literal::Long(123))));
        assert_eq!(lex_literal("-123"), Ok(("", Literal::Long(-123))));
    }

    // #[test]
    // fn match_int_underscore() {
    //     assert_eq!(lex_literal("1_000"), Ok(("", Literal::Integer(1000))));
    // }

    // #[test]
    // fn match_double() {
    //     assert_eq!(lex_literal("1.23"), Ok(("", Literal::Double(1.23))));
    // }

    // #[test]
    // fn match_double_with_multiple_decimals() {
    //     assert_eq!(lex_literal("1.1.1"), Err(NErr::Error(("", ErrorKind::Tag))));
    // }

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
