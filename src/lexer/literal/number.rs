use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, hex_digit1, oct_digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    multi::{many0, many1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use thiserror::Error;

use crate::lexer::{LexerError, Span};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Number {
    pub value: NumberValue,
    pub kind: NumberKind,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NumberValue {
    Integer { value: u128, kind: IntegerKind },
    Floating(f64),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerKind {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NumberKind(pub char, pub BitCount);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BitCount {
    _8,
    _16,
    _32,
    _64,
    _128,
    Size,
}

impl Display for BitCount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = match self {
            BitCount::_8 => 8,
            BitCount::_16 => 16,
            BitCount::_32 => 32,
            BitCount::_64 => 64,
            BitCount::_128 => 128,
            BitCount::Size => 8 * std::mem::size_of::<usize>(),
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
    #[error("Failed to parse to as an u128: {0}")]
    ParseIntegerError(#[from] ParseIntError),

    #[error("Failed to parse to as an f64: {0}")]
    ParseFloatingError(#[from] ParseFloatError),

    #[error("Failed to parse {0} containing exponent")]
    IntegerWithExponent(NumberKind),

    #[error("Invalid floating point precision: {0}. Only `f32` and `f64` are allowed")]
    InvalidFloatPrecision(NumberKind),
}

fn desugar(i: &str) -> String {
    i.replace('_', "")
}

fn lex_hexadecimal(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        recognize(tuple((
            alt((tag("0x"), tag("0X"))),
            many0(char('_')),
            pair(hex_digit1, many0(alt((hex_digit1, tag("_"))))),
        ))),
        |s: Span| s.into_fragment(),
    )(i)
}

fn lex_octal(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        recognize(tuple((
            alt((tag("0o"), tag("0O"))),
            many0(char('_')),
            pair(oct_digit1, many0(alt((oct_digit1, tag("_"))))),
        ))),
        |s: Span| s.into_fragment(),
    )(i)
}

fn lex_binary(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        recognize(tuple((
            alt((tag("0b"), tag("0B"))),
            many0(char('_')),
            many1(terminated(one_of("01"), many0(char('_')))),
        ))),
        |s: Span| s.into_fragment(),
    )(i)
}

fn lex_decimal(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        recognize(pair(digit1, many0(alt((digit1, tag("_")))))),
        |s: Span| s.into_fragment(),
    )(i)
}

fn lex_integer(i: Span) -> IResult<Span, String, LexerError> {
    map(
        alt((lex_hexadecimal, lex_octal, lex_binary, lex_decimal)),
        desugar,
    )(i)
}

fn lex_float(i: Span) -> IResult<Span, String, LexerError> {
    map(
        separated_pair(lex_decimal, char('.'), cut(lex_decimal)),
        |(int_part, frac_part)| desugar(&format!("{int_part}.{frac_part}")),
    )(i)
}

fn lex_bit_count(i: Span) -> IResult<Span, BitCount, LexerError> {
    alt((
        value(BitCount::_8, char('8')),
        value(BitCount::_16, tag("16")),
        value(BitCount::_32, tag("32")),
        value(BitCount::_64, tag("64")),
        value(BitCount::_128, tag("128")),
        value(BitCount::Size, tag("size")),
    ))(i)
}

fn lex_number_kind(i: Span) -> IResult<Span, NumberKind, LexerError> {
    map(
        pair(alt((char('f'), char('u'), char('i'))), lex_bit_count),
        |(c, b)| NumberKind(c, b),
    )(i)
}

fn lex_exponent(i: Span) -> IResult<Span, i64, LexerError> {
    map_res(
        tuple((one_of("eE"), opt(one_of("+-")), cut(lex_decimal))),
        |(_, sign, decimal)| {
            let n = format!("{}{}", sign.unwrap_or('+'), desugar(decimal));
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

        // Otherwise, parse as an integer
        _ => {
            let val = if value.starts_with("0x") || value.starts_with("0X") {
                u128::from_str_radix(&value[2..], 16).map(|i| NumberValue::Integer {
                    value: i,
                    kind: IntegerKind::Hexadecimal,
                })
            } else if value.starts_with("0o") || value.starts_with("0O") {
                u128::from_str_radix(&value[2..], 8).map(|i| NumberValue::Integer {
                    value: i,
                    kind: IntegerKind::Octal,
                })
            } else if value.starts_with("0b") || value.starts_with("0B") {
                u128::from_str_radix(&value[2..], 2).map(|i| NumberValue::Integer {
                    value: i,
                    kind: IntegerKind::Binary,
                })
            } else {
                value.parse::<u128>().map(|i| NumberValue::Integer {
                    value: i,
                    kind: IntegerKind::Decimal,
                })
            };

            val.map_err(NumberParsingError::ParseIntegerError)
        }
    }?;

    let parsed_kind = match (exp, kind) {
        (Some(_), None) => Ok(NumberKind('f', BitCount::_64)),
        (Some(_), Some(n @ NumberKind('f', _))) => Ok(n),
        (Some(_), Some(n @ NumberKind('i', _))) | (Some(_), Some(n @ NumberKind('u', _))) => {
            Err(NumberParsingError::IntegerWithExponent(n))
        }
        (_, kind) => Ok(kind.unwrap_or(match parsed_value {
            // integers default to `i32`
            NumberValue::Integer { value: _, kind: _ } => NumberKind('i', BitCount::_32),
            // floats default to `f64`
            NumberValue::Floating(_) => NumberKind('f', BitCount::_64),
        })),
    }?;

    if parsed_kind.0 == 'f' && parsed_kind.1 != BitCount::_32 && parsed_kind.1 != BitCount::_64 {
        return Err(NumberParsingError::InvalidFloatPrecision(parsed_kind));
    }

    Ok(Number {
        value: parsed_value,
        kind: parsed_kind,
    })
}

pub fn lex_number(i: Span) -> IResult<Span, Number, LexerError> {
    map_res(
        tuple((
            alt((lex_float, lex_integer)),
            opt(lex_exponent),
            opt(lex_number_kind),
        )),
        parse_number,
    )(i)
}

#[cfg(test)]
mod tests {
    use crate::assert_lex_eq;
    use crate::lexer::{
        lex_literal,
        literal::number::{BitCount, IntegerKind, Number, NumberKind, NumberValue},
        Literal,
    };

    macro_rules! assert_number_eq {
        ($text: expr, $kind: expr, $value: expr) => {
            assert_lex_eq!(
                lex_literal($text.into()),
                Literal::Number(Number {
                    value: $value,
                    kind: $kind
                })
            );
        };
    }

    #[test]
    fn match_unsigned_integers() {
        assert_number_eq!(
            "123u8",
            NumberKind('u', BitCount::_8),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123u16",
            NumberKind('u', BitCount::_16),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123u32",
            NumberKind('u', BitCount::_32),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123u64",
            NumberKind('u', BitCount::_64),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123u128",
            NumberKind('u', BitCount::_128),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123usize",
            NumberKind('u', BitCount::Size),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
    }

    #[test]
    fn match_signed_integers() {
        assert_number_eq!(
            "123i8",
            NumberKind('i', BitCount::_8),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123i16",
            NumberKind('i', BitCount::_16),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123i32",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123i64",
            NumberKind('i', BitCount::_64),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123i128",
            NumberKind('i', BitCount::_128),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
        assert_number_eq!(
            "123isize",
            NumberKind('i', BitCount::Size),
            NumberValue::Integer {
                value: 123,
                kind: IntegerKind::Decimal,
            }
        );
    }

    #[test]
    fn match_f64() {
        assert_number_eq!(
            "123_100.0",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123_100.0)
        );
        assert_number_eq!(
            "42e42",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(42e42)
        );
        assert_number_eq!(
            "42.42e2",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(42.42e2)
        );
        // TODO: this doesn't work due to a precision error
        // assert_number_eq!(
        //     "42.42e42",
        //     NumberKind('f', BitCount::_64),
        //     NumberValue::Floating(42.42e42)
        // );
        assert_number_eq!(
            "123.23f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23)
        );
        assert_number_eq!(
            "123.23e10f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23e10)
        );
        assert_number_eq!(
            "123.23E10f64",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23E10)
        );
        assert_number_eq!(
            "123.23e10",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123.23e10)
        );
        assert_number_eq!(
            "123E10",
            NumberKind('f', BitCount::_64),
            NumberValue::Floating(123E10)
        );
    }

    #[test]
    fn match_f32() {
        assert_number_eq!(
            "123f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.0)
        );
        assert_number_eq!(
            "123.23f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23)
        );
        assert_number_eq!(
            "123.23e10f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23e10)
        );
        assert_number_eq!(
            "123.23E10f32",
            NumberKind('f', BitCount::_32),
            NumberValue::Floating(123.23E10)
        );
    }

    #[test]
    fn match_hexidecimal() {
        assert_number_eq!(
            "0x_1ab",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0x_1ab,
                kind: IntegerKind::Hexadecimal,
            }
        );
        assert_number_eq!(
            "0X1AB",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0x1AB,
                kind: IntegerKind::Hexadecimal,
            }
        );
        assert_number_eq!(
            "0x0",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0x0,
                kind: IntegerKind::Hexadecimal,
            }
        );
    }

    #[test]
    fn match_octal() {
        assert_number_eq!(
            "0o_7000",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0o_7000,
                kind: IntegerKind::Octal,
            }
        );
        assert_number_eq!(
            "0O7000",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0o7000,
                kind: IntegerKind::Octal,
            }
        );
        assert_number_eq!(
            "0o_123_456",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0o_123_456,
                kind: IntegerKind::Octal,
            }
        );
    }

    #[test]
    fn match_binary() {
        assert_number_eq!(
            "0b___01__010_10__",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0b___01__010_10__,
                kind: IntegerKind::Binary,
            }
        );
        assert_number_eq!(
            "0B0101",
            NumberKind('i', BitCount::_32),
            NumberValue::Integer {
                value: 0b0101,
                kind: IntegerKind::Binary,
            }
        );
    }

    #[test]
    fn fail_exponent() {
        // number with exp can only be floats
        assert!(lex_literal("123e3_i32".into()).is_err());
        // exp must be followed by decimal
        assert!(lex_literal("123e_f32".into()).is_err());
    }

    #[test]
    fn fail_invalid_float_precision() {
        assert!(lex_literal("123e12f8".into()).is_err());
        assert!(lex_literal("123f8".into()).is_err());
        assert!(lex_literal("123.12e-12f16".into()).is_err());
        assert!(lex_literal("123.12f16".into()).is_err());
        assert!(lex_literal("123fsize".into()).is_err());
    }
}
