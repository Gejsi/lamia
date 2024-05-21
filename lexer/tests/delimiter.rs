use lexer::{ok_first_token, Delimiter, Token};

#[test]
fn match_comma() {
    ok_first_token!(",", Token::Delimiter(Delimiter::Comma));
}

#[test]
fn match_semicolon() {
    ok_first_token!(";", Token::Delimiter(Delimiter::Semicolon));
}

#[test]
fn match_colon() {
    ok_first_token!(":", Token::Delimiter(Delimiter::Colon));
}

#[test]
fn match_right_arrow() {
    ok_first_token!("->", Token::Delimiter(Delimiter::RightArrow));
}
