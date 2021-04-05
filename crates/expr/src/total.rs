use super::*;


pub(super) trait Total {
    fn total(&self, db: &Database) -> ExprResult<i64>;
}


impl Total for Expression {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        let res = if !self.kept {
            0
        } else {
            self.kind.total(db)?
        };

        Ok(res)
    }
}


impl Total for ExprKind {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        match self {
            Self::BinaryExpr(x) => x.total(db),
            Self::Dice(x) => x.total(db),
            Self::Die(x) => x.total(db),
            Self::Literal(x) => x.total(db),
            Self::Set(x) => x.total(db),
            Self::UnaryExpr(x) => x.total(db),
        }
    }
}


impl Total for BinaryExpr {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        let lhs = db.total_idx(self.lhs)?;
        let rhs = db.total_idx(self.rhs)?;

        match self.op {
            BinaryOp::Add => Ok(lhs + rhs),
            BinaryOp::Sub => Ok(lhs - rhs),
            BinaryOp::Mul => Ok(lhs * rhs),
            BinaryOp::Div => {
                if rhs == 0 {
                    Err(ExprError::DivideByZero)
                } else {
                    Ok(lhs / rhs)
                }
            }
        }
    }
}


impl Total for Dice {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        self.values.iter().map(|&die| db.total_idx(die)).sum()
    }
}


impl Total for Die {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        self.values.last()
            .map(|&idx| db.total_idx(idx))
            .unwrap_or(Err(ExprError::NoValues))
    }
}


impl Total for Literal {
    fn total(&self, _db: &Database) -> ExprResult<i64> {
        self.values.last()
            .map(|v| *v as i64)
            .ok_or(ExprError::NoValues)
    }
}


impl Total for Set {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        let items: ExprResult<Vec<i64>> = self.items.iter()
            .map(|item| db.total_idx(*item))
            .collect();
        items.map(|v| v.iter().sum())
    }
}


impl Total for UnaryExpr {
    fn total(&self, db: &Database) -> ExprResult<i64> {
        let res = db.total_idx(self.expr)?;

        match self.op {
            UnaryOp::Neg => Ok(-res),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn build_dice(sides: u64, values: &[u64]) -> (Expression, Database) {
        let mut db = Database::default();
        let mut res = Vec::new();

        for v in values.iter() {
            let tmp = db.alloc(Expression::Literal(*v));
            let tmp = db.alloc(Expression::Die(sides, tmp));
            res.push(tmp);
        }

        let dice = Expression::Dice(sides, res, Vec::new());
        (dice, db)
    }

    fn check_total(expr: Expression, db: Database, expected_total: ExprResult<i64>) {
        let total = db.total(&expr);

        assert_eq!(total.ok(), expected_total.ok())
    }

    #[test]
    fn total_literal() {
        check_total(
            Expression::Literal(2),
            Database::default(),
            Ok(2),
        );
    }

    #[test]
    fn total_die() {
        let mut db = Database::default();
        let idx = db.alloc(Expression::Literal(5));

        check_total(
            Expression::Die(12, idx),
            db,
            Ok(5),
        );
    }

    #[test]
    fn total_dice() {
        let values = [1, 5, 8, 3, 4];
        let (dice, db) = build_dice(8, &values);

        check_total(
            dice,
            db,
            Ok(values.iter().map(|v| *v as i64).sum()),
        );
    }

    #[test]
    fn total_set() {
        let mut db = Database::default();
        let items = vec![
            db.alloc(Expression::Literal(5)),
            db.alloc(Expression::Literal(13)),
        ];

        check_total(
            Expression::Set(items, Vec::new()),
            db,
            Ok(18),
        );
    }

    #[test]
    fn total_binary_expr() {
        let rolls: [u64; 3] = [18, 4, 11];
        let (lhs, mut db) = build_dice(20, &rolls);

        let lhs = db.alloc(lhs);
        let rhs = db.alloc(Expression::Literal(5));

        check_total(
            Expression::BinaryExpr(lhs, rhs, BinaryOp::Sub),
            db,
            Ok(28),
        );
    }

    #[test]
    fn total_unary_expr() {
        let mut db = Database::default();
        let expr = db.alloc(Expression::Literal(2));

        check_total(
            Expression::UnaryExpr(expr, UnaryOp::Neg),
            db,
            Ok(-2),
        );
    }
}