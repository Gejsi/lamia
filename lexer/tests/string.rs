use lexer::{ok_first_token, Token};

#[test]
fn match_simple_string() {
    ok_first_token!(r#""test""#, Token::String(r#""test""#));
}

#[test]
fn match_escaped_string() {
    ok_first_token!(r#""test\"""#, Token::String(r#""test\"""#));
    ok_first_token!(r#""12\"34""#, Token::String(r#""12\"34""#));
    ok_first_token!(r#""hello\nworld""#, Token::String(r#""hello\nworld""#));
}

#[test]
fn match_unicode_string() {
    ok_first_token!(r#""東京""#, Token::String(r#""東京""#));
    ok_first_token!(r#""こんにちは""#, Token::String(r#""こんにちは""#));
    ok_first_token!(r#""erfüllen""#, Token::String(r#""erfüllen""#));
    ok_first_token!(r#""Здравствуйте""#, Token::String(r#""Здравствуйте""#));
    ok_first_token!(
        r#""\u{48}\u{65}\u{6C}\u{6C}\u{6F}""#,
        Token::String(r#""\u{48}\u{65}\u{6C}\u{6C}\u{6F}""#)
    );
    ok_first_token!(
        r#""hello\u{1F600}world""#,
        Token::String(r#""hello\u{1F600}world""#)
    );
    ok_first_token!(r#""hello😀world""#, Token::String(r#""hello😀world""#));
}

#[test]
fn match_whitespace_string() {
    ok_first_token!(r#""     ""#, Token::String(r#""     ""#));
    ok_first_token!(
        r#"" This is a test ""#,
        Token::String(r#"" This is a test ""#)
    );
    ok_first_token!(r#""test ""#, Token::String(r#""test ""#));
    ok_first_token!(r#"" test""#, Token::String(r#"" test""#));
}
