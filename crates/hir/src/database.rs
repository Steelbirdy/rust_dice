use crate::{BinaryOp, Expr, ExprIdx, Expression, SetOp, SetOperation, SetSel, UnaryOp};
use la_arena::Arena;
use syntax::SyntaxKind;
use std::ops::Index;


#[derive(Debug, PartialEq, Default)]
pub struct Database {
    exprs: Arena<Expression>,
}

impl Database {
    pub fn get(&self, idx: ExprIdx) -> &Expression {
        self.exprs.index(idx)
    }

    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expression {
        let expr = if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(ast),
                ast::Expr::Dice(ast) => self.lower_dice(ast),
                ast::Expr::Literal(ast) => Expr::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()).expr,
                ast::Expr::Set(ast) => self.lower_set(ast),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(ast),
            }
        } else {
            Expr::Missing
        };

        let kept = expr != Expr::Missing;

        Expression { expr, kept }
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
        let rhs = self.lower_expr(ast.rhs());

        Expr::Binary {
            op,
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
        }
    }

    fn lower_dice(&mut self, ast: ast::Dice) -> Expr {
        let ops = ast.ops().map(|op| self.lower_set_op(op)).collect();

        Expr::Dice {
            count: ast.count(),
            sides: ast.sides(),
            ops,
        }
    }

    fn lower_set(&mut self, ast: ast::Set) -> Expr {
        let mut items = Vec::new();

        for item in ast.items().into_iter() {
            let item = self.lower_expr(Some(item));
            let item = self.exprs.alloc(item);

            items.push(item);
        }

        let ops = ast.ops().map(|op| self.lower_set_op(op)).collect();

        Expr::Set {
            items,
            ops,
        }
    }

    fn lower_unary(&mut self, ast: ast::UnaryExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        Expr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
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

        (op, sel, ast.num())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_expr(input: &str, expected_hir: Expression, expected_database: Database) {
        let root = parse(input);
        let expr = root.expr();
        let mut database = Database::default();
        let hir = database.lower_expr(expr);

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    fn alloc(arena: &mut Arena<Expression>, expr: Expr) -> ExprIdx {
        arena.alloc(Expression { expr, kept: true })
    }

    fn alloc_missing(arena: &mut Arena<Expression>) -> ExprIdx {
        arena.alloc(Expression { expr: Expr::Missing, kept: false })
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = alloc(&mut exprs, Expr::Literal { n: Some(1) });
        let rhs = alloc(&mut exprs, Expr::Literal { n: Some(2) });

        let expr = Expr::Binary { lhs, rhs, op: BinaryOp::Add };

        check_expr(
            "1 + 2",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_dice_no_ops() {
        let expr = Expr::Dice { count: Some(1), sides: Some(12), ops: Vec::new() };

        check_expr(
            "1d12",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_implicit_count() {
        let expr = Expr::Dice { count: Some(1), sides: Some(20), ops: Vec::new() };

        check_expr(
            "d20",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_percentage_dice() {
        let expr = Expr::Dice { count: Some(3), sides: Some(100), ops: Vec::new() };

        check_expr(
            "3d%",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_one_op() {
        let expr = Expr::Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                (SetOp::Keep, SetSel::Highest, Some(1)),
            ],
        };

        check_expr(
            "2d20kh1",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_multiple_ops() {
        let expr = Expr::Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                (SetOp::Drop, SetSel::Lowest, Some(1)),
                (SetOp::RerollOnce, SetSel::Less, Some(2)),
                (SetOp::Explode, SetSel::Number, Some(5)),
            ],
        };

        check_expr(
            "2d20pl1ro<2e5",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_literal() {
        let expr = Expr::Literal { n: Some(999) };

        check_expr(
            "999",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_empty_set() {
        let expr = Expr::Set { items: Vec::new(), ops: Vec::new() };

        check_expr(
            "()",
            Expression { expr, kept: true },
            Database::default(),
        );
    }

    #[test]
    fn lower_singleton_set() {
        let mut exprs = Arena::new();
        let items: Vec<ExprIdx> = vec![
            alloc(&mut exprs, Expr::Literal { n: Some(2) }),
        ];

        assert_eq!(items.len(), 1);
        let expr = Expr::Set { items, ops: Vec::new() };

        check_expr(
            "(2,)",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_set() {
        let mut exprs = Arena::new();
        let items: Vec<ExprIdx> = vec![
            Expr::Unary { expr: alloc(&mut exprs, Expr::Literal { n: Some(10) }), op: UnaryOp::Neg },
            Expr::Dice { count: Some(8), sides: Some(6), ops: Vec::new() },
            Expr::Literal { n: Some(3) },
        ].into_iter()
            .map(|expr| alloc(&mut exprs, expr))
            .collect();

        assert_eq!(items.len(), 3);
        let expr = Expr::Set { items, ops: Vec::new() };

        check_expr(
            "(-10, 8d6, 3)",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_set_with_ops() {
        let mut exprs = Arena::new();
        let items: Vec<ExprIdx> = vec![
            Expr::Literal { n: Some(100) },
            Expr::Dice { count: Some(2), sides: Some(100), ops: Vec::new() },
        ].into_iter()
            .map(|expr| alloc(&mut exprs, expr))
            .collect();

        assert_eq!(items.len(), 2);
        let expr = Expr::Set { items, ops: vec![(SetOp::Explode, SetSel::Number, Some(100))] };

        check_expr(
            "(100, 2d100)e100",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let inner = alloc(&mut exprs,
                          Expr::Dice { count: Some(3), sides: Some(4), ops: Vec::new() });

        let expr = Expr::Unary {
            expr: inner,
            op: UnaryOp::Neg,
        };

        check_expr(
            "-3d4",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = alloc(&mut exprs, Expr::Literal { n: Some(10) });
        let rhs = alloc_missing(&mut exprs);

        let expr = Expr::Binary {
            lhs,
            rhs,
            op: BinaryOp::Sub,
        };

        check_expr(
            "10 -",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut exprs = Arena::new();
        let expr = alloc_missing(&mut exprs);

        let expr = Expr::Unary {
            expr,
            op: UnaryOp::Neg,
        };

        check_expr(
            "-",
            Expression { expr, kept: true },
            Database { exprs },
        );
    }
}