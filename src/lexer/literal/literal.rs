use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{anychar, char, one_of},
    combinator::{map, value},
    sequence::delimited,
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

fn lex_string(i: Span) -> IResult<Span, &str, LexerError> {
    map(
        delimited(
            char('\"'),
            escaped(alphanumeric1, '\\', one_of(r#""'nrt0\"#)),
            char('\"')
        ),
        |s: Span| s.into_fragment()
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
    use crate::lexer::{lex_literal, Literal};

    macro_rules! assert_literal_expr {
        ($n: expr, $lit: expr) => {
            assert_eq!(lex_literal($n.into()), Ok(("".into(), $lit)));
        };
    }

    #[test]
    fn match_bool() {
        assert_literal_expr!("true", Literal::Boolean(true));
        assert_literal_expr!("false", Literal::Boolean(false));
    }

    #[test]
    fn match_character() {
        assert_literal_expr!("'a'", Literal::Character('a'));
        assert_literal_expr!("'\\''", Literal::Character('\''));
        assert_literal_expr!("'\\n'", Literal::Character('\n'));
        assert_literal_expr!("''", Literal::Character(' '));
    }

    #[test]
    fn match_simple_string() {
        assert_literal_expr!("\"test\"", Literal::String("test".into()));
    }

    #[test]
    fn match_escaped_string() {
        assert_literal_expr!("\"test\\\"\"", Literal::String("test\\\"".into()));
    }

    #[test]
    fn match_newline_string() {
        assert_literal_expr!("\"test\\n\"", Literal::String("test\\n".into()));
    }

    #[test]
    fn match_unicode_string() {
        assert_literal_expr!("\"東京\"", Literal::String("東京".into()));
        assert_literal_expr!("\"こんにちは\"", Literal::String("こんにちは".into()));
        assert_literal_expr!("\"erfüllen\"", Literal::String("erfüllen".into()));
        assert_literal_expr!("\"Здравствуйте\"", Literal::String("Здравствуйте".into()));
        assert_literal_expr!("\"Москва\"", Literal::String("Москва".into()));
    }
}
