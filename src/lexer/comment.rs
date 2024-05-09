use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

use super::{LexerError, Span};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Comment<'a> {
    value: &'a str,
    kind: CommentKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CommentKind {
    LineComment,
    BlockComment,
}

pub fn lex_line_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    map(preceded(tag("//"), is_not("\n\r")), |text: Span| Comment {
        value: text.into_fragment(),
        kind: CommentKind::LineComment,
    })(i)
}

// TODO: handle nested block comments
pub fn lex_block_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    map(
        delimited(tag("/*"), take_until("*/"), tag("*/")),
        |text: Span| Comment {
            value: text.into_fragment(),
            kind: CommentKind::BlockComment,
        },
    )(i)
}

pub fn lex_comment(i: Span) -> IResult<Span, Comment, LexerError> {
    alt((lex_line_comment, lex_block_comment))(i)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_lex_eq,
        lexer::{comment::CommentKind, lex_comment, Comment},
    };

    #[test]
    fn match_line_comment() {
        assert_lex_eq!(
            lex_comment("// Lorem ipsum\t".into()),
            Comment {
                value: " Lorem ipsum\t",
                kind: CommentKind::LineComment
            }
        );
        assert_lex_eq!(
            lex_comment("// ".into()),
            Comment {
                value: " ",
                kind: CommentKind::LineComment
            }
        );
        assert!(lex_comment("//".into()).is_err());
    }

    #[test]
    fn match_block_comment() {
        assert_lex_eq!(
            lex_comment("/*test*/".into()),
            Comment {
                value: "test",
                kind: CommentKind::BlockComment
            }
        );
        assert_lex_eq!(
            lex_comment("/* Lorem\nipsum */".into()),
            Comment {
                value: " Lorem\nipsum ",
                kind: CommentKind::BlockComment
            }
        );
        assert_lex_eq!(
            lex_comment("/* */".into()),
            Comment {
                value: " ",
                kind: CommentKind::BlockComment
            }
        );
    }
}
