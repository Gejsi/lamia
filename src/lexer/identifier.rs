use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{recognize, map},
    multi::many0,
    sequence::pair,
    IResult,
};

use super::{LexerError, Span};

// TODO: bring back to a &'a str
pub type Identifier<'a> = &'a str;

pub fn lex_identifier(i: Span) -> IResult<Span, Identifier, LexerError> {
    map(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )), |s: Span| s.into_fragment())(i)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, Err as NErr};

    use super::lex_identifier;

    #[test]
    fn match_simple_identifier() {
        assert_eq!(lex_identifier("varname".into()), Ok(("".into(), "varname".into())));
    }

    #[test]
    fn match_underscore_identifier() {
        assert_eq!(lex_identifier("var_name".into()), Ok(("".into(), "var_name".into())));
    }

    #[test]
    fn match_number_identifier() {
        assert_eq!(lex_identifier("var_name1".into()), Ok(("".into(), "var_name1".into())));
    }

    #[test]
    fn match_indentifier_starting_underscore() {
        assert_eq!(lex_identifier("_var_name".into()), Ok(("".into(), "_var_name".into())));
    }

    #[test]
    fn not_match_identifier_number() {
        assert_eq!(
            lex_identifier("1var_name".into()),
            Err(NErr::Error(("1var_name".into(), ErrorKind::Tag)))
        );
    }
}
