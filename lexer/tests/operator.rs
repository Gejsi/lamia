use lexer::{ok_first_token, Operator, Token};

#[test]
fn match_plus() {
    ok_first_token!("+", Token::Operator(Operator::Plus));
}

#[test]
fn match_minus() {
    ok_first_token!("-", Token::Operator(Operator::Minus));
}

#[test]
fn match_bang() {
    ok_first_token!("!", Token::Operator(Operator::Bang));
}

#[test]
fn match_star() {
    ok_first_token!("*", Token::Operator(Operator::Star));
}

#[test]
fn match_slash() {
    ok_first_token!("/", Token::Operator(Operator::Slash));
}

#[test]
fn match_modulo() {
    ok_first_token!("%", Token::Operator(Operator::Modulo));
}

#[test]
fn match_equal() {
    ok_first_token!("==", Token::Operator(Operator::Equal));
}

#[test]
fn match_not_equal() {
    ok_first_token!("!=", Token::Operator(Operator::NotEqual));
}

#[test]
fn match_less_than() {
    ok_first_token!("<", Token::Operator(Operator::LessThan));
}

#[test]
fn match_greater_than() {
    ok_first_token!(">", Token::Operator(Operator::GreaterThan));
}

#[test]
fn match_less_than_equal() {
    ok_first_token!("<=", Token::Operator(Operator::LessThanEqual));
}

#[test]
fn match_greater_than_equal() {
    ok_first_token!(">=", Token::Operator(Operator::GreaterThanEqual));
}

#[test]
fn match_logical_and() {
    ok_first_token!("&&", Token::Operator(Operator::LogicalAnd));
}

#[test]
fn match_logical_or() {
    ok_first_token!("||", Token::Operator(Operator::LogicalOr));
}
