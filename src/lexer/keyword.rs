use nom::{combinator::map, IResult};

use super::{lex_identifier, LexerError, Span};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

pub fn lex_keyword(i: Span) -> IResult<Span, Keyword, LexerError> {
    map(lex_identifier, match_keyword)(i)
}

#[cfg(test)]
mod tests {
    use nom_locate::LocatedSpan;

    use super::{lex_keyword, Keyword};

    #[test]
    fn match_function() {
        assert_eq!(lex_keyword("fn".into()), Ok(("".into(), Keyword::Function)));
    }

    #[test]
    fn match_let() {
        assert_eq!(lex_keyword("let".into()), Ok(("".into(), Keyword::Let)));
    }

    #[test]
    fn match_if() {
        assert_eq!(lex_keyword("if".into()), Ok(("", Keyword::If)));
    }

    #[test]
    fn match_else() {
        assert_eq!(lex_keyword("else".into()), Ok(("".into(), Keyword::Else)));
    }

    #[test]
    fn match_return() {
        assert_eq!(lex_keyword("return".into()), Ok(("".into(), Keyword::Return)));
    }
}
