use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::not_line_ending,
    combinator::{cut, map},
    error::{Error, ErrorKind, ParseError},
    sequence::{delimited, preceded},
    Err as NErr, IResult,
};

use super::{LexerError, Span};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Comment<'a> {
    LineComment(&'a str),
    BlockComment(&'a str),
}

pub fn lex_line_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    map(
        preceded(tag("//"), cut(not_line_ending)),
        |comment: Span| Comment::LineComment(comment.into_fragment()),
    )(i)
}

// pub fn nested_comment(i: Span) -> IResult<Span, Span, LexerError> {
//     match take_until("*/")(i) {
//         Ok((i, comment)) => {
//             // no nested block detected, since the remaining input is just `*/`
//             if i.fragment() == &"*/" {
//                 Ok((i, comment))
//             } else {
//                 match preceded(tag("*/"), nested_comment)(i) {
//                     Ok((remaining, final_comment)) => {
//                         todo!("accumulate the entire comment");
//                     }
//                     Err(err) => return Err(err),
//                 }
//             }
//         }
//         Err(err) => return Err(err),
//     }
// }

const OPEN_BLOCK_COMMENT_BRACKET: &str = "/*";
const CLOSE_BLOCK_COMMENT_BRACKET: &str = "*/";

// TODO: refactor this function through the use of nom
pub fn take_until_unbalanced() -> impl Fn(Span) -> IResult<Span, Span, LexerError> {
    move |i: Span| {
        let mut index = 0;
        let mut bracket_counter = 0;

        while let Some(n) = &i
            .get(index..)
            .ok_or(NErr::Error((i, ErrorKind::TakeUntil)))?
            .find(OPEN_BLOCK_COMMENT_BRACKET)
            .or_else(|| {
                i.get(index..)
                    .and_then(|s| s.find(CLOSE_BLOCK_COMMENT_BRACKET))
            })
        {
            index += n;
            let mut it = i
                .get(index..)
                .ok_or(NErr::Error((i, ErrorKind::TakeUntil)))?
                .chars()
                .peekable();

            let cur_char = it.next().unwrap_or_default();

            if let Some(peek_char) = it.peek() {
                if cur_char == '/' && peek_char == &'*' {
                    bracket_counter += 1;
                    index += OPEN_BLOCK_COMMENT_BRACKET.len();
                } else if cur_char == '*' && peek_char == &'/' {
                    bracket_counter -= 1;
                    index += CLOSE_BLOCK_COMMENT_BRACKET.len();
                }
            }

            // no matched closing bracket found
            if bracket_counter == -1 {
                // do not consume it
                index -= CLOSE_BLOCK_COMMENT_BRACKET.len();
                let remaining = i
                    .get(index..)
                    .ok_or(NErr::Error((i, ErrorKind::TakeUntil)))?;
                let matching = i
                    .get(0..index)
                    .ok_or(NErr::Error((i, ErrorKind::TakeUntil)))?;
                return Ok((remaining.into(), matching.into()));
            };
        }

        if bracket_counter == 0 {
            Ok(("".into(), i))
        } else {
            Err(NErr::Error((i, ErrorKind::TakeUntil)))
        }
    }
}

pub fn lex_block_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    map(
        delimited(
            tag(OPEN_BLOCK_COMMENT_BRACKET),
            take_until_unbalanced(),
            tag(CLOSE_BLOCK_COMMENT_BRACKET),
        ),
        |comment: Span| Comment::BlockComment(comment.into_fragment()),
    )(i)
}

pub fn lex_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    alt((lex_line_comment, lex_block_comment))(i)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_lex_eq,
        lexer::{lex_comment, Comment},
    };

    #[test]
    fn match_line_comment() {
        assert_lex_eq!(
            lex_comment("// Lorem ipsum\t".into()),
            Comment::LineComment(" Lorem ipsum\t")
        );
        assert_lex_eq!(lex_comment("// ".into()), Comment::LineComment(" "));
        assert_lex_eq!(lex_comment("//".into()), Comment::LineComment(""));
    }

    #[test]
    fn match_simple_block_comment() {
        assert_lex_eq!(
            lex_comment("/*test*/".into()),
            Comment::BlockComment("test")
        );
        assert_lex_eq!(lex_comment("/* */".into()), Comment::BlockComment(" "));
    }

    #[test]
    fn match_multiline_block_comment() {
        assert_lex_eq!(
            lex_comment("/* Lorem\nipsum */".into()),
            Comment::BlockComment(" Lorem\nipsum ")
        );
        assert_lex_eq!(
            lex_comment("/* Line 1\nLine 2\nLine 3 */".into()),
            Comment::BlockComment(" Line 1\nLine 2\nLine 3 ")
        );
    }

    #[test]
    fn fail_missing_delimiter_block_comment() {
        assert!(lex_comment("/* test".into()).is_err());
        assert!(lex_comment("test */".into()).is_err());
        assert!(lex_comment("test".into()).is_err());
    }

    #[test]
    fn match_nested_block_comment() {
        assert_lex_eq!(
            lex_comment("/*Nested /* Block */ Comment*/".into()),
            Comment::BlockComment("Nested /* Block */ Comment")
        );

        assert_lex_eq!(
            lex_comment("/*Nested /* Block /* Lorem */ */ Comment*/".into()),
            Comment::BlockComment("Nested /* Block /* Lorem */ */ Comment")
        );

        // TODO: make this test work
        // assert_lex_eq!(
        //     lex_comment("/*Nested /* Block */ /* Lorem */ Comment*/".into()),
        //     Comment::BlockComment("Nested /* Block /* Lorem */ */ Comment")
        // );
    }
}
