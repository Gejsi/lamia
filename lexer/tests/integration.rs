use lexer::{ok_all_tokens, ok_no_whitespace, Grouping, Keyword, Number, Operator, Token};

#[test]
fn match_expr() {
    ok_all_tokens!(
        "1 + 2 * 3 / 4",
        [
            Token::Number(Number::Integer("1")),
            Token::Whitespace(" "),
            Token::Operator(Operator::Plus),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("2")),
            Token::Whitespace(" "),
            Token::Operator(Operator::Star),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("3")),
            Token::Whitespace(" "),
            Token::Operator(Operator::Slash),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("4"))
        ]
    );
    ok_all_tokens!(
        "n % 2 == 0",
        [
            Token::Identifier("n"),
            Token::Whitespace(" "),
            Token::Operator(Operator::Modulo),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("2")),
            Token::Whitespace(" "),
            Token::Operator(Operator::Equal),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("0")),
        ]
    );
    ok_all_tokens!(
        "if val { 1 } else { 2 }",
        [
            Token::Keyword(Keyword::If),
            Token::Whitespace(" "),
            Token::Identifier("val"),
            Token::Whitespace(" "),
            Token::Grouping(Grouping::OpenBrace),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("1")),
            Token::Whitespace(" "),
            Token::Grouping(Grouping::CloseBrace),
            Token::Whitespace(" "),
            Token::Keyword(Keyword::Else),
            Token::Whitespace(" "),
            Token::Grouping(Grouping::OpenBrace),
            Token::Whitespace(" "),
            Token::Number(Number::Integer("2")),
            Token::Whitespace(" "),
            Token::Grouping(Grouping::CloseBrace),
        ]
    );

    ok_no_whitespace!(
        "(n + 1) * 2",
        [
            Token::Grouping(Grouping::OpenParen),
            Token::Identifier("n"),
            Token::Operator(Operator::Plus),
            Token::Number(Number::Integer("1")),
            Token::Grouping(Grouping::CloseParen),
            Token::Operator(Operator::Star),
            Token::Number(Number::Integer("2")),
        ]
    );
}
