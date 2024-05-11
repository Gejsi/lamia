use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

use super::{LexerError, Span};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Bang,
    Star,
    Slash,
    Modulo,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
}

pub fn lex_operator(i: Span) -> IResult<Span, Operator, LexerError> {
    alt((
        value(Operator::Equal, tag("==")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::GreaterThanEqual, tag(">=")),
        value(Operator::LessThanEqual, tag("<=")),
        value(Operator::LogicalAnd, tag("&&")),
        value(Operator::LogicalOr, tag("||")),
        value(Operator::Plus, tag("+")),
        value(Operator::Minus, tag("-")),
        value(Operator::Bang, tag("!")),
        value(Operator::Star, tag("*")),
        value(Operator::Slash, tag("/")),
        value(Operator::Modulo, tag("%")),
        value(Operator::LessThan, tag("<")),
        value(Operator::GreaterThan, tag(">")),
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::assert_lex_eq;

    use super::{lex_operator, Operator};

    macro_rules! assert_operator_eq {
        ($text: expr, $op: expr) => {
            assert_lex_eq!(lex_operator($text.into()), $op);
        };
    }

    #[test]
    fn match_plus() {
        assert_operator_eq!("+", Operator::Plus);
    }

    #[test]
    fn match_minus() {
        assert_operator_eq!("-", Operator::Minus);
    }

    #[test]
    fn match_bang() {
        assert_operator_eq!("!", Operator::Bang);
    }

    #[test]
    fn match_star() {
        assert_operator_eq!("*", Operator::Star);
    }

    #[test]
    fn match_slash() {
        assert_operator_eq!("/", Operator::Slash);
    }

    #[test]
    fn match_modulo() {
        assert_operator_eq!("%", Operator::Modulo);
    }

    #[test]
    fn match_equal() {
        assert_operator_eq!("==", Operator::Equal);
    }

    #[test]
    fn match_not_equal() {
        assert_operator_eq!("!=", Operator::NotEqual);
    }

    #[test]
    fn match_less_than() {
        assert_operator_eq!("<", Operator::LessThan);
    }

    #[test]
    fn match_greater_than() {
        assert_operator_eq!(">", Operator::GreaterThan);
    }

    #[test]
    fn match_less_than_equal() {
        assert_operator_eq!("<=", Operator::LessThanEqual);
    }

    #[test]
    fn match_greater_than_equal() {
        assert_operator_eq!(">=", Operator::GreaterThanEqual);
    }

    #[test]
    fn match_logical_and() {
        assert_operator_eq!("&&", Operator::LogicalAnd);
    }

    #[test]
    fn match_logical_or() {
        assert_operator_eq!("||", Operator::LogicalOr);
    }
}
