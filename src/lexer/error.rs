use nom::error::ErrorKind;

pub type LexerError<'a> = (&'a str, ErrorKind);