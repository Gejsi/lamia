use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
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
    BlockStmt,
    ReturnStmt,

    FunctionExpr,
    IfExpr,

    Error,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxElement = rowan::SyntaxElement<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
