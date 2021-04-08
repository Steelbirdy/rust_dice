use super::{Database, ExprIdx, Total};


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub(super) expr: Expr,
    kept: bool,
}

impl Expression {
    pub(super) fn new(expr: Expr) -> Self {
        let kept = expr != Expr::Missing;

        Expression { expr, kept }
    }
}

impl Total for Expression {
    fn total(&self, db: &Database) -> i64 {
        if self.kept {
            self.expr.total(db)
        } else {
            0
        }
    }
}


#[derive(Debug, PartialEq)]
pub(super) enum Expr {
    Missing,
    Binary(Binary),
    Dice(Dice),
    Literal(Literal),
    Set(Set),
    Unary(Unary),
}

impl Expr {
    pub(super) fn binary(op: BinaryOp, lhs: ExprIdx, rhs: ExprIdx) -> Self {
        Self::Binary(Binary { op, lhs, rhs })
    }

    pub(super) fn dice(count: Option<u64>, sides: Option<u64>,
                       ops: Vec<SetOperation>, db: &mut Database) -> Expr {
        let dice = Dice::new(count, sides, ops, db);

        Self::Dice(dice)
    }

    pub(super) fn literal(n: Option<u64>) -> Self {
        Self::Literal(Literal::new(n))
    }

    pub(super) fn set(items: Vec<ExprIdx>, ops: Vec<SetOperation>) -> Self {
        Self::Set(Set { items, ops })
    }

    pub(super) fn unary(op: UnaryOp, expr: ExprIdx) -> Self {
        Self::Unary(Unary { op, expr })
    }
}

impl Total for Expr {
    fn total(&self, db: &Database) -> i64 {
        match self {
            Self::Missing => 0,
            Self::Binary(binary) => binary.total(db),
            Self::Dice(dice) => dice.total(db),
            Self::Literal(literal) => literal.total(db),
            Self::Set(set) => set.total(db),
            Self::Unary(unary) => unary.total(db),
        }
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Binary {
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
}

impl Total for Binary {
    fn total(&self, db: &Database) -> i64 {
        let lhs = db.get(self.lhs);
        let lhs = lhs.total(db);

        let rhs = db.get(self.rhs);
        let rhs = rhs.total(db);

        match self.op {
            BinaryOp::Add => lhs + rhs,
            BinaryOp::Sub => lhs - rhs,
            BinaryOp::Mul => lhs * rhs,
            BinaryOp::Div => lhs / rhs,  // TODO: handle division by 0
        }
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Dice {
    count: Option<u64>,
    sides: Option<u64>,
    values: Vec<Die>,
    ops: Vec<SetOperation>,
}

impl Dice {
    fn new(count: Option<u64>, sides: Option<u64>,
           ops: Vec<SetOperation>, db: &mut Database) -> Self {
        if let (Some(sides), Some(count)) = (sides, count) {
            let values: Vec<_> =
                db.roll_many(sides, count)
                    .map(|roll| db.alloc(
                        Expression::new(
                            Expr::literal(Some(roll)))))
                    .map(|idx| Die::new(sides, idx))
                    .collect();

            Self { count: Some(count), sides: Some(sides), values, ops }
        } else {
            Self { count, sides, values: Vec::new(), ops }  // TODO: Passing the buck
        }
    }

    fn roll_another(&mut self, db: &mut Database) {
        let die = Die::roll_new(self.sides.unwrap(), db);  // TODO

        self.values.push(die);
    }
}

impl Total for Dice {
    fn total(&self, db: &Database) -> i64 {
        self.values
            .iter()
            .map(|die| die.total(db))
            .sum()
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Die {
    sides: u64,
    values: Vec<ExprIdx>,
}

impl Die {
    fn new(sides: u64, value: ExprIdx) -> Self {
        Self { sides, values: vec![value] }
    }

    fn roll_new(sides: u64, db: &mut Database) -> Self {
        let value = Expr::literal(Some(db.roll(sides)));
        let value = db.alloc(Expression::new(value));

        Self { sides, values: vec![value] }
    }

    fn reroll(&mut self, db: &mut Database) {
        self.values.pop();

        self.add_roll(db);
    }

    fn explode(&mut self, db: &mut Database) {
        let expr = self.values.last().map(|idx| db.get_mut(*idx));

        if let Some(Expression { expr: Expr::Literal(literal), .. }) = expr {
            literal.explode();
        } else if expr.is_some() {
            unreachable!();
        }
    }

    fn force_value(&mut self, value: u64, db: &mut Database) {
        let expr = self.values.last().map(|idx| db.get_mut(*idx));

        if let Some(Expression { expr: Expr::Literal(literal), .. }) = expr {
            literal.update(value);
        } else if expr.is_some() {
            unreachable!();
        }
    }

    fn add_roll(&mut self, db: &mut Database) {
        let roll = Expression::new(Expr::literal(Some(db.roll(self.sides))));
        let roll = db.alloc(roll);

        self.values.push(roll);
    }
}

impl Total for Die {
    fn total(&self, db: &Database) -> i64 {
        let expr = self.values.last()
            .map(|idx| db.get(*idx));

        if let Some(Expression { expr: Expr::Literal(literal), .. }) = expr {
            literal.total(db)
        } else if expr.is_some() {
            unreachable!()
        } else {
            0  // TODO: Handle no value
        }
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Literal {
    values: Vec<u64>,
    exploded: bool,
}

impl Literal {
    fn new(n: Option<u64>) -> Self {
        let values = n.map(|n| vec![n]).unwrap_or(Vec::new());
        Self { values, exploded: false }
    }

    fn explode(&mut self) {
        self.exploded = true;
    }

    fn update(&mut self, value: u64) {
        self.values.push(value);
    }
}

impl Total for Literal {
    fn total(&self, _db: &Database) -> i64 {
        self.values.last().map(|&x| x as i64).unwrap_or(0)  // TODO: handle missing
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Set {
    items: Vec<ExprIdx>,
    ops: Vec<SetOperation>,
}

impl Total for Set {
    fn total(&self, db: &Database) -> i64 {
        self.items
            .iter()
            .map(|idx| db.get(*idx))
            .map(|expr| expr.total(db))
            .sum()
    }
}


#[derive(Debug, PartialEq)]
pub(super) struct Unary {
    op: UnaryOp,
    expr: ExprIdx,
}

impl Total for Unary {
    fn total(&self, db: &Database) -> i64 {
        let expr = db.get(self.expr);
        let total = expr.total(db);

        match self.op {
            UnaryOp::Neg => -total,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum UnaryOp {
    Neg,
}


#[derive(Debug, PartialEq)]
pub(super) struct SetOperation {
    op: SetOp,
    sel: SetSel,
    num: Option<u64>,
}

impl SetOperation {
    const INCOMPATIBLE_SET_OPS: &'static [(SetOp, &'static [SetSel])] = &[
        (SetOp::Reroll, &[SetSel::Highest, SetSel::Lowest]),
        (SetOp::Min, &[SetSel::Lowest, SetSel::Highest, SetSel::Greater, SetSel::Less]),
        (SetOp::Max, &[SetSel::Lowest, SetSel::Highest, SetSel::Greater, SetSel::Less]),
    ];

    pub(super) fn new(op: SetOp, sel: SetSel, num: Option<u64>) -> Self {
        Self { op, sel, num }
    }

    fn validate(&self) -> bool {
        for (op, sels) in Self::INCOMPATIBLE_SET_OPS.iter() {
            if self.op != *op {
                continue
            }
            for sel in sels.iter() {
                if self.sel == *sel {
                    return false;
                }
            }
        }

        true
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum SetOp {
    Keep,
    Drop,
    Reroll,
    RerollOnce,
    RerollAdd,
    Explode,
    Min,
    Max,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum SetSel {
    Number,
    Highest,
    Lowest,
    Greater,
    Less,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SEED, RollContext};
    use rand::prelude::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn roll(sides: u64) -> i64 {
        StdRng::seed_from_u64(SEED).sample(rand::distributions::Uniform::new_inclusive(1, sides)) as i64
    }

    fn check(input: &str, expected_total: i64) {
        let ast = parse(input);
        let expr = ast.expr();
        let mut db = Database {
            exprs: la_arena::Arena::new(),
            ctx: RollContext::new(StdRng::seed_from_u64(SEED)),
        };
        let hir = db.lower_expr(expr);

        assert_eq!(hir.total(&mut db), expected_total);
    }

    #[test]
    fn total_literal() {
        check("2", 2);
    }

    #[test]
    fn total_dice() {
        check("1d20", roll(20));
    }

    #[test]
    fn total_unary() {
        check("-5", -5);
    }

    #[test]
    fn total_binary() {
        check("1d12 - 2", roll(12) - 2);
    }

    #[test]
    fn total_set() {
        check("(1, 1d6)", roll(6) + 1);
    }

    #[test]
    fn total_expr() {
        let total = 5 * roll(8) - 6;

        check("5 * 1d8 - 6", total);
    }

    #[test]
    fn rng_is_deterministic() {
        let rng1 = StdRng::seed_from_u64(SEED);
        let rng2 = StdRng::seed_from_u64(SEED);

        assert!(
            rng1.sample_iter(rand::distributions::Uniform::new_inclusive(1, 20)).take(10)
                .zip(
                    rng2.sample_iter(rand::distributions::Uniform::new_inclusive(1, 20)).take(10))
                .map(|(a, b)| a == b)
                .reduce(|a, b| a && b)
                .unwrap()
        );
    }

    #[test]
    fn catches_invalid_set_operations() {
        let op = SetOperation::new(SetOp::Reroll, SetSel::Highest, Some(2));
        assert!(!op.validate());
    }

    #[test]
    fn allows_valid_set_operations() {
        let op = SetOperation::new(SetOp::Keep, SetSel::Highest, Some(5));
        assert!(op.validate());
    }
}