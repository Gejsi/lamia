use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0,
    sequence::pair,
    IResult,
};

use super::LexerError;

pub type Identifier<'a> = &'a str;

pub fn lex_identifier(i: &str) -> IResult<&str, Identifier, LexerError> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(i)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, Err as NErr};

    use super::lex_identifier;

    #[test]
    fn match_simple_identifier() {
        assert_eq!(lex_identifier("varname"), Ok(("", "varname")));
    }

    #[test]
    fn match_underscore_identifier() {
        assert_eq!(lex_identifier("var_name"), Ok(("", "var_name")));
    }

    #[test]
    fn match_number_identifier() {
        assert_eq!(lex_identifier("var_name1"), Ok(("", "var_name1")));
    }

    #[test]
    fn match_indentifier_starting_underscore() {
        assert_eq!(lex_identifier("_var_name"), Ok(("", "_var_name")));
    }

    #[test]
    fn not_match_identifier_number() {
        assert_eq!(
            lex_identifier("1var_name"),
            Err(NErr::Error(("1var_name", ErrorKind::Tag)))
        );
    }
}
