use lexer::{ok_first_token, Number, Token};

#[test]
fn match_underscore() {
    ok_first_token!("1_000_000", Token::Number(Number::Integer("1_000_000")));
    ok_first_token!(
        "1_000_000.000_001",
        Token::Number(Number::Float("1_000_000.000_001"))
    );
}

#[test]
fn match_unsigned_integers() {
    ok_first_token!("123u8", Token::Number(Number::Integer("123u8")));
    ok_first_token!("123u16", Token::Number(Number::Integer("123u16")));
    ok_first_token!("123u16", Token::Number(Number::Integer("123u16")));
    ok_first_token!("123u32", Token::Number(Number::Integer("123u32")));
    ok_first_token!("123u64", Token::Number(Number::Integer("123u64")));
    ok_first_token!("123u128", Token::Number(Number::Integer("123u128")));
    ok_first_token!("123usize", Token::Number(Number::Integer("123usize")));
}

#[test]
fn match_signed_integers() {
    ok_first_token!("123i8", Token::Number(Number::Integer("123i8")));
    ok_first_token!("123i16", Token::Number(Number::Integer("123i16")));
    ok_first_token!("123i16", Token::Number(Number::Integer("123i16")));
    ok_first_token!("123i32", Token::Number(Number::Integer("123i32")));
    ok_first_token!("123i64", Token::Number(Number::Integer("123i64")));
    ok_first_token!("123i128", Token::Number(Number::Integer("123i128")));
    ok_first_token!("123isize", Token::Number(Number::Integer("123isize")));
}

#[test]
fn match_f64() {
    ok_first_token!("123_100.0", Token::Number(Number::Float("123_100.0")));
    ok_first_token!("42e42", Token::Number(Number::Float("42e42")));
    ok_first_token!("42.42e2", Token::Number(Number::Float("42.42e2")));
    ok_first_token!("123.23f64", Token::Number(Number::Float("123.23f64")));
    ok_first_token!("123.23e10f64", Token::Number(Number::Float("123.23e10f64")));
    ok_first_token!("123.23E10f64", Token::Number(Number::Float("123.23E10f64")));
    ok_first_token!("123.23e10", Token::Number(Number::Float("123.23e10")));
    ok_first_token!("123E10", Token::Number(Number::Float("123E10")));
}

#[test]
fn match_f32() {
    ok_first_token!("123f32", Token::Number(Number::Float("123f32")));
    ok_first_token!("123.23f32", Token::Number(Number::Float("123.23f32")));
    ok_first_token!("123.23e10f32", Token::Number(Number::Float("123.23e10f32")));
    ok_first_token!("123.23E10f32", Token::Number(Number::Float("123.23E10f32")));
}

#[test]
fn match_hex_integer() {
    ok_first_token!("0x1_ab", Token::Number(Number::HexInteger("0x1_ab")));
    ok_first_token!("0X1AB", Token::Number(Number::HexInteger("0X1AB")));
    ok_first_token!("0x0", Token::Number(Number::HexInteger("0x0")));
}

#[test]
fn match_oct_integer() {
    ok_first_token!("0o7000", Token::Number(Number::OctalInteger("0o7000")));
    ok_first_token!("0O7000", Token::Number(Number::OctalInteger("0O7000")));
    ok_first_token!(
        "0o123_456",
        Token::Number(Number::OctalInteger("0o123_456"))
    );
}

#[test]
fn match_bin_integer() {
    ok_first_token!(
        "0b01__010_10__",
        Token::Number(Number::BinaryInteger("0b01__010_10__"))
    );
    ok_first_token!("0B0101", Token::Number(Number::BinaryInteger("0B0101")));
}

#[test]
fn match_hex_float() {
    ok_first_token!("0x0.3p10", Token::Number(Number::HexFloat("0x0.3p10")));
    ok_first_token!("0X0.3p10", Token::Number(Number::HexFloat("0X0.3p10")));
    ok_first_token!(
        "0x0.3p-10f32",
        Token::Number(Number::HexFloat("0x0.3p-10f32"))
    );
}
