use lexer::{Assign, Comment, Delimiter, Grouping, Keyword, Operator, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // trivia kinds
    LineComment,
    BlockComment,
    Whitespace,

    FunctionKeyword,
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    ReturnKeyword,

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

    SingleEqual, // NOTE: maps to `lexer::Assign::Equal`
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
    ModuloEqual,

    Comma,
    Semicolon,
    Colon,
    RightArrow,

    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenBrace,
    CloseBrace,

    Identifier,
    Number,
    Bool,
    String,
    Character,

    LetStmt,
    ReturnStmt,
    BreakStmt,
    ContinueStmt,
    StructStmt,

    FunctionExpr,
    IfExpr,

    Error,
    Root,

    /// hidden variant to determine how many variants there are
    #[doc(hidden)]
    _LAST,
}

impl<'source> From<Token<'source>> for SyntaxKind {
    fn from(token: Token) -> Self {
        match token {
            Token::Comment(val) => match val {
                Comment::Line(_) => Self::LineComment,
                Comment::Block(_) => Self::BlockComment,
            },
            Token::Delimiter(val) => match val {
                Delimiter::Comma => Self::Comma,
                Delimiter::Semicolon => Self::Semicolon,
                Delimiter::Colon => Self::Colon,
                Delimiter::RightArrow => Self::RightArrow,
            },
            Token::Operator(val) => match val {
                Operator::Equal => Self::Equal,
                Operator::NotEqual => Self::NotEqual,
                Operator::LessThan => Self::LessThan,
                Operator::GreaterThan => Self::GreaterThan,
                Operator::LessThanEqual => Self::LessThanEqual,
                Operator::GreaterThanEqual => Self::GreaterThanEqual,
                Operator::LogicalAnd => Self::LogicalAnd,
                Operator::LogicalOr => Self::LogicalOr,
                Operator::Plus => Self::Plus,
                Operator::Minus => Self::Minus,
                Operator::Bang => Self::Bang,
                Operator::Star => Self::Star,
                Operator::Slash => Self::Slash,
                Operator::Modulo => Self::Modulo,
            },
            Token::Assign(val) => match val {
                Assign::Equal => Self::SingleEqual,
                Assign::PlusEqual => Self::PlusEqual,
                Assign::MinusEqual => Self::MinusEqual,
                Assign::SlashEqual => Self::SlashEqual,
                Assign::StarEqual => Self::StarEqual,
                Assign::ModuloEqual => Self::ModuloEqual,
            },
            Token::Keyword(val) => match val {
                Keyword::Function => Self::FunctionKeyword,
                Keyword::Let => Self::LetKeyword,
                Keyword::If => Self::IfKeyword,
                Keyword::Else => Self::ElseKeyword,
                Keyword::Return => Self::ReturnKeyword,
            },
            Token::Grouping(val) => match val {
                Grouping::OpenParen => Self::OpenParen,
                Grouping::CloseParen => Self::CloseParen,
                Grouping::OpenSquare => Self::OpenSquare,
                Grouping::CloseSquare => Self::CloseSquare,
                Grouping::OpenBrace => Self::OpenBrace,
                Grouping::CloseBrace => Self::CloseBrace,
            },
            Token::Whitespace(_) => Self::Whitespace,
            Token::Identifier(_) => Self::Identifier,
            Token::Character(_) => Self::Character,
            Token::String(_) => Self::String,
            Token::Bool(_) => Self::Bool,
            Token::Number(_) => Self::Number,
        }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= Self::Kind::_LAST as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxElement = rowan::SyntaxElement<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
