use nom::{character::complete::char, combinator::map, multi::many0, sequence::delimited, IResult};

use super::{lexer, LexerError, Span, TokenKind};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Delimiter {
    Paren,
    Square,
    Brace,
}

macro_rules! generate_lex_group {
    ($name: ident, $openc: expr, $closec: expr, $delim: expr) => {
        pub fn $name(i: Span) -> IResult<Span, TokenKind, LexerError> {
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
