use super::*;
use la_arena::Arena;
use syntax::SyntaxKind;
use std::ops::{Index, IndexMut};


#[derive(Debug, PartialEq, Default)]
pub struct Database {
    exprs: Arena<Expression>,
    ctx: RollContext,
}

impl Database {
    pub(super) fn get(&self, idx: ExprIdx) -> &Expression {
        self.exprs.index(idx)
    }

    pub(super) fn get_mut(&mut self, idx: ExprIdx) -> &mut Expression {
        self.exprs.index_mut(idx)
    }

    pub(super) fn alloc(&mut self, expr: Expression) -> ExprIdx {
        self.exprs.alloc(expr)
    }

    pub(super) fn roll(&mut self, sides: u64) -> u64 {
        self.ctx.roll(sides)
    }

    pub(super) fn roll_many(&mut self, sides: u64, count: u64) -> impl Iterator<Item=u64> {
        self.ctx.roll_many(sides, count)
    }

    pub(super) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expression {
        let expr = if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(ast),
                ast::Expr::Dice(ast) => self.lower_dice(ast),
                ast::Expr::Literal(ast) => Expr::literal(ast.parse()),
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()).expr,
                ast::Expr::Set(ast) => self.lower_set(ast),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(ast),
            }
        } else {
            Expr::Missing
        };

        Expression::new(expr)
    }

    fn lower_binary(&mut self, ast: ast::BinaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(ast.lhs());
        let lhs = self.exprs.alloc(lhs);
        
        let rhs = self.lower_expr(ast.rhs());
        let rhs = self.exprs.alloc(rhs);
        
        Expr::binary(op, lhs, rhs)
    }

    fn lower_dice(&mut self, ast: ast::Dice) -> Expr {
        let ops = ast.ops().map(|op| self.lower_set_op(op)).collect();

        Expr::dice(ast.count(), ast.sides(), ops, self)
    }

    fn lower_set(&mut self, ast: ast::Set) -> Expr {
        let mut items = Vec::new();

        for item in ast.items().into_iter() {
            let item = self.lower_expr(Some(item));
            let item = self.exprs.alloc(item);

            items.push(item);
        }

        let ops = ast.ops().map(|op| self.lower_set_op(op)).collect();

        Expr::set(items, ops)
    }

    fn lower_unary(&mut self, ast: ast::UnaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());
        let expr = self.exprs.alloc(expr);
        
        Expr::unary(op, expr)
    }

    fn lower_set_op(&mut self, ast: ast::SetOp) -> SetOperation {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Keep => SetOp::Keep,
            SyntaxKind::Drop => SetOp::Drop,
            SyntaxKind::Reroll => SetOp::Reroll,
            SyntaxKind::RerollOnce => SetOp::RerollOnce,
            SyntaxKind::RerollAdd => SetOp::RerollAdd,
            SyntaxKind::Explode => SetOp::Explode,
            SyntaxKind::Min => SetOp::Min,
            SyntaxKind::Max => SetOp::Max,
            _ => unreachable!(),
        };

        let sel = ast.sel()
            .map_or(SetSel::Number,
                    |token| match token.kind() {
                        SyntaxKind::Highest => SetSel::Highest,
                        SyntaxKind::Lowest => SetSel::Lowest,
                        SyntaxKind::Greater => SetSel::Greater,
                        SyntaxKind::Less => SetSel::Less,
                        _ => unreachable!(),
                    });

        SetOperation::new(op, sel, ast.num())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 10353;

    fn default_db() -> Database {
        Database {
            exprs: Arena::new(),
            ctx: RollContext::new(
                StdRng::seed_from_u64(SEED)
            ),
        }
    }

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_expr(input: &str, expected_hir: Expression, expected_database: Database) {
        let root = parse(input);
        let expr = root.expr();
        let mut database = default_db();
        let hir = database.lower_expr(expr);

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    fn alloc(db: &mut Database, expr: Expr) -> ExprIdx {
        db.alloc(Expression::new(expr))
    }

    fn alloc_missing(db: &mut Database) -> ExprIdx {
        db.alloc(Expression::new(Expr::Missing))
    }

    #[test]
    fn lower_binary_expr() {
        let mut db = default_db();
        let lhs = alloc(&mut db, Expr::literal(Some(1)));
        let rhs = alloc(&mut db, Expr::literal(Some(2)));

        let expr = Expr::binary(BinaryOp::Add, lhs, rhs);

        check_expr(
            "1 + 2",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_dice_no_ops() {
        let mut db = default_db();
        let expr = Expr::dice(Some(1), Some(12), Vec::new(), &mut db);

        check_expr(
            "1d12",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_dice_implicit_count() {
        let mut db = default_db();
        let expr = Expr::dice(Some(1), Some(20), Vec::new(), &mut db);

        check_expr(
            "d20",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_percentage_dice() {
        let mut db = default_db();
        let expr = Expr::dice(Some(3), Some(100), Vec::new(), &mut db);

        check_expr(
            "3d%",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_dice_one_op() {
        let mut db = default_db();
        let expr = Expr::dice(Some(2), Some(20), vec![
            SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1))
        ], &mut db);

        check_expr(
            "2d20kh1",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_dice_multiple_ops() {
        let mut db = default_db();
        let expr = Expr::dice(Some(2), Some(20), vec![
            SetOperation::new(SetOp::Drop, SetSel::Lowest, Some(1)),
            SetOperation::new(SetOp::RerollOnce, SetSel::Less, Some(2)),
            SetOperation::new(SetOp::Explode, SetSel::Number, Some(5)),
        ], &mut db);

        check_expr(
            "2d20pl1ro<2e5",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_literal() {
        let expr = Expr::literal(Some(999));

        check_expr(
            "999",
            Expression::new(expr),
            default_db(),
        );
    }

    #[test]
    fn lower_empty_set() {
        let expr = Expr::set(Vec::new(), Vec::new());

        check_expr(
            "()",
            Expression::new(expr),
            default_db(),
        );
    }

    #[test]
    fn lower_singleton_set() {
        let mut db = default_db();
        let items: Vec<ExprIdx> = vec![
            alloc(&mut db, Expr::literal(Some(2))),
        ];

        assert_eq!(items.len(), 1);
        let expr = Expr::set(items, Vec::new());

        check_expr(
            "(2,)",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_set() {
        let mut db = default_db();
        let items: Vec<ExprIdx> = vec![
            Expr::dice(Some(8), Some(6), Vec::new(), &mut db),
            Expr::literal(Some(3)),
        ].into_iter()
            .map(|expr| alloc(&mut db, expr))
            .collect();

        assert_eq!(items.len(), 2);
        let expr = Expr::set(items, Vec::new());

        check_expr(
            "(8d6, 3)",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_set_with_ops() {
        let mut db = default_db();
        let mut items: Vec<_> = Vec::new();

        let item = Expr::literal(Some(100));
        items.push(alloc(&mut db, item));

        let item = Expr::dice(Some(2), Some(100), Vec::new(), &mut db);
        items.push(alloc(&mut db, item));

        assert_eq!(items.len(), 2);
        let expr = Expr::set(items, vec![
            SetOperation::new(SetOp::Explode, SetSel::Number, Some(100))
        ]);

        check_expr(
            "(100, 2d100)e100",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut db = default_db();
        let inner = Expr::dice(Some(3), Some(4), Vec::new(), &mut db);
        let inner = alloc(&mut db, inner);

        let expr = Expr::unary(UnaryOp::Neg, inner);

        check_expr(
            "-3d4",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut db = default_db();
        let lhs = alloc(&mut db, Expr::literal(Some(10)));
        let rhs = alloc_missing(&mut db);

        let expr = Expr::binary(BinaryOp::Sub, lhs, rhs);

        check_expr(
            "10 -",
            Expression::new(expr),
            db,
        );
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut db = default_db();
        let expr = alloc_missing(&mut db);

        let expr = Expr::unary(UnaryOp::Neg, expr);

        check_expr(
            "-",
            Expression::new(expr),
            db,
        );
    }
}