use super::ExprIdx;


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub(super) expr: Expr,
    kept: bool,
}

impl Expression {
    pub(super) fn new(expr: Expr) -> Self {
        let kept = expr != Expr::Missing;

        Expression { expr, kept }
    }
}


#[derive(Debug, PartialEq)]
pub(super) enum Expr {
    Missing,
    Binary(Binary),
    Dice(Dice),
    Literal(Literal),
    Set(Set),
    Unary(Unary),
}

impl Expr {
    pub(super) fn binary(op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx) -> Self {
        Self::Binary(Binary { op, lhs, rhs })
    }

    pub(super) fn dice(count: Option<u64>, sides: Option<u64>, ops: Vec<SetOperation>) -> Self {
        Self::Dice(Dice { count, sides, ops })
    }

    pub(super) fn literal(n: Option<u64>) -> Self {
        Self::Literal(Literal { n })
    }

    pub(super) fn set(items: Vec<ExprIdx>, ops: Vec<SetOperation>) -> Self {
        Self::Set(Set { items, ops })
    }

    pub(super) fn unary(op: UnaryOp, expr: ExprIdx) -> Self {
        Self::Unary(Unary { op, expr })
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Binary {
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
}


#[derive(Debug, PartialEq)]
pub(super) struct Dice {
    count: Option<u64>,
    sides: Option<u64>,
    ops: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
pub(super) struct Literal {
    n: Option<u64>,
}


#[derive(Debug, PartialEq)]
pub(super) struct Set {
    items: Vec<ExprIdx>,
    ops: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
pub(super) struct Unary {
    op: UnaryOp,
    expr: ExprIdx,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum UnaryOp {
    Neg,
}


pub(super) type SetOperation = (SetOp, SetSel, Option<u64>);


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum SetOp {
    Keep,
    Drop,
    Reroll,
    RerollOnce,
    RerollAdd,
    Explode,
    Min,
    Max,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum SetSel {
    Number,
    Highest,
    Lowest,
    Greater,
    Less,
}