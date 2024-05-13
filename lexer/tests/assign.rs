use lexer::{ok_first_token, Assign, Token};

#[test]
fn match_plus() {
    ok_first_token!("+=", Token::Assign(Assign::PlusEqual));
}

#[test]
fn match_minus() {
    ok_first_token!("-=", Token::Assign(Assign::MinusEqual));
}

#[test]
fn match_star() {
    ok_first_token!("*=", Token::Assign(Assign::StarEqual));
}

#[test]
fn match_slash() {
    ok_first_token!("/=", Token::Assign(Assign::SlashEqual));
}

#[test]
fn match_modulo() {
    ok_first_token!("%=", Token::Assign(Assign::ModuloEqual));
}

#[test]
fn match_equal() {
    ok_first_token!("=", Token::Assign(Assign::Equal));
}
