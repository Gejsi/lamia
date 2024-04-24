use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use super::LexerError;

#[derive(Debug, PartialEq, PartialOrd)]
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

fn match_operator(identifier: &str) -> Operator {
    match identifier {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "!" => Operator::Bang,
        "*" => Operator::Star,
        "/" => Operator::Slash,
        "%" => Operator::Modulo,
        "==" => Operator::Equal,
        "!=" => Operator::NotEqual,
        "<" => Operator::LessThan,
        ">" => Operator::GreaterThan,
        "<=" => Operator::LessThanEqual,
        ">=" => Operator::GreaterThanEqual,
        "&&" => Operator::LogicalAnd,
        "||" => Operator::LogicalOr,
        _ => unreachable!(),
    }
}

pub fn lex_operator(i: &str) -> IResult<&str, Operator, LexerError> {
    map(
        alt((
            tag("!="),
            tag(">="),
            tag("<="),
            tag("=="),
            tag("&&"),
            tag("||"),
            tag("+"),
            tag("-"),
            tag("!"),
            tag("*"),
            tag("/"),
            tag("%"),
            tag("<"),
            tag(">"),
        )),
        match_operator,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{lex_operator, Operator};

    #[test]
    fn match_plus() {
        assert_eq!(lex_operator("+"), Ok(("", Operator::Plus)));
    }

    #[test]
    fn match_minus() {
        assert_eq!(lex_operator("-"), Ok(("", Operator::Minus)));
    }

    #[test]
    fn match_bang() {
        assert_eq!(lex_operator("!"), Ok(("", Operator::Bang)));
    }

    #[test]
    fn match_star() {
        assert_eq!(lex_operator("*"), Ok(("", Operator::Star)));
    }

    #[test]
    fn match_slash() {
        assert_eq!(lex_operator("/"), Ok(("", Operator::Slash)));
    }

    #[test]
    fn match_modulo() {
        assert_eq!(lex_operator("%"), Ok(("", Operator::Modulo)));
    }

    #[test]
    fn match_equal() {
        assert_eq!(lex_operator("=="), Ok(("", Operator::Equal)));
    }

    #[test]
    fn match_not_equal() {
        assert_eq!(lex_operator("!="), Ok(("", Operator::NotEqual)));
    }

    #[test]
    fn match_less_than() {
        assert_eq!(lex_operator("<"), Ok(("", Operator::LessThan)));
    }

    #[test]
    fn match_greater_than() {
        assert_eq!(lex_operator(">"), Ok(("", Operator::GreaterThan)));
    }

    #[test]
    fn match_less_than_equal() {
        assert_eq!(lex_operator("<="), Ok(("", Operator::LessThanEqual)));
    }

    #[test]
    fn match_greater_than_equal() {
        assert_eq!(lex_operator(">="), Ok(("", Operator::GreaterThanEqual)));
    }

    #[test]
    fn match_logical_and() {
        assert_eq!(lex_operator("&&"), Ok(("", Operator::LogicalAnd)));
    }

    #[test]
    fn match_logical_or() {
        assert_eq!(lex_operator("||"), Ok(("", Operator::LogicalOr)));
    }
}
