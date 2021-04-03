use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};


#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    Dice,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LParen,
    RParen,
    Comma,
    Keep,
    Drop,
    Reroll,
    RerollOnce,
    RerollAdd,
    Explode,
    Min,
    Max,
    Highest,
    Lowest,
    Greater,
    Less,
    Error,

    Root,
    DiceExpr,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    SetExpr,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Dice => Self::Dice,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Percent => Self::Percent,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::Comma => Self::Comma,
            TokenKind::Keep => Self::Keep,
            TokenKind::Drop => Self::Drop,
            TokenKind::Reroll => Self::Reroll,
            TokenKind::RerollOnce => Self::RerollOnce,
            TokenKind::RerollAdd => Self::RerollAdd,
            TokenKind::Explode => Self::Explode,
            TokenKind::Min => Self::Min,
            TokenKind::Max => Self::Max,
            TokenKind::Highest => Self::Highest,
            TokenKind::Lowest => Self::Lowest,
            TokenKind::Greater => Self::Greater,
            TokenKind::Less => Self::Less,
            TokenKind::Error => Self::Error,
        }
    }
}


pub type SyntaxNode = rowan::SyntaxNode<DiceLanguage>;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DiceLanguage {}

impl rowan::Language for DiceLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}