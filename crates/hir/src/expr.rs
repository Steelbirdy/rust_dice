use super::{Database, ExprIdx};


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

    pub(super) fn dice(count: Option<u64>, sides: Option<u64>,
                       ops: Vec<SetOperation>, db: &mut Database) -> Expr {
        let dice = Dice::new(count, sides, ops, db);

        Self::Dice(dice)
    }

    pub(super) fn literal(n: Option<u64>) -> Self {
        Self::Literal(Literal::new(n))
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
    values: Vec<Die>,
    ops: Vec<SetOperation>,
}

impl Dice {
    fn new(count: Option<u64>, sides: Option<u64>,
           ops: Vec<SetOperation>, db: &mut Database) -> Self {
        if let (Some(sides), Some(count)) = (sides, count) {
            let mut values: Vec<_> =
                db.roll_many(sides, count)
                    .map(|roll| db.alloc(
                        Expression::new(
                            Expr::literal(Some(roll)))))
                    .map(|idx| Die::new(sides, idx))
                    .collect();

            Self { count: Some(count), sides: Some(sides), values, ops }
        } else {
            Self { count, sides, values: Vec::new(), ops }  // TODO: Passing the buck
        }
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Die {
    sides: u64,
    values: Vec<ExprIdx>,
}

impl Die {
    fn new(sides: u64, value: ExprIdx) -> Self {
        Self { sides, values: vec![value] }
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Literal {
    n: Option<u64>,
}

impl Literal {
    fn new(n: Option<u64>) -> Self {
        Self { n }
    }
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


#[derive(Debug, PartialEq)]
pub(super) struct SetOperation {
    op: SetOp,
    sel: SetSel,
    num: Option<u64>,
}

impl SetOperation {
    pub(super) fn new(op: SetOp, sel: SetSel, num: Option<u64>) -> Self {
        Self { op, sel, num }
    }
}


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