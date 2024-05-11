use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value, IResult,
};

use super::{LexerError, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Comma,
    Semicolon,
    Colon,
    RightArrow,
}

pub fn lex_punctuation(i: Span) -> IResult<Span, Punctuation, LexerError> {
    alt((
        value(Punctuation::Comma, char(',')),
        value(Punctuation::Semicolon, char(';')),
        value(Punctuation::Colon, char(':')),
        value(Punctuation::RightArrow, tag("->")),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::assert_lex_eq;

    use super::{lex_punctuation, Punctuation};

    #[test]
    fn match_comma() {
        assert_lex_eq!(lex_punctuation(",".into()), Punctuation::Comma);
    }

    #[test]
    fn match_semicolon() {
        assert_lex_eq!(lex_punctuation(";".into()), Punctuation::Semicolon);
    }

    #[test]
    fn match_colon() {
        assert_lex_eq!(lex_punctuation(":".into()), Punctuation::Colon);
    }

    #[test]
    fn match_right_arrow() {
        assert_lex_eq!(lex_punctuation("->".into()), Punctuation::RightArrow);
    }
}
