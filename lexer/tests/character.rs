use logos::Logos;

use lexer::{ok_first_token, Token};

#[test]
fn match_simple_character() {
    ok_first_token!("'a'", Token::Character("'a'"));
}

#[test]
fn match_escaped_character() {
    ok_first_token!(r#"'\n'"#, Token::Character("'\\n'"));
    ok_first_token!(r#"'\r'"#, Token::Character("'\\r'"));
    ok_first_token!(r#"'\t'"#, Token::Character("'\\t'"));
    ok_first_token!(r#"'\\'"#, Token::Character("'\\\\'"));
    ok_first_token!(r#"'/'"#, Token::Character("'/'"));
    ok_first_token!(r#"'"'"#, Token::Character("'\"'"));
    ok_first_token!(r#"' '"#, Token::Character("' '"));
    ok_first_token!(r#"'\''"#, Token::Character("'\\''"));
    // ok_first_token!("'\\u{1F600}'", Token::Character("'\\u{1F600}'"));
}

#[test]
fn match_unicode_character() {
    ok_first_token!("'\u{1F600}'", Token::Character("'\u{1F600}'"));
    ok_first_token!("'😀'", Token::Character("'😀'"));
    ok_first_token!("'東'", Token::Character("'東'"));
    ok_first_token!("'д'", Token::Character("'д'"));
    ok_first_token!("'ل'", Token::Character("'ل'"));
}

// #[test]
// fn fail_too_many_characters() {
//     err_first_token!("'ab'", ());
//     err_first_token!("'  '", ());
//     err_first_token!("'東京'", ());
//     err_first_token!("'''", ());
// }
