use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use super::LexerError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    String(&'a str),
    Bool(bool),
    Number(Number),
}

// TODO: questo è veramente temporaneo, non so neanche come vogliamo chiamare i nostri tipi
// (vogliamo la nomenclatura alla java?).
// Vanno ancora distinti gli interi decimali da binari e compagnia.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Number {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
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

fn lex_integer(i: &str) -> IResult<&str, i64, LexerError> {
    map_res(
        recognize(pair(digit1, many0(alt((digit1, tag("_")))))),
        |s: &str| s.replace("_", "").parse::<i64>(),
    )(i)
}

// TODO: togliere gli underscore, ho provato ad usare `map` per togliere gli underscore ma `replace` ritorna `String` e a
// noi serve `&str`.
fn decimal(i: &str) -> IResult<&str, &str, LexerError> {
    recognize(pair(digit1, many0(alt((digit1, tag("_"))))))(i)
}

fn lex_number(i: &str) -> IResult<&str, Number, LexerError> {
    alt((
        // TODO: handle hexadecimal floats (vogliamo questa cosa?)
        // NOTE: this section handle floats
        // Case: .42
        // recognize(tuple((
        //     char('.'),
        //     decimal,
        //     opt(tuple((one_of("eE"), opt(one_of("+-")), cut(decimal)))),
        // ))),
        // Case: 42e42 and 42.42e42
        // recognize(tuple((
        //     decimal,
        //     opt(preceded(char('.'), decimal)),
        //     one_of("eE"),
        //     opt(one_of("+-")),
        //     cut(decimal),
        // ))),
        // Case: 42.42
        // recognize(separated_pair(decimal, char('.'), decimal)),
        // Case: 42.
        // recognize(tuple((decimal, char('.')))),

        // TODO: handle binary/octal/hexadecimal integers
        // NOTE: this section handle integers
        map_res(
            // Ho provato ad usare `cut` qui per short-circuitare il parsing quando si sbaglia a
            // scrivere i suffissi ma non andava come volevo, es. `10i3` dovrebbe fallire ma ora
            // lascia semplicemente `i3` come resto dell'input e fa solo il parsing di `10`.
            // Poi qui servono anche tutti gli altri suffissi...
            tuple((decimal, opt(alt((tag("i32"), tag("i64")))))),
            |(s, suffix)| {
                // non mi piace sto codice, gli `unwrap_or` :(
                Ok::<Number, LexerError>(match suffix {
                    Some(suffix) => {
                        if suffix == "i64" {
                            Number::Long(s.parse::<i64>().unwrap_or(0))
                        } else {
                            Number::Integer(s.parse::<i32>().unwrap_or(0))
                        }
                    }
                    None => Number::Integer(s.parse::<i32>().unwrap_or(0)),
                })
            },
        ),
    ))(i)
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
    use crate::lexer::{lex_literal, literal::Number, Literal};

    #[test]
    fn match_bool() {
        assert_eq!(lex_literal("true"), Ok(("", Literal::Bool(true))));
        assert_eq!(lex_literal("false"), Ok(("", Literal::Bool(false))));
    }

    #[test]
    fn match_long() {
        assert_eq!(
            lex_literal("123i64"),
            Ok(("", Literal::Number(Number::Long(123))))
        );
    }

    // #[test]
    // fn temp_test() {
    // assert_eq!(
    //     lex_literal(".456_e-12"),
    //     Ok(("", Literal::Number(Number::Long(123))))
    // );
    // assert_eq!(
    //     lex_literal("21.456e+12"),
    //     Ok(("", Literal::Number(Number::Long(123))))
    // );
    // assert_eq!(
    //     lex_literal("21.100_10"),
    //     Ok(("", Literal::Number(Number::Long(123))))
    // );
    // assert_eq!(
    //     lex_literal("21."),
    //     Ok(("", Literal::Number(Number::Long(123))))
    // );
    // assert_eq!(
    //     lex_literal("21"),
    //     Ok(("", Literal::Number(Number::Long(123))))
    // );
    // }

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
