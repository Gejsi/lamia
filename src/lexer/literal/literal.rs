use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while_m_n},
    character::complete::{anychar, char, one_of},
    combinator::{map, map_opt, map_res, value},
    sequence::{delimited, preceded},
    IResult,
};
use nom_unicode::complete::alphanumeric1;

use crate::lexer::{LexerError, Span};

use super::number::{lex_number, Number};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Literal<'a> {
    String(&'a str),
    Character(char),
    Boolean(bool),
    Number(Number),
}

fn lex_boolean(i: Span) -> IResult<Span, bool, LexerError> {
    alt((value(true, tag("true")), value(false, tag("false"))))(i)
}

fn lex_character(i: Span) -> IResult<Span, char, LexerError> {
    delimited(char('\''), anychar, char('\''))(i)
}

fn escape_unicode(i: Span) -> IResult<Span, char, LexerError> {
    let parse_hex = preceded(
        char('u'),
        delimited(
            char('{'),
            take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
            char('}'),
        ),
    );

    let parse_u32 = map_res(parse_hex, move |hex: Span| {
        u32::from_str_radix(hex.into_fragment(), 16)
    });

    map_opt(parse_u32, std::char::from_u32)(i)
}

fn lex_string(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        delimited(
            char('\"'),
            escaped(
                alphanumeric1,
                '\\',
                alt((escape_unicode, one_of(r#""'nrt0\"#))),
            ),
            char('\"'),
        ),
        |s: Span| s.into_fragment(),
    )(i)
}

pub fn lex_literal(i: Span) -> IResult<Span, Literal, LexerError> {
    alt((
        map(lex_boolean, Literal::Boolean),
        map(lex_character, Literal::Character),
        map(lex_string, Literal::String),
        map(lex_number, Literal::Number),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::lexer::{assert_lex_eq, lex_literal, Literal};

    fn assert_literal_eq(text: &str, lit: Literal) {
        assert_lex_eq!(lex_literal(text.into()), lit);
    }

    #[test]
    fn match_bool() {
        assert_literal_eq("true", Literal::Boolean(true));
        assert_literal_eq("false", Literal::Boolean(false));
    }

    #[test]
    fn match_character() {
        assert_literal_eq(r#"'a'"#, Literal::Character('a'));
        assert!(lex_literal(r#"'ab'"#.into()).is_err());
        // assert_literal_eq(r#"'\''"#, Literal::Character('\''));
        // assert_literal_eq(r#"'\n'"#, Literal::Character('\n'));
        // assert_literal_eq(r#"'\u{1F604}'"#, Literal::Character('😄'));
    }

    #[test]
    fn match_simple_string() {
        assert_literal_eq("\"test\"", Literal::String("test"));
    }

    #[test]
    fn match_escaped_string() {
        assert_literal_eq("\"test\\\"\"", Literal::String("test\\\""));
    }

    #[test]
    fn match_newline_string() {
        assert_literal_eq("\"test\\n\"", Literal::String("test\\n"));
    }

    #[test]
    fn match_unicode_string() {
        assert_literal_eq("\"東京\"", Literal::String("東京"));
        assert_literal_eq("\"こんにちは\"", Literal::String("こんにちは"));
        assert_literal_eq("\"erfüllen\"", Literal::String("erfüllen"));
        assert_literal_eq("\"Здравствуйте\"", Literal::String("Здравствуйте"));
        assert_literal_eq(r#""Москва\u{1F605}""#, Literal::String("Москва\\u{1F605}"));
    }

    #[test]
    fn match_whitespace_string() {
        todo!()
    }
}
