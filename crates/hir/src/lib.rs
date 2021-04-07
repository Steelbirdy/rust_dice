mod database;
pub(crate) use database::Database;

mod expr;
pub(crate) use expr::Expression;
pub(crate) use expr::*;

use rand::prelude::*;


pub(crate) type ExprIdx = la_arena::Idx<Expression>;


pub fn roll(ast: ast::Root) -> RollResult {
    RollResult::from(ast)
}


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


#[derive(Debug, PartialEq)]
struct RollContext {
    rng: StdRng,
}

impl RollContext {
    fn new(rng: StdRng) -> Self {
        Self { rng }
    }

    fn roll(&mut self, sides: u64) -> u64 {
        self.rng.gen_range(1..=sides)
    }

    fn roll_many(&mut self, sides: u64, count: u64) -> impl Iterator<Item=u64> {
        rand::distributions::Uniform::new_inclusive(1, sides)
            .sample_iter(self.rng.clone())
            .take(count as usize)
    }
}

impl Default for RollContext {
    fn default() -> Self {
        Self::new(StdRng::from_entropy())
    }
}