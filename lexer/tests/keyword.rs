use lexer::{ok_first_token, Keyword, Token};

#[test]
fn match_function() {
    ok_first_token!("fn", Token::Keyword(Keyword::Function));
}

#[test]
fn match_let() {
    ok_first_token!("let", Token::Keyword(Keyword::Let));
}

#[test]
fn match_if() {
    ok_first_token!("if", Token::Keyword(Keyword::If));
}

#[test]
fn match_else() {
    ok_first_token!("else", Token::Keyword(Keyword::Else));
}

#[test]
fn match_return() {
    ok_first_token!("return", Token::Keyword(Keyword::Return));
}
