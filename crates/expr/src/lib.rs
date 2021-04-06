mod database;
mod total;

pub(crate) use database::{Database, ExprIdx};
pub(crate) use total::Total;
use hir::{BinaryOp, SetOp, SetSel, UnaryOp};


enum ExprError {
    DivideByZero,
    NoValues,
}


type ExprResult<T> = Result<T, ExprError>;


#[derive(Debug, PartialEq)]
struct Expression {
    kind: ExprKind,
    kept: bool,
}

impl Expression {
    fn new(kind: ExprKind) -> Self {
        Self {
            kind,
            kept: true,
        }
    }

    #[allow(non_snake_case)]
    fn BinaryExpr(lhs: ExprIdx, rhs: ExprIdx, op: BinaryOp) -> Self {
        Self::new(ExprKind::BinaryExpr(BinaryExpr { lhs, rhs, op }))
    }

    #[allow(non_snake_case)]
    fn Dice(count: u64, sides: u64, ops: Vec<SetOperation>) -> Self {
        let res = Dice {
            count,
            sides,
            values: Vec::new(),
            operations: ops,
        };

        Self::new(ExprKind::Dice(res))
    }

    #[allow(non_snake_case)]
    fn Die(sides: u64, value: ExprIdx) -> Self {
        let res = Die {
            sides,
            values: vec![value],
        };

        Self::new(ExprKind::Die(res))
    }

    #[allow(non_snake_case)]
    fn Literal(value: u64) -> Self {
        Self::new(ExprKind::Literal(Literal::new(value)))
    }

    #[allow(non_snake_case)]
    fn Set(items: Vec<ExprIdx>, ops: Vec<SetOperation>) -> Self {
        Self::new(ExprKind::Set(Set { items, ops }))
    }

    #[allow(non_snake_case)]
    fn UnaryExpr(expr: ExprIdx, op: UnaryOp) -> Self {
        Self::new(ExprKind::UnaryExpr(UnaryExpr { expr, op }))
    }
}


#[derive(Debug, PartialEq)]
enum ExprKind {
    BinaryExpr(BinaryExpr),
    Dice(Dice),
    Die(Die),
    Literal(Literal),
    Set(Set),
    UnaryExpr(UnaryExpr),
}


#[derive(Debug, PartialEq)]
struct BinaryExpr {
    lhs: ExprIdx,
    rhs: ExprIdx,
    op: BinaryOp,
}


#[derive(Debug, PartialEq)]
struct Dice {
    count: u64,
    sides: u64,
    values: Vec<ExprIdx>,
    operations: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
struct Die {
    sides: u64,
    values: Vec<ExprIdx>,
}


#[derive(Debug, PartialEq)]
struct Literal {
    values: Vec<u64>,
    exploded: bool,
}

impl Literal {
    fn new(value: u64) -> Self {
        Self {
            values: vec![value],
            exploded: false,
        }
    }
}


#[derive(Debug, PartialEq)]
struct Set {
    items: Vec<ExprIdx>,
    ops: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
struct UnaryExpr {
    expr: ExprIdx,
    op: UnaryOp,
}


#[derive(Debug, PartialEq)]
struct SetOperation {
    op: SetOp,
    sel: SetSel,
    num: u64,
}