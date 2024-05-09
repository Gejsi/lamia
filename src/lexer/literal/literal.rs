use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_opt, map_res, value, verify},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

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

fn escape_char(i: Span) -> IResult<Span, char, LexerError> {
    alt((
        escape_unicode,
        value('\n', char('n')),
        value('\r', char('r')),
        value('\t', char('t')),
        value('\\', char('\\')),
        value('/', char('/')),
        value('"', char('"')),
        value('\'', char('\'')),
    ))(i)
}

macro_rules! parse_char {
    ($c: expr) => {
        alt((
            preceded(char('\\'), escape_char),
            verify(anychar, |any| any != &$c),
        ))
    };
}

fn lex_character(i: Span) -> IResult<Span, char, LexerError> {
    delimited(char('\''), parse_char!('\''), char('\''))(i)
}

fn lex_string(i: Span) -> IResult<Span, String, LexerError> {
    map(
        delimited(char('\"'), many0(parse_char!('"')), char('\"')),
        |chars| chars.into_iter().collect::<String>(),
    )(i)
}

pub fn lex_literal(i: Span) -> IResult<Span, Literal, LexerError> {
    alt((
        map(lex_boolean, Literal::Boolean),
        map(lex_number, Literal::Number),
        map(lex_character, Literal::Character),
        map(lex_string, Literal::String),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::assert_lex_eq;
    use crate::lexer::{lex_literal, Literal};

    macro_rules! assert_literal_eq {
        ($text: expr, $lit: expr) => {
            assert_lex_eq!(lex_literal($text.into()), $lit);
        };
    }

    #[test]
    fn match_bool() {
        assert_literal_eq!("true", Literal::Boolean(true));
        assert_literal_eq!("false", Literal::Boolean(false));
    }

    #[test]
    fn match_simple_character() {
        assert_literal_eq!("'a'", Literal::Character('a'));
    }

    #[test]
    fn match_escaped_character() {
        assert_literal_eq!(r#"'\n'"#, Literal::Character('\n'));
        assert_literal_eq!(r#"'\r'"#, Literal::Character('\r'));
        assert_literal_eq!(r#"'\t'"#, Literal::Character('\t'));
        assert_literal_eq!(r#"'\\'"#, Literal::Character('\\'));
        assert_literal_eq!(r#"'/'"#, Literal::Character('/'));
        assert_literal_eq!(r#"'"'"#, Literal::Character('"'));
        assert_literal_eq!(r#"' '"#, Literal::Character(' '));
        assert_literal_eq!(r#"'\''"#, Literal::Character('\''));
    }

    #[test]
    fn match_unicode_character() {
        assert_literal_eq!("'\\u{1F600}'", Literal::Character('😀'));
        assert_literal_eq!("'😀'", Literal::Character('😀'));
        assert_literal_eq!("'東'", Literal::Character('東'));
        assert_literal_eq!("'д'", Literal::Character('д'));
        assert_literal_eq!("'ل'", Literal::Character('ل'));
    }

    #[test]
    fn fail_too_many_characters() {
        assert!(lex_literal("'ab'".into()).is_err());
        assert!(lex_literal("'  '".into()).is_err());
        assert!(lex_literal("'東京'".into()).is_err());
        assert!(lex_literal("'''".into()).is_err());
    }

    #[test]
    fn match_simple_string() {
        assert_literal_eq!(r#""test""#, Literal::String("test".into()));
    }

    #[test]
    fn match_escaped_string() {
        assert_literal_eq!("\"test\\\"\"", Literal::String("test\"".into()));
        assert_literal_eq!("\"12\\\"34\"", Literal::String("12\"34".into()));
        assert_literal_eq!("\"hello\\nworld\"", Literal::String("hello\nworld".into()));
    }

    #[test]
    fn match_unicode_string() {
        assert_literal_eq!(r#""東京""#, Literal::String("東京".into()));
        assert_literal_eq!(r#""こんにちは""#, Literal::String("こんにちは".into()));
        assert_literal_eq!(r#""erfüllen""#, Literal::String("erfüllen".into()));
        assert_literal_eq!(r#""Здравствуйте""#, Literal::String("Здравствуйте".into()));
        assert_literal_eq!(
            r#""\u{48}\u{65}\u{6C}\u{6C}\u{6F}""#,
            Literal::String("Hello".into())
        );
        assert_literal_eq!(
            r#""hello\u{1F600}world""#,
            Literal::String("hello😀world".into())
        );
        assert_literal_eq!(r#""hello😀world""#, Literal::String("hello😀world".into()));
    }

    #[test]
    fn match_whitespace_string() {
        assert_literal_eq!("\"     \"", Literal::String("     ".into()));
        assert_literal_eq!(
            "\" This is a test \"",
            Literal::String(" This is a test ".into())
        );
        assert_literal_eq!("\"test \"", Literal::String("test ".into()));
        assert_literal_eq!("\" test\"", Literal::String(" test".into()));
    }
}
