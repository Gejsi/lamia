use logos::Logos;

#[derive(Debug)]
pub struct Lexer<'a>(logos::Lexer<'a, Token<'a>>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(Token::lexer(input))
    }
}

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
pub enum Assign {
    Equal,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
    ModuloEqual,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Comma,
    Semicolon,
    Colon,
    RightArrow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Grouping {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Number<'source> {
    Integer(&'source str),
    HexInteger(&'source str),
    OctalInteger(&'source str),
    BinaryInteger(&'source str),

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
#[logos(subpattern int_suffix = r"(i|u)(8|16|32|64|128|size)")]
#[logos(subpattern float_suffix = r"f(32|64)")]
pub enum Token<'source> {
    #[regex("//[^\n]*\n?", |c| Comment::Line(c.slice()))]
    /// TODO: this doesn't handle nestedness
    #[regex(r"/\*(?:[^*]|\*[^/])*\*/", |c| Comment::Block(c.slice()))]
    Comment(Comment<'source>),

    #[token(",", |_| Delimiter::Comma)]
    #[token(";", |_| Delimiter::Semicolon)]
    #[token(":", |_| Delimiter::Colon)]
    #[token("->", |_| Delimiter::RightArrow)]
    Delimiter(Delimiter),

    #[regex("(?&ident)")]
    Identifier(&'source str),

    #[regex(r"'(?:[^']|\\')*'")]
    Character(&'source str),

    // TODO: probably more escapes are needed
    #[regex(r#""(?:[^"]|\\")*""#)]
    String(&'source str),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    #[regex("(?&decimal)(?&int_suffix)?", |n| Number::Integer(n.slice()))]
    #[regex("0[xX](?&hex)(?&int_suffix)?", |n| Number::HexInteger(n.slice()))]
    #[regex("0[oO](?&octal)(?&int_suffix)?", |n| Number::OctalInteger(n.slice()))]
    #[regex("0[bB](?&binary)(?&int_suffix)?", |n| Number::BinaryInteger(n.slice()))]
    #[regex(r#"(((?&decimal)\.(?&decimal)?(?&exp)?(?&float_suffix)?)|(\.(?&decimal)(?&exp)?(?&float_suffix)?)|((?&decimal)(?&exp)(?&float_suffix)?)|((?&decimal)(?&exp)?(?&float_suffix)))"#, |n| Number::Float(n.slice()))]
    #[regex(r"0[xX](((?&hex))|((?&hex)\.)|((?&hex)?\.(?&hex)))[pP][+-]?(?&decimal)(?&float_suffix)?", |n| Number::HexFloat(n.slice()))]
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

    #[token("+=", |_| Assign::PlusEqual)]
    #[token("-=", |_| Assign::MinusEqual)]
    #[token("*=", |_| Assign::StarEqual)]
    #[token("/=", |_| Assign::SlashEqual)]
    #[token("%=", |_| Assign::ModuloEqual)]
    #[token("=", |_| Assign::Equal)]
    Assign(Assign),

    #[token("fn", |_| Keyword::Function)]
    #[token("let", |_| Keyword::Let)]
    #[token("if", |_| Keyword::If)]
    #[token("else", |_| Keyword::Else)]
    #[token("return", |_| Keyword::Return)]
    Keyword(Keyword),

    #[regex(r"[ \t\n\f]+")]
    Whitespace(&'source str),

    #[token("(", |_| Grouping::OpenParen)]
    #[token(")", |_| Grouping::CloseParen)]
    #[token("[", |_| Grouping::OpenSquare)]
    #[token("]", |_| Grouping::CloseSquare)]
    #[token("{", |_| Grouping::OpenBrace)]
    #[token("}", |_| Grouping::CloseBrace)]
    Grouping(Grouping),
}

#[macro_export]
macro_rules! ok_first_token {
    ($src: expr, $expect: expr) => {
        let mut lexer = logos::Lexer::<Token>::new($src);
        assert_eq!(lexer.next(), Some(Ok($expect)));
    };
}

#[macro_export]
macro_rules! err_first_token {
    ($src: expr, $expect: expr) => {
        let mut lexer = logos::Lexer::<lexer::Token>::new($src);
        assert_eq!(lexer.next(), Some(Err($expect)));
    };
}

#[macro_export]
macro_rules! ok_all_tokens {
    ($src: expr, $expect: expr) => {
        let mut lexer = logos::Lexer::<lexer::Token>::new($src);
        let tokens = lexer.collect::<Result<Vec<lexer::Token>, ()>>();
        assert_eq!(tokens, Ok($expect.into()));
    };
}

#[macro_export]
macro_rules! ok_no_whitespace {
    ($src: expr, $expect: expr) => {
        let mut lexer = logos::Lexer::<lexer::Token>::new($src);
        let tokens = lexer
            .filter(|t| !matches!(t, Ok(lexer::Token::Whitespace(_))))
            .collect::<Result<Vec<lexer::Token>, ()>>();
        assert_eq!(tokens, Ok($expect.into()));
    };
}
