mod database;
pub(crate) use database::Database;

mod expr;
pub(crate) use expr::Expression;
pub(crate) use expr::*;

pub(crate) type ExprIdx = la_arena::Idx<Expression>;


#[derive(Debug)]
pub struct RollResult {
    ast: ast::Root,
    expr: Expression,
    db: Database,
}

impl From<ast::Root> for RollResult {
    fn from(ast: ast::Root) -> Self {
        let mut db = Database::default();
        let expr = db.lower_expr(ast.expr());

        Self { ast, expr, db }
    }
}