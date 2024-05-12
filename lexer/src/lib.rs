use logos::Logos;

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Function,
    Let,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comment<'source> {
    Line(&'source str),
    Block(&'source str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,

    Plus,
    Minus,
    Bang,
    Star,
    Slash,
    Modulo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    Semicolon,
    Colon,
    RightArrow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Number<'source> {
    Integer(&'source str),
    HexInteger(&'source str),
    OctalInteger(&'source str),
    BinaryInteger(&'source str),

    // NOTE: Not sure we want to keep the sign in the float regexp, other
    // numbers (in particular, the integer) don't have it.
    Float(&'source str),
    HexFloat(&'source str),
}

// many patters are from https://github.com/maciejhirsz/logos/issues/133
#[derive(Logos, Debug, PartialEq, Eq)]
#[logos(subpattern ident = r"[\p{XID_Start}_]\p{XID_Continue}*")]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern octal = r"[0-7][_0-7]*")]
#[logos(subpattern binary = r"[0-1][_0-1]*")]
#[logos(subpattern exp = r"[eE][+-]?[0-9][_0-9]*")]
pub enum Token<'source> {
    #[regex("//[^\n]*\n?", |c| Comment::Line(c.slice()))]
    /// TODO: this doesn't handle nestedness
    #[regex(r"/\*(?:[^*]|\*[^/])*\*/", |c| Comment::Block(c.slice()))]
    Comment(Comment<'source>),

    #[token(",", |_| Punctuation::Comma)]
    #[token(";", |_| Punctuation::Semicolon)]
    #[token(":", |_| Punctuation::Colon)]
    #[token("->", |_| Punctuation::RightArrow)]
    Punctuation(Punctuation),

    #[regex("(?&ident)")]
    Identifier(&'source str),

    #[regex(r"'(?:[^']|\\')*'")]
    Character(&'source str),

    // TODO: probably more escapes are needed
    #[regex("\"(?:[^\"]|\\\")*\"")]
    String(&'source str),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex("(?&decimal)", |n| Number::Integer(n.slice()))]
    #[regex("0[xX](?&hex)", |n| Number::HexInteger(n.slice()))]
    #[regex("0[oO](?&octal)", |n| Number::OctalInteger(n.slice()))]
    #[regex("0[bB](?&binary)", |n| Number::BinaryInteger(n.slice()))]
    // NOTE: Not sure we want to keep the sign in the float regexp, other
    // numbers (in particular, the integer) don't have it.
    #[regex(r#"[+-]?(((?&decimal)\.(?&decimal)?(?&exp)?[fFdD]?)|(\.(?&decimal)(?&exp)?[fFdD]?)|((?&decimal)(?&exp)[fFdD]?)|((?&decimal)(?&exp)?[fFdD]))"#, |n| Number::Float(n.slice()))]
    #[regex(r"0[xX](((?&hex))|((?&hex)\.)|((?&hex)?\.(?&hex)))[pP][+-]?(?&decimal)[fFdD]?", |n| Number::HexFloat(n.slice()))]
    Number(Number<'source>),

    #[token("==", |_| Operator::Equal)]
    #[token("!=", |_| Operator::NotEqual)]
    #[token("<", |_| Operator::LessThan)]
    #[token(">", |_| Operator::GreaterThan)]
    #[token("<=", |_| Operator::LessThanEqual)]
    #[token(">=", |_| Operator::GreaterThanEqual)]
    #[token("&&", |_| Operator::LogicalAnd)]
    #[token("||", |_| Operator::LogicalOr)]
    #[token("+", |_| Operator::Plus)]
    #[token("-", |_| Operator::Minus)]
    #[token("!", |_| Operator::Bang)]
    #[token("*", |_| Operator::Star)]
    #[token("/", |_| Operator::Slash)]
    #[token("%", |_| Operator::Modulo)]
    Operator(Operator),

    #[token("fn", |_| Keyword::Function)]
    #[token("let", |_| Keyword::Let)]
    #[token("if", |_| Keyword::If)]
    #[token("else", |_| Keyword::Else)]
    #[token("return", |_| Keyword::Return)]
    Keyword(Keyword),
}

#[macro_export]
macro_rules! ok_first_token {
    ($src: expr, $expect: expr) => {
        let mut lexer = Token::lexer($src);
        assert_eq!(lexer.next(), Some(Ok($expect)));
    };
}

#[macro_export]
macro_rules! err_first_token {
    ($src: expr, $expect: expr) => {
        let mut lexer = Token::lexer($src);
        assert_eq!(lexer.next(), Some(Err($expect)));
    };
}
