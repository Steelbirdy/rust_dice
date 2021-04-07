mod database;
pub use database::Database;

use la_arena::Idx;
pub type ExprIdx = Idx<Expression>;


pub fn lower(ast: ast::Root) -> (Database, Expression) {
    let mut db = Database::default();
    let lowered_expr = db.lower_expr(ast.expr());

    (db, lowered_expr)
}


#[derive(Debug, PartialEq)]
pub struct Expression {
    expr: Expr,
    kept: bool,
}


#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary(Binary),
    Dice(Dice),
    Literal(Literal),
    Set(Set),
    Unary(Unary),
}

impl Expr {
    fn binary(op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx) -> Self {
        Self::Binary(Binary { op, lhs, rhs })
    }

    fn dice(count: Option<u64>, sides: Option<u64>, ops: Vec<SetOperation>) -> Self {
        Self::Dice(Dice { count, sides, ops })
    }

    fn literal(n: Option<u64>) -> Self {
        Self::Literal(Literal { n })
    }

    fn set(items: Vec<ExprIdx>, ops: Vec<SetOperation>) -> Self {
        Self::Set(Set { items, ops })
    }

    fn unary(op: UnaryOp, expr: ExprIdx) -> Self {
        Self::Unary(Unary { op, expr })
    }
}


#[derive(Debug, PartialEq)]
pub struct Binary {
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
}


#[derive(Debug, PartialEq)]
pub struct Dice {
    count: Option<u64>,
    sides: Option<u64>,
    ops: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
pub struct Literal {
    n: Option<u64>,
}


#[derive(Debug, PartialEq)]
pub struct Set {
    items: Vec<ExprIdx>,
    ops: Vec<SetOperation>,
}


#[derive(Debug, PartialEq)]
pub struct Unary {
    op: UnaryOp,
    expr: ExprIdx,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
}


pub type SetOperation = (SetOp, SetSel, Option<u64>);


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SetOp {
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
pub enum SetSel {
    Number,
    Highest,
    Lowest,
    Greater,
    Less,
}