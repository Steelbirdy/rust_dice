use crate::{BinaryOp, Expr, SetOperation, SetOp, SetSel, UnaryOp};
use la_arena::Arena;
use syntax::SyntaxKind;


#[derive(Debug, Default)]
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

    fn lower_set(&mut self, ast: ast::Set) -> Expr {
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