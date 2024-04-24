#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub literal: &'a str,
    pub source: TokenSource,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal<'a> {
    Integer(i64),
    String(&'a str),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Plus,
    Minus,
    Bang,
    Star,
    Slash,
    Modulo,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    AndAnd,
    OrOr,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind<'a> {
    /// Unknown token, not expected by the lexer, e.g. "№"
    Illegal,
    Eof,

    Identifier(&'a str),
    Literal(Literal<'a>),
    Operator(Operator),
    Keyword(Keyword),

    Assign {
        operator: Option<Operator>
    },

    Comma,
    Semicolon,
    Colon,

    Group {
        delimiter: Delimiter,
        tokens: &'a [Token<'a>],
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delimiter {
    Paren,
    Square,
    Brace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenSource {
    pub position: (usize, usize),
}