use logos::Logos;

use lexer::{ok_first_token, Punctuation, Token};

#[test]
fn match_comma() {
    ok_first_token!(",", Token::Punctuation(Punctuation::Comma));
}

#[test]
fn match_semicolon() {
    ok_first_token!(";", Token::Punctuation(Punctuation::Semicolon));
}

#[test]
fn match_colon() {
    ok_first_token!(":", Token::Punctuation(Punctuation::Colon));
}

#[test]
fn match_right_arrow() {
    ok_first_token!("->", Token::Punctuation(Punctuation::RightArrow));
}
