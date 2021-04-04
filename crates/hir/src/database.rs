use crate::{BinaryOp, Expr, SetOperation, SetOp, SetSel, UnaryOp};
use la_arena::Arena;
use syntax::SyntaxKind;


#[derive(Debug, PartialEq, Default)]
pub struct Database {
    exprs: Arena<Expr>,
}

impl Database {
    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expr {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(ast),
                ast::Expr::Dice(ast) => self.lower_dice(ast),
                ast::Expr::Literal(ast) => Expr::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()),
                ast::Expr::Set(ast) => self.lower_set(ast),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(ast),
            }
        } else {
            Expr::Missing
        }
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

    fn lower_set(&mut self, ast: ast::Set) -> Expr {  // TODO: Allocate set items in arena?
        let items = ast.items().map(|expr| self.lower_expr(Some(expr))).collect();
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

    fn check_expr(input: &str, expected_hir: Expr, expected_database: Database) {
        let root = parse(input);
        let expr = root.expr();
        let mut database = Database::default();
        let hir = database.lower_expr(expr);

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: 1 });
        let rhs = exprs.alloc(Expr::Literal { n: 2 });

        check_expr(
            "1 + 2",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Add,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_dice_no_ops() {
        check_expr(
            "1d12",
            Expr::Dice { count: 1, sides: 12, ops: Vec::new() },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_implicit_count() {
        check_expr(
            "d20",
            Expr::Dice { count: 1, sides: 20, ops: Vec::new() },
            Database::default(),
        );
    }

    #[test]
    fn lower_percentage_dice() {
        check_expr(
            "3d%",
            Expr::Dice { count: 3, sides: 100, ops: Vec::new() },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_one_op() {
        check_expr(
            "2d20kh1",
            Expr::Dice {
                count: 2,
                sides: 20,
                ops: vec![
                    (SetOp::Keep, SetSel::Highest, 1),
                ],
            },
            Database::default(),
        );
    }

    #[test]
    fn lower_dice_multiple_ops() {
        check_expr(
            "2d20pl1ro<2e5",
            Expr::Dice {
                count: 2,
                sides: 20,
                ops: vec![
                    (SetOp::Drop, SetSel::Lowest, 1),
                    (SetOp::RerollOnce, SetSel::Less, 2),
                    (SetOp::Explode, SetSel::Number, 5),
                ],
            },
            Database::default(),
        );
    }

    #[test]
    fn lower_literal() {
        check_expr(
            "999",
            Expr::Literal { n: 999 },
            Database::default(),
        );
    }

    #[test]
    fn lower_empty_set() {
        check_expr(
            "()",
            Expr::Set { items: Vec::new(), ops: Vec::new() },
            Database::default(),
        );
    }

    #[test]
    fn lower_singleton_set() {
        let inner = Expr::Literal { n: 2 };

        check_expr(
            "(2,)",
            Expr::Set { items: vec![inner], ops: Vec::new() },
            Database::default(),
        );
    }

    #[test]
    fn lower_set() {
        let mut exprs = Arena::new();
        let items = vec![
            Expr::Unary { expr: exprs.alloc(Expr::Literal { n: 10 }), op: UnaryOp::Neg },
            Expr::Dice { count: 8, sides: 6, ops: Vec::new() },
            Expr::Literal { n: 3 },
        ];

        check_expr(
            "(-10, 8d6, 3)",
            Expr::Set { items, ops: Vec::new() },
            Database { exprs },
        );
    }

    #[test]
    fn lower_set_with_ops() {
        let items = vec![
            Expr::Literal { n: 100, },
            Expr::Dice { count: 2, sides: 100, ops: Vec::new() },
        ];

        check_expr(
            "(100, 2d100)e100",
            Expr::Set { items, ops: vec![(SetOp::Explode, SetSel::Number, 100)] },
            Database::default(),
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let inner = exprs.alloc(Expr::Dice { count: 3, sides: 4, ops: Vec::new() });

        check_expr(
            "-3d4",
            Expr::Unary {
                expr: inner,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }
}