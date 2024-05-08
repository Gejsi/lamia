use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_opt, map_res, value},
    sequence::{delimited, preceded},
    IResult,
};
use nom_unicode::complete::alphanumeric1;

use crate::lexer::{LexerError, Span};

use super::number::{lex_number, Number};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Literal {
    String(String),
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

fn lex_string(i: Span) -> IResult<Span, String, LexerError> {
    delimited(
        char('\"'),
        escaped_transform(
            alphanumeric1,
            '\\',
            alt((
                escape_unicode,
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value('\x07', char('a')),
                value('\x08', char('b')),
                value('\x0B', char('v')),
                value('\x0C', char('f')),
                value('\\', char('\\')),
                value('/', char('/')),
                value('"', char('"')),
            )),
        ),
        char('\"'),
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
        assert_literal_eq("\"test\"", Literal::String("test".into()));
    }

    #[test]
    fn match_escaped_string() {
        assert_literal_eq("\"test\\\"\"", Literal::String("test\"".into()));
        assert_literal_eq("\"12\\\"34\"", Literal::String("12\"34".into()));
        assert_literal_eq("\"hello\\nworld\"", Literal::String("hello\nworld".into()));
    }

    #[test]
    fn match_unicode_string() {
        assert_literal_eq(r#""東京""#, Literal::String("東京".into()));
        assert_literal_eq(r#""こんにちは""#, Literal::String("こんにちは".into()));
        assert_literal_eq(r#""erfüllen""#, Literal::String("erfüllen".into()));
        assert_literal_eq(r#""Здравствуйте""#, Literal::String("Здравствуйте".into()));
        // println!("😄\u{09}\u{1F604}");
        assert_literal_eq(
            r#""\u{48}\u{65}\u{6C}\u{6C}\u{6F}""#,
            Literal::String("Hello".into()),
        );
        assert_literal_eq(
            r#""hello\u{1F600}world""#,
            Literal::String("hello😀world".into()),
        );
    }

    #[test]
    fn match_whitespace_string() {
        // todo!()
    }
}
