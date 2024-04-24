use std::str::Chars;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of},
    combinator::{all_consuming, cut, map, map_res, recognize, value},
    error::{FromExternalError, ParseError},
    multi::many0,
    error::ErrorKind,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};


use thiserror::Error;

use super::token::{Token, TokenKind, TokenSource, Keyword, Operator};

type LexerError<'a> = (&'a str, ErrorKind);

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

fn identifier(i: &str) -> IResult<&str, &str, LexerError> {
    recognize(pair(
        alt((alpha1::<&str, LexerError>, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(i)
}

pub fn match_keyword(identifier: &str) -> TokenKind {
    match identifier {
        "fn" => TokenKind::Keyword(Keyword::Function),
        "let" => TokenKind::Keyword(Keyword::Let),
        "true" => TokenKind::Keyword(Keyword::True),
        "false" => TokenKind::Keyword(Keyword::False),
        "if" => TokenKind::Keyword(Keyword::If),
        "else" => TokenKind::Keyword(Keyword::Else),
        "return" => TokenKind::Keyword(Keyword::Return),
        i => TokenKind::Identifier(i),
    }
}

fn keyword(i: &str) -> IResult<&str, TokenKind, LexerError> {
    map(identifier, match_keyword)(i)
}

pub fn match_operator(identifier: &str) -> Operator {
    match identifier {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "!" => Operator::Bang,
        "*" => Operator::Star,
        "/" => Operator::Slash,
        "%" => Operator::Modulo,
        "==" => Operator::Equal,
        "!=" => Operator::NotEqual,
        "<" => Operator::LessThan,
        ">" => Operator::GreaterThan,
        "<=" => Operator::LessThanEqual,
        ">=" => Operator::GreaterThanEqual,
        "&&" => Operator::AndAnd,
        "||" => Operator::OrOr,
        _ => unreachable!()
    }
}

fn operator(i: &str) -> IResult<&str, Operator, LexerError> {
    map(alt((
        tag("+"),
        tag("-"),
        tag("!"),
        tag("*"),
        tag("/"),
        tag("%"),
        tag("=="),
        tag("!="),
        tag("<"),
        tag(">"),
        tag("<="),
        tag(">="),
        tag("&&"),
        tag("||")
    )), match_operator)(i)
}

macro_rules! group_gen {
    ($func_name: ident, $open: expr, $close: expr) => {
        fn $func_name<'a, E>(i: &'a str) -> IResult<&'a str, token::Token, E>
        where
            E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
        {
            map(
                delimited(char($open), sublex, char($close)),
                |ts: token::TokenStream| {
                    token::Token::Group(token::Group {
                        delimiter: $open,
                        token_stream: ts,
                    })
                },
            )
            .parse(i)
        }
    };
}