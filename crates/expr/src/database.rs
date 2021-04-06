use super::*;
use std::ops::{Index, IndexMut};


pub(crate) type ExprIdx = la_arena::Idx<Expression>;


#[derive(Debug, PartialEq)]
pub(crate) enum DatabaseError {
    Missing,
}


#[derive(Debug, PartialEq, Default)]
pub(crate) struct Database {
    arena: la_arena::Arena<Expression>,
}

impl Database {
    pub(crate) fn get(&self, idx: ExprIdx) -> &Expression {
        self.arena.index(idx)
    }

    pub(crate) fn get_mut(&mut self, idx: ExprIdx) -> &mut Expression {
        self.arena.index_mut(idx)
    }

    pub(crate) fn alloc(&mut self, expr: Expression) -> ExprIdx {
        self.arena.alloc(expr)
    }

    pub(crate) fn total(&self, expr: &Expression) -> ExprResult<i64> {
        expr.kind.total(self)
    }

    pub(crate) fn total_idx(&self, idx: ExprIdx) -> ExprResult<i64> {
        self.total(self.get(idx))
    }

    pub(super) fn raise_expr(&mut self, expr: &hir::Expr, hir_db: &hir::Database) -> Result<Expression, DatabaseError> {
        match expr {
            hir::Expr::Binary { lhs, rhs, op } => {
                self.raise_binary(lhs, rhs, *op, hir_db)
            }
            hir::Expr::Dice { count, sides, ops } => {
                self.raise_dice(*count, *sides, ops)
            }
            hir::Expr::Literal { n } => {
                self.raise_literal(*n)
            }
            hir::Expr::Set { items, ops } => {
                self.raise_set(items, ops, hir_db)
            }
            hir::Expr::Unary { expr, op } => {
                self.raise_unary(expr, *op, hir_db)
            }
            hir::Expr::Missing => Err(DatabaseError::Missing),
        }
    }

    fn raise_binary(&mut self, lhs: &hir::ExprIdx, rhs: &hir::ExprIdx,
                    op: BinaryOp, hir_db: &hir::Database) -> Result<Expression, DatabaseError> {
        let lhs = hir_db.get(*lhs);
        let lhs = self.raise_expr(lhs, hir_db)?;
        let lhs = self.alloc(lhs);

        let rhs = hir_db.get(*rhs);
        let rhs = self.raise_expr(rhs, hir_db)?;
        let rhs = self.alloc(rhs);

        Ok(Expression::BinaryExpr(lhs, rhs, op))
    }

    fn raise_dice(&mut self, count: Option<u64>, sides: Option<u64>,
                  ops: &Vec<hir::SetOperation>) -> Result<Expression, DatabaseError> {
        let count = count.ok_or(DatabaseError::Missing)?;
        let sides = sides.ok_or(DatabaseError::Missing)?;

        let ops = ops.iter()
            .map(|op| self.raise_set_operation(op))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Expression::Dice(count, sides, ops))
    }

    fn raise_literal(&mut self, n: Option<u64>) -> Result<Expression, DatabaseError> {
        let n = n.ok_or(DatabaseError::Missing)?;

        Ok(Expression::Literal(n))
    }

    fn raise_set(&mut self, items: &Vec<hir::Expr>, ops: &Vec<hir::SetOperation>,
                 hir_db: &hir::Database) -> Result<Expression, DatabaseError> {
        let items = items.iter()
            .map(|item| self.raise_expr(item, hir_db))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|item| self.alloc(item))
            .collect();

        let ops = ops.iter()
            .map(|op| self.raise_set_operation(op))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Expression::Set(items, ops))
    }

    fn raise_unary(&mut self, expr: &hir::ExprIdx, op: UnaryOp, hir_db: &hir::Database) -> Result<Expression, DatabaseError> {
        let expr = hir_db.get(*expr);
        let expr = self.raise_expr(expr, hir_db)?;
        let expr = self.alloc(expr);

        Ok(Expression::UnaryExpr(expr, op))
    }

    fn raise_set_operation(&mut self, operation: &(SetOp, SetSel, Option<u64>)) -> Result<SetOperation, DatabaseError> {
        let (op, sel, n) = operation;
        let n = n.ok_or(DatabaseError::Missing)?;

        Ok(SetOperation { op: *op, sel: *sel, num: n })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_expr(input: &str, expected_expr: Expression, expected_db: Database) {
        let root = parse(input);
        let (hir_db, lowered) = hir::lower(root);
        let mut db = Database::default();
        let expression = db.raise_expr(&lowered, &hir_db).ok().unwrap();

        assert_eq!(expression, expected_expr);
        assert_eq!(db, expected_db);
    }

    fn check_err(input: &str, expected_err: DatabaseError) {
        let root = parse(input);
        let (hir_db, lowered) = hir::lower(root);
        let mut db = Database::default();
        let res = db.raise_expr(&lowered, &hir_db);

        assert_eq!(res, Err(expected_err));
    }

    #[test]
    fn raise_binary_expr() {
        let mut arena = la_arena::Arena::new();
        let lhs = arena.alloc(Expression::Literal(1));
        let rhs = arena.alloc(Expression::Literal(2));

        check_expr(
            "1+2",
            Expression::BinaryExpr(lhs, rhs, BinaryOp::Add),
            Database { arena },
        );
    }

    #[test]
    fn err_missing_binary_expr() {
        check_err(
            "3+",
            DatabaseError::Missing,
        );
    }

    #[test]
    fn raise_dice() {
        check_expr(
            "2d20kl1rr<5",
            Expression::Dice(2, 20, vec![
                SetOperation { op: SetOp::Keep, sel: SetSel::Lowest, num: 1 },
                SetOperation { op: SetOp::Reroll, sel: SetSel::Less, num: 5 },
            ]),
            Database::default(),
        );
    }

    #[test]
    fn raise_literal() {
        check_expr(
            "12345",
            Expression::Literal(12345),
            Database::default(),
        );
    }

    #[test]
    fn raise_set() {
        let mut arena = la_arena::Arena::new();
        check_expr(
            "(1,2d10)e4",
            Expression::Set(vec![
                arena.alloc(Expression::Literal(1)),
                arena.alloc(Expression::Dice(2, 10, Vec::new())),
            ], vec![
                SetOperation { op: SetOp::Explode, sel: SetSel::Number, num: 4 },
            ]),
            Database { arena },
        );
    }

    #[test]
    fn raise_unary_expr() {
        let mut arena = la_arena::Arena::new();
        check_expr(
            "-3",
            Expression::UnaryExpr(
                arena.alloc(Expression::Literal(3)),
                UnaryOp::Neg,
            ),
            Database { arena },
        );
    }
}