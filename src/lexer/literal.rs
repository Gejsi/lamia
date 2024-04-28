use std::{fmt::Display, num::ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use thiserror::Error;

use super::LexerError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    String(&'a str),
    Bool(bool),
    Number(Number),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BitCount {
    _8,
    _16,
    _32,
    _64,
}

impl Display for BitCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = match self {
            BitCount::_8 => 8,
            BitCount::_16 => 16,
            BitCount::_32 => 32,
            BitCount::_64 => 64,
        };
        write!(f, "{}", n)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum NumberValue {
    Integer(u64),
    Floating(f64),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NumberKind(char, BitCount);

impl Display for NumberKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Number {
    pub kind: Option<NumberKind>,
    pub value: NumberValue,
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

fn remove_number_sugar(i: &str) -> String {
    i.replace("_", "")
}

fn lex_decimal(i: &str) -> IResult<&str, String, LexerError> {
    map(
        recognize(pair(digit1, many0(alt((digit1, tag("_")))))),
        remove_number_sugar,
    )(i)
}

fn lex_float(i: &str) -> IResult<&str, String, LexerError> {
    map(
        separated_pair(lex_decimal, char('.'), cut(lex_decimal)),
        |(int_part, frac_part)| remove_number_sugar(&format!("{int_part}.{frac_part}")),
    )(i)
}

fn lex_bit_count(i: &str) -> IResult<&str, BitCount, LexerError> {
    alt((
        value(BitCount::_8, char('8')),
        value(BitCount::_16, tag("16")),
        value(BitCount::_32, tag("32")),
        value(BitCount::_64, tag("64")),
    ))(i)
}

fn lex_number_type(i: &str) -> IResult<&str, NumberKind, LexerError> {
    map(pair(alt((char('f'), char('u'), char('i'))), lex_bit_count), |(c,b)| NumberKind(c,b))(i)
}

fn lex_exponent(i: &str) -> IResult<&str, i64, LexerError> {
    map_res(
        tuple((one_of("eE"), opt(one_of("+-")), cut(lex_decimal))),
        |(_, sign, decimal)| {
            let n = format!("{}{}", sign.unwrap_or('+'), remove_number_sugar(&decimal));
            n.parse::<i64>()
        },
    )(i)
}

// TODO: proper number error handling pls
#[derive(Error, Debug)]
enum NumberParsingError {
    #[error("")]
    LexError,

    #[error("Failed to parse to a 32 bit integer: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Failed to parse {0} which contains exponent")]
    IntegerWithExponent(NumberKind),
}

fn parse_number(
    value: String,
    exp: Option<i64>,
    kind: Option<NumberKind>,
) -> Result<Number, NumberParsingError> {
    let final_kind = match (exp, kind) {
        (Some(_), None) => Ok(Some(('f', BitCount::_64))),
        (Some(_), Some(NumberKind('f', size))) => Ok(Some(('f', size))),
        (Some(_), Some(NumberKind('i', _))) | (Some(_), Some(NumberKind('u', _))) => {
            return Err(NumberParsingError::LexError)
        }
        (_, kind) => Ok(kind),
    }?;
    let has_dot = value.contains('.');
    let parsed_value = match (final_kind.clone(), has_dot) {
        // Parse as a float if it's hard-coded or if the number contains a dot
        (Some(('f', _)), _) | (_, true) => value
            .parse::<f64>()
            .map(|f| NumberValue::Floating(exp.unwrap_or(1) as f64 * f))?,
        _ => value.parse::<u64>().map(NumberValue::Integer)?,
    }?;

    Ok(Number {
        kind: final_kind,
        value: parsed_value,
    })
}

fn lex_number(i: &str) -> IResult<&str, Number, LexerError> {
    map_res(
        tuple((
            alt((lex_float, lex_decimal)),
            opt(lex_exponent),
            opt(lex_number_type),
        )),
        |(value, exp, kind)| parse_number(value, exp, kind),
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
    use crate::lexer::{
        lex_literal,
        literal::{BitCount, Number, NumberValue},
        Literal,
    };

    #[test]
    fn match_bool() {
        assert_eq!(lex_literal("true"), Ok(("", Literal::Bool(true))));
        assert_eq!(lex_literal("false"), Ok(("", Literal::Bool(false))));
    }

    macro_rules! assert_number_expr {
        ($n: expr, $kind: expr, $value: expr) => {
            assert_eq!(
                lex_literal($n),
                Ok((
                    "",
                    Literal::Number(Number {
                        kind: $kind,
                        value: $value
                    })
                ))
            );
        };
    }

    #[test]
    fn match_u64() {
        assert_number_expr!(
            "123u64",
            Some(('u', BitCount::_64)),
            NumberValue::Integer(123)
        );

        // Without suffix
        assert_number_expr!("123", None, NumberValue::Integer(123));
    }

    #[test]
    fn match_i64() {
        assert_number_expr!(
            "123u64",
            Some(('i', BitCount::_64)),
            NumberValue::Integer(123)
        );
    }

    fn match_f64() {
        assert_number_expr!(
            "123f64",
            Some(('f', BitCount::_32)),
            NumberValue::Floating(123f64)
        );
        assert_number_expr!(
            "123.23f64",
            Some(('f', BitCount::_64)),
            NumberValue::Floating(123.23f64)
        );
        assert_number_expr!(
            "123.23e10f64",
            Some(('f', BitCount::_64)),
            NumberValue::Floating(123.23e10f64)
        );
        assert_number_expr!(
            "123.23E10f64",
            Some(('f', BitCount::_64)),
            NumberValue::Floating(123.23E10f64)
        );

        // Without suffix
        assert_number_expr!("123.0", None, NumberValue::Floating(123.0));
        assert_number_expr!("123.23", None, NumberValue::Floating(123.23));
        assert_number_expr!("123.23e10", None, NumberValue::Floating(123.23e10));
        assert_number_expr!("123.23E10", None, NumberValue::Floating(123.23E10));
    }

    fn match_f32() {
        assert_number_expr!(
            "123f32",
            Some(('f', BitCount::_32)),
            NumberValue::Floating(123.0)
        );
        assert_number_expr!(
            "123.23f32",
            Some(('f', BitCount::_32)),
            NumberValue::Floating(123.23)
        );
        assert_number_expr!(
            "123.23e10f32",
            Some(('f', BitCount::_32)),
            NumberValue::Floating(123.23e10)
        );
        assert_number_expr!(
            "123.23E10f32",
            Some(('f', BitCount::_32)),
            NumberValue::Floating(123.23E10)
        );
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
