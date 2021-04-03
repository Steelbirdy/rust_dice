use lexer::SyntaxKind;
use num_traits::{FromPrimitive, ToPrimitive};


pub(crate) type SyntaxNode = rowan::SyntaxNode<DiceLanguage>;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum DiceLanguage {}

impl rowan::Language for DiceLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}