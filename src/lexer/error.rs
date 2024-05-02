use nom::error::ErrorKind;
use super::Span;

pub type LexerError<'a> = (Span<'a>, ErrorKind);