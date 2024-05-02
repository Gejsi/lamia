use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

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

fn match_operator(identifier: Span) -> Operator {
    match identifier.to_string().as_str() {
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

pub fn lex_operator(i: Span) -> IResult<Span, Operator, LexerError> {
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

    macro_rules! assert_operator_eq {
        ($n: expr, $value: expr) => {
            assert_eq!(
                lex_operator($n.into()),
                Ok((
                    "".into(),
                    $value
                ))
            );
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
        assert_operator_eq!("!",Operator::Bang);
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
