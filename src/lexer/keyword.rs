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
    use crate::lexer::assert_lex_eq;

    use super::{lex_keyword, Keyword};

    fn assert_keyword_eq(text: &str, keyword: Keyword) {
        assert_lex_eq!(lex_keyword(text.into()), keyword);
    }

    #[test]
    fn match_function() {
        assert_keyword_eq("fn", Keyword::Function);
    }

    #[test]
    fn match_let() {
        assert_keyword_eq("let", Keyword::Let);
    }

    #[test]
    fn match_if() {
        assert_keyword_eq("if", Keyword::If);
    }

    #[test]
    fn match_else() {
        assert_keyword_eq("else", Keyword::Else);
    }

    #[test]
    fn match_return() {
        assert_keyword_eq("return", Keyword::Return);
    }
}
