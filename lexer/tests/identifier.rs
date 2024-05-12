use lexer::{ok_first_token, Token};

#[test]
fn match_simple() {
    ok_first_token!("varname", Token::Identifier("varname"));
}

#[test]
fn match_underscore() {
    ok_first_token!("var_name", Token::Identifier("var_name"));
    ok_first_token!("varname_", Token::Identifier("varname_"));
    ok_first_token!("_var_name", Token::Identifier("_var_name"));
    ok_first_token!("_", Token::Identifier("_"));
}

#[test]
fn match_number() {
    ok_first_token!("var1name", Token::Identifier("var1name"));
    ok_first_token!("var_name1", Token::Identifier("var_name1"));
}
