use super::{Expression, ExprResult, Total};


pub(crate) type ExprIdx = la_arena::Idx<Expression>;


#[derive(Debug, PartialEq, Default)]
pub(crate) struct Database {
    arena: la_arena::Arena<Expression>,
}

impl Database {
    pub(crate) fn get(&self, idx: ExprIdx) -> &Expression {
        &self.arena[idx]
    }

    pub(crate) fn alloc(&mut self, expr: Expression) -> ExprIdx {
        self.arena.alloc(expr)
    }

    pub(crate) fn total(&self, expr: &Expression) -> ExprResult<i64> {
        expr.kind.total(&self)
    }

    pub(crate) fn total_idx(&self, idx: ExprIdx) -> ExprResult<i64> {
        self.total(self.get(idx))
    }
}