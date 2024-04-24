use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alpha1, alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res, recognize, value},
    multi::many0,
    number::complete::double,
    sequence::{pair, preceded, terminated},
    IResult,
};

use super::LexerError;

pub type Identifier<'a> = &'a str;

pub fn lex_identifier(i: &str) -> IResult<&str, Identifier, LexerError> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(i)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    Bool(bool),
    Real(f64),
    Integer(i64),
    String(&'a str),
}

fn lex_bool(i: &str) -> IResult<&str, bool, LexerError> {
    alt((value(true, tag("true")), value(false, tag("false"))))(i)
}

fn lex_integer(i: &str) -> IResult<&str, i64, LexerError> {
    map_res(
        recognize(pair(digit1, many0(alt((digit1, tag("_")))))),
        |s: &str| s.replace("_", "").parse::<i64>(),
    )(i)
}

pub fn lex_literal(i: &str) -> IResult<&str, Literal, LexerError> {
    alt((
        map(lex_bool, Literal::Bool),
        map(double, Literal::Real),
        map(lex_integer, Literal::Integer),
        map(
            preceded(
                char('\"'),
                cut(terminated(
                    escaped(alphanumeric1, '\\', one_of("\"n\\")),
                    char('\"'),
                )),
            ),
            Literal::String,
        ),
    ))(i)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, Err as NErr};

    use crate::lexer::{lex_literal, Literal};

    use super::lex_identifier;

    #[test]
    fn match_simple_identifier() {
        assert_eq!(lex_identifier("varname"), Ok(("", "varname")));
    }

    #[test]
    fn match_underscore_identifier() {
        assert_eq!(lex_identifier("var_name"), Ok(("", "var_name")));
    }

    #[test]
    fn match_number_identifier() {
        assert_eq!(lex_identifier("var_name1"), Ok(("", "var_name1")));
    }

    #[test]
    fn match_indentifier_starting_underscore() {
        assert_eq!(lex_identifier("_var_name"), Ok(("", "_var_name")));
    }

    #[test]
    fn not_match_identifier_number() {
        assert_eq!(
            lex_identifier("1var_name"),
            Err(NErr::Error(("1var_name", ErrorKind::Tag)))
        );
    }

    #[test]
    fn match_true() {
        assert_eq!(lex_literal("true"), Ok(("", Literal::Bool(true))));
    }

    #[test]
    fn match_false() {
        assert_eq!(lex_literal("false"), Ok(("", Literal::Bool(false))));
    }

    // #[test]
    // fn match_simple_int() {
    //     assert_eq!(lex_literal("123"), Ok(("", Literal::Integer(123))));
    // }

    // #[test]
    // fn match_int_underscore() {
    //     assert_eq!(lex_literal("1_000"), Ok(("", Literal::Integer(1000))));
    // }

    // #[test]
    // fn match_real() {
    //     assert_eq!(lex_literal("1.23"), Ok(("", Literal::Real(1.23))));
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
