use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};


#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}


#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Dice(Dice),
    Literal(Literal),
    ParenExpr(ParenExpr),
    Set(Set),
    UnaryExpr(UnaryExpr),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::DiceExpr => Self::Dice(Dice(node)),
            SyntaxKind::Literal => Self::Literal(Literal(node)),
            SyntaxKind::ParenExpr => Self::ParenExpr(ParenExpr(node)),
            SyntaxKind::SetExpr => Self::Set(Set(node)),
            SyntaxKind::PrefixExpr => Self::UnaryExpr(UnaryExpr(node)),
            _ => return None,
        };

        Some(result)
    }
}


#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| matches!(
                token.kind(),
                SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Star | SyntaxKind::Slash,
            ))
    }
}


#[derive(Debug)]
pub struct Dice(SyntaxNode);

impl Dice {
    pub fn count(&self) -> u64 {
        self.0.first_token().unwrap().text().split("d").nth(0).unwrap().parse().unwrap()
    }

    pub fn sides(&self) -> u64 {
        self.0.first_token().unwrap().text().split("d").nth(1).unwrap().parse().unwrap()
    }

    pub fn ops(&self) -> impl Iterator<Item=SetOp> {
        self.0.children()
            .filter(|node| node.kind() == SyntaxKind::SetOp)
            .map(SetOp)
    }
}


#[derive(Debug)]
pub struct Literal(SyntaxNode);

impl Literal {
    pub fn parse(&self) -> u64 {
        self.0.first_token().unwrap().text().parse().unwrap()
    }
}


#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}


#[derive(Debug)]
pub struct Set(SyntaxNode);

impl Set {
    pub fn items(&self) -> impl Iterator<Item=Expr> {
        self.0.children()
            .filter_map(Expr::cast)
    }

    pub fn ops(&self) -> impl Iterator<Item=SetOp> {
        self.0.children()
            .filter(|node| node.kind() == SyntaxKind::SetOp)
            .map(SetOp)
    }
}


#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Minus)
    }
}


#[derive(Debug)]
pub struct SetOp(SyntaxNode);

impl SetOp {
    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| SyntaxKind::SET_OPERATORS.contains(&token.kind()))
    }

    pub fn sel(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| SyntaxKind::SET_SELECTORS.contains(&token.kind()))
    }

    pub fn num(&self) -> u64 {
        self.0
            .last_child()
            .unwrap()
            .first_token()
            .unwrap()
            .text()
            .parse()
            .unwrap()
    }
}