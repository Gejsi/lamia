use nom::{combinator::map, IResult};

use super::{lex_identifier, LexerError};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Keyword {
    Function,
    Let,
    If,
    Else,
    Return,
}

fn match_keyword(identifier: &str) -> Keyword {
    match identifier {
        "fn" => Keyword::Function,
        "let" => Keyword::Let,
        "if" => Keyword::If,
        "else" => Keyword::Else,
        "return" => Keyword::Return,

        _ => unreachable!(),
    }
}

pub fn lex_keyword(i: &str) -> IResult<&str, Keyword, LexerError> {
    map(lex_identifier, match_keyword)(i)
}

#[cfg(test)]
mod tests {
    use super::{lex_keyword, Keyword};

    #[test]
    fn match_function() {
        assert_eq!(lex_keyword("fn"), Ok(("", Keyword::Function)));
    }

    #[test]
    fn match_let() {
        assert_eq!(lex_keyword("let"), Ok(("", Keyword::Let)));
    }

    #[test]
    fn match_if() {
        assert_eq!(lex_keyword("if"), Ok(("", Keyword::If)));
    }

    #[test]
    fn match_else() {
        assert_eq!(lex_keyword("else"), Ok(("", Keyword::Else)));
    }

    #[test]
    fn match_return() {
        assert_eq!(lex_keyword("return"), Ok(("", Keyword::Return)));
    }
}
