use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{pair, separated_pair, tuple},
    IResult,
};
use thiserror::Error;

use crate::lexer::LexerError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Number {
    pub value: NumberValue,
    pub kind: NumberKind,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum NumberValue {
    Integer(u64),
    Floating(f64),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NumberKind(pub char, pub BitCount);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BitCount {
    _8,
    _16,
    _32,
    _64,
}

impl Display for BitCount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = match self {
            BitCount::_8 => 8,
            BitCount::_16 => 16,
            BitCount::_32 => 32,
            BitCount::_64 => 64,
        };
        write!(f, "{}", n)
    }
}

impl Display for NumberKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Error, Debug)]
enum NumberParsingError {
    #[error("Failed to parse to as an u64: {0}")]
    ParseIntegerError(#[from] ParseIntError),

    #[error("Failed to parse to as an f64: {0}")]
    ParseFloatingError(#[from] ParseFloatError),

    #[error("Failed to parse {0} containing exponent")]
    IntegerWithExponent(NumberKind),
}

fn remove_number_sugar(i: &str) -> String {
    i.replace('_', "")
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

fn lex_number_kind(i: &str) -> IResult<&str, NumberKind, LexerError> {
    map(
        pair(alt((char('f'), char('u'), char('i'))), lex_bit_count),
        |(c, b)| NumberKind(c, b),
    )(i)
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

fn parse_number(
    (value, exp, kind): (String, Option<i64>, Option<NumberKind>),
) -> Result<Number, NumberParsingError> {
    let parsed_value = match (&kind, value.contains('.'), exp) {
        // Parse as a float if it has the appropriate suffix or contains a dot or contains exponent
        (Some(NumberKind('f', _)), _, _) | (_, true, _) | (_, _, Some(_)) => value
            .parse::<f64>()
            .map(|f| NumberValue::Floating(f * (10f64.powi(exp.unwrap_or(0) as i32))))
            .map_err(NumberParsingError::ParseFloatingError),
        _ => value
            .parse::<u64>()
            .map(NumberValue::Integer)
            .map_err(NumberParsingError::ParseIntegerError),
    }?;

    let parsed_kind = match (exp, kind) {
        (Some(_), None) => Ok(NumberKind('f', BitCount::_64)),
        (Some(_), Some(NumberKind('f', size))) => Ok(NumberKind('f', size)),
        (Some(_), Some(n @ NumberKind('i', _))) | (Some(_), Some(n @ NumberKind('u', _))) => {
            Err(NumberParsingError::IntegerWithExponent(n))
        }
        (_, kind) => Ok(kind.unwrap_or(match parsed_value {
            NumberValue::Integer(_) => NumberKind('i', BitCount::_32),
            NumberValue::Floating(_) => NumberKind('f', BitCount::_64),
        })),
    }?;

    Ok(Number {
        value: parsed_value,
        kind: parsed_kind,
    })
}

pub fn lex_number(i: &str) -> IResult<&str, Number, LexerError> {
    map_res(
        tuple((
            alt((lex_float, lex_decimal)),
            opt(lex_exponent),
            opt(lex_number_kind),
        )),
        parse_number,
    )(i)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, Err as NErr};

    use crate::lexer::{
        lex_literal,
        literal::number::{BitCount, Number, NumberKind, NumberValue},
        Literal,
    };

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
    fn match_unsigned_integers() {
        assert_number_expr!(
            "123u8",
            NumberKind('u', BitCount::_8),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123u16",
            NumberKind('u', BitCount::_16),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123u32",
            NumberKind('u', BitCount::_32),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123u64",
            NumberKind('u', BitCount::_64),
            NumberValue::Integer(123)
        );
    }

    #[test]
    fn match_signed_integers() {
        assert_number_expr!(
            "123i8",
            NumberKind('i', BitCount::_8),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123i16",
            NumberKind('i', BitCount::_16),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123i32",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer(123)
        );
        assert_number_expr!(
            "123i64",
            NumberKind('i', BitCount::_64),
            NumberValue::Integer(123)
        );
    }

    #[test]
    fn match_f64() {
        assert_number_expr!(
            "123_100.0",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123_100.0)
        );
        assert_number_expr!(
            "42e42",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(42e42)
        );
        assert_number_expr!(
            "42.42e2",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(42.42e2)
        );
        // TODO: this doesn't work due to a overflow error
        // assert_number_expr!(
        //     "42.42e42",
        //     NumberKind('f', BitCount::_64),
        //     NumberValue::Floating(42.42e42)
        // );
        assert_number_expr!(
            "123.23f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23)
        );
        assert_number_expr!(
            "123.23e10f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23e10)
        );
        assert_number_expr!(
            "123.23E10f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23E10)
        );
        assert_number_expr!(
            "123.23e10",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23e10)
        );
        assert_number_expr!(
            "123E10",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123E10)
        );
    }

    #[test]
    fn match_f32() {
        assert_number_expr!(
            "123f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.0)
        );
        assert_number_expr!(
            "123.23f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23)
        );
        assert_number_expr!(
            "123.23e10f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23e10)
        );
        assert_number_expr!(
            "123.23E10f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23E10)
        );
    }

    // TODO: refactor these asserts into separate named tests
    #[test]
    fn edge_cases() {
        // number with exp cannot be an int
        assert_eq!(
            lex_literal("123e3_i32"),
            Err(NErr::Error(("123e3_i32", ErrorKind::MapRes)))
        );
        // number with exp must be followed by decimal
        assert_eq!(
            lex_literal("123e_f32"),
            Err(NErr::Failure(("_f32", ErrorKind::Digit)))
        );
        // ....more....
    }
}
