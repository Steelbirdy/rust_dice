mod database;
pub use database::Database;

use la_arena::Idx;
type ExprIdx = Idx<Expr>;


pub fn lower(ast: ast::Root) -> (Database, Expr) {
    let mut db = Database::default();
    let lowered_expr = db.lower_expr(ast.expr());

    (db, lowered_expr)
}


#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary { op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx },
    Dice { count: u64, sides: u64, ops: Vec<SetOperation> },
    Literal { n: u64 },
    Set { items: Vec<Self>, ops: Vec<SetOperation> },
    Unary { op: UnaryOp, expr: ExprIdx },
}


#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}


#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}


pub type SetOperation = (SetOp, SetSel, u64);


#[derive(Debug)]
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


#[derive(Debug)]
pub enum SetSel {
    Number,
    Highest,
    Lowest,
    Greater,
    Less,
}