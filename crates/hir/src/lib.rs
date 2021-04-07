mod database;
pub(crate) use database::Database;

mod expr;
pub use expr::Expression;
pub(crate) use expr::*;

use la_arena::Idx;
pub(crate) type ExprIdx = Idx<Expression>;


pub fn lower(ast: ast::Root) -> (Database, Expression) {
    let mut db = Database::default();
    let lowered_expr = db.lower_expr(ast.expr());

    (db, lowered_expr)
}


