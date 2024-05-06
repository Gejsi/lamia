use nom::{
    character::complete::char,
    combinator::{map, value},
    multi::many0,
    sequence::delimited,
    IResult,
};

use super::{lexer, Delimiter, LexerError, Span, TokenKind};

pub fn lex_comma(i: Span) -> IResult<Span, TokenKind, LexerError> {
    value(TokenKind::Comma, char(','))(i)
}

pub fn lex_semicolon(i: Span) -> IResult<Span, TokenKind, LexerError> {
    value(TokenKind::Semicolon, char(';'))(i)
}

pub fn lex_colon(i: Span) -> IResult<Span, TokenKind, LexerError> {
    value(TokenKind::Colon, char(','))(i)
}

macro_rules! generate_lex_group {
    ($name: ident, $openc: expr, $closec: expr, $delim: expr) => {
        fn $name(i: Span) -> IResult<Span, TokenKind, LexerError> {
            map(
                delimited(char($openc), many0(lexer), char($closec)),
                |tokens| TokenKind::Group {
                    delimiter: $delim,
                    tokens: tokens.into_iter().map(|t| t.to_owned()).collect(),
                },
            )(i)
        }
    };
}

generate_lex_group!(lex_group_paren, '(', ')', Delimiter::Paren);
generate_lex_group!(lex_group_square, '[', ']', Delimiter::Square);
generate_lex_group!(lex_group_brace, '{', '}', Delimiter::Brace);


