use std::fmt;
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
    SetOp,
}

impl SyntaxKind {
    pub const SET_OPERATORS: &'static [SyntaxKind; 8] = &[
        SyntaxKind::Keep,
        SyntaxKind::Drop,
        SyntaxKind::Reroll,
        SyntaxKind::RerollOnce,
        SyntaxKind::RerollAdd,
        SyntaxKind::Explode,
        SyntaxKind::Min,
        SyntaxKind::Max,
    ];

    pub const SET_SELECTORS: &'static [SyntaxKind; 5] = &[
        SyntaxKind::Number,
        SyntaxKind::Highest,
        SyntaxKind::Lowest,
        SyntaxKind::Greater,
        SyntaxKind::Less,
    ];

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

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::Dice => "dice",
            SyntaxKind::Number => "number",
            SyntaxKind::Plus => "'+'",
            SyntaxKind::Minus => "'-'",
            SyntaxKind::Star => "'*'",
            SyntaxKind::Slash => "'/'",
            SyntaxKind::Percent => "'%'",
            SyntaxKind::LParen => "'('",
            SyntaxKind::RParen => "')'",
            SyntaxKind::Comma => "','",
            SyntaxKind::Keep => "'k'",
            SyntaxKind::Drop => "'p'",
            SyntaxKind::Reroll => "'rr'",
            SyntaxKind::RerollOnce => "'ro'",
            SyntaxKind::RerollAdd => "'ra'",
            SyntaxKind::Explode => "'e'",
            SyntaxKind::Min => "'mi'",
            SyntaxKind::Max => "'ma'",
            SyntaxKind::Highest => "'h'",
            SyntaxKind::Lowest => "'l'",
            SyntaxKind::Greater => "'>'",
            SyntaxKind::Less => "'<'",
            _ => unreachable!(),
        })
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