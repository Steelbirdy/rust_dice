use super::*;
use std::collections::HashSet;
use std::iter::FromIterator;


fn select_dice(op: &SetOperation, target: &mut Dice, db: &mut Database, max_targets: Option<usize>) -> HashSet<usize> {
    let SetOperation { op: _, sel, num } = op;
    let num = num.unwrap();
    let inum = num as i64;

    let max_targets = max_targets.unwrap_or(num as usize);

    let res =
        target.values
            .iter()
            .map(|d| d.total(db))
            .enumerate();

    let res: Vec<usize> = match sel {
        SetSel::Highest => {
            let mut res: Vec<_> = res.collect();
            res.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
            res.iter()
                .take(max_targets)
                .map(|(i, _)| *i)
                .collect()
        }
        SetSel::Lowest => {
            let mut res: Vec<_> = res.collect();
            res.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));
            res.iter()
                .take(max_targets)
                .map(|(i, _)| *i)
                .collect()
        }
        SetSel::Number => {
            res.filter(|(_, d)| *d == inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
        SetSel::Greater => {
            res.filter(|(_, d)| *d > inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
        SetSel::Less => {
            res.filter(|(_, d)| *d < inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
    };

    HashSet::from_iter(res)
}


fn select_set(op: &SetOperation, target: &mut Set, db: &mut Database) -> HashSet<usize> {
    let SetOperation { op: _, sel, num } = op;
    let max_targets = num.unwrap() as usize;
    let inum = max_targets as i64;

    let res =
        target.items
            .iter()
            .map(|d| db.get(*d))
            .enumerate();

    let res: Vec<usize> = match sel {
        SetSel::Highest => {
            let mut res: Vec<_> = res.collect();
            res.sort_by(|(_, d1), (_, d2)| d2.total(db).cmp(&d1.total(db)));
            res.iter()
                .take(max_targets)
                .map(|(i, _)| *i)
                .collect()
        }
        SetSel::Lowest => {
            let mut res: Vec<_> = res.collect();
            res.sort_by(|(_, d1), (_, d2)| d1.total(db).cmp(&d2.total(db)));
            res.iter()
                .take(max_targets)
                .map(|(i, _)| *i)
                .collect()
        }
        SetSel::Number => {
            res.filter(|(_, d)| d.total(db) == inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
        SetSel::Greater => {
            res.filter(|(_, d)| d.total(db) > inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
        SetSel::Less => {
            res.filter(|(_, d)| d.total(db) < inum)
                .take(max_targets)
                .map(|(i, _)| i)
                .collect()
        }
    };

    HashSet::from_iter(res)
}


pub(super) fn operate_on_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    match op.op {
        SetOp::Keep | SetOp::Drop => keep_or_drop_dice(op, target, db),
        SetOp::Reroll => reroll_dice(op, target, db),
        SetOp::RerollOnce => reroll_dice_once(op, target, db),
        SetOp::Explode => explode_dice(op, target, db),
        SetOp::RerollAdd => explode_dice_once(op, target, db),
        SetOp::Min => min_dice(op, target, db),
        SetOp::Max => max_dice(op, target, db),
    }
}

pub(super) fn operate_on_set(op: &SetOperation, target: &mut Set, db: &mut Database) {
    match op.op {
        SetOp::Keep | SetOp::Drop => {},
        _ => panic!("Keep and Drop are the only operations valid on sets."),
    }

    let selection: HashSet<usize> = select_set(op, target, db);
    let is_drop = op.op == SetOp::Drop;

    for (i, idx) in target.items.iter_mut().enumerate() {
        if selection.contains(&i) == is_drop {
            db.get_mut(*idx).drop();
        }
    }
}


fn keep_or_drop_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let selection = select_dice(op, target, db, None);
    let is_drop = op.op == SetOp::Drop;

    for (i, d) in target.values.iter_mut().enumerate() {
        if selection.contains(&i) == is_drop {
            d.drop();
        }
    }
}

fn reroll_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let mut to_reroll: HashSet<usize> = select_dice(op, target, db, None);

    while !to_reroll.is_empty() {
        for (i, d) in target.values.iter_mut().enumerate() {
            if to_reroll.contains(&i) {
                d.reroll(db);
            }
        }

        to_reroll = select_dice(op, target, db, None);
    };
}

fn reroll_dice_once(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let to_reroll: HashSet<usize> = select_dice(op, target, db, None);

    for (i, d) in target.values.iter_mut().enumerate() {
        if to_reroll.contains(&i) {
            d.reroll(db);
        }
    }
}

fn explode_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let mut already_exploded: HashSet<usize> = HashSet::new();
    let mut to_explode: HashSet<usize> = select_dice(op, target, db, None);

    while !to_explode.is_empty() {
        for (i, d) in target.values.iter_mut().enumerate() {
            if to_explode.contains(&i) {
                d.explode(db);
            }
        }

        already_exploded = already_exploded.union(&to_explode).map(|x| *x).collect();
        to_explode = select_dice(op, target, db, None)
            .difference(&already_exploded)
            .map(|x| *x).
            collect();
    }
}

fn explode_dice_once(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let to_explode: HashSet<usize> = select_dice(op, target, db, None);

    for (i, d) in target.values.iter_mut().enumerate() {
        if to_explode.contains(&i) {
            d.explode(db);
        }
    }
}

fn min_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let min = op.num.unwrap();
    let imin = min as i64;

    for d in target.values.iter_mut() {
        if d.total(db) < imin {
            d.force_value(min, db);
        }
    }
}

fn max_dice(op: &SetOperation, target: &mut Dice, db: &mut Database) {
    let max = op.num.unwrap();
    let imax = max as i64;

    for d in target.values.iter_mut() {
        if d.total(db) > imax {
            d.force_value(max, db);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SEED, RollContext};
    use rand::prelude::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn roll(count: usize, sides: u64, db: &mut Database) -> (Vec<u64>, Vec<ExprIdx>) {
        let distr = rand::distributions::Uniform::new_inclusive(1, sides);

        StdRng::seed_from_u64(SEED).sample_iter(distr).take(count)
            .map(|x| (x, db.alloc(Expression::new(Expr::literal(Some(x))))))
            .unzip()
    }

    fn check_dice(input_dice: Expression, db: &mut Database, ops: Vec<SetOperation>, expected_output: Dice, expected_db: &mut Database) {
        let mut input_dice = input_dice;

        for op in ops.iter() {
            op.operate(&mut input_dice, db);
        }

        if let Expression { expr: Expr::Dice(dice), .. } = input_dice {
            assert_eq!(dice, expected_output);
            assert_eq!(db, expected_db);
        } else {
            panic!()
        }
    }

    fn default_db() -> Database {
        Database {
            exprs: la_arena::Arena::new(),
            ctx: RollContext::new(StdRng::seed_from_u64(SEED)),
        }
    }

    #[test]
    fn keep_highest_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15]
        let (rolls, idxs) = roll(2, 20, &mut output_db);
        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
            ],
            values: vec![
                Die {
                    sides: 20,
                    values: vec![idxs[0]],
                    kept: false,
                },
                Die {
                    sides: 20,
                    values: vec![idxs[1]],
                    kept: true,
                },
            ],
        };
    }

    #[test]
    fn drop_highest_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::Drop, SetSel::Highest, Some(1)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15]
        let (rolls, idxs) = roll(2, 20, &mut output_db);

        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::Drop, SetSel::Highest, Some(1)),
            ],
            values: vec![
                Die {
                    sides: 20,
                    values: vec![idxs[0]],
                    kept: true,
                },
                Die {
                    sides: 20,
                    values: vec![idxs[1]],
                    kept: false,
                },
            ],
        };
    }

    #[test]
    fn reroll_highest_once_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::RerollOnce, SetSel::Highest, Some(1)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15, ..]
        let (rolls, idxs) = roll(3, 20, &mut output_db);
        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::RerollOnce, SetSel::Highest, Some(1)),
            ],
            values: vec![
                Die {
                    sides: 20,
                    values: vec![idxs[0]],
                    kept: true,
                },
                Die {
                    sides: 20,
                    values: vec![idxs[1], idxs[2]],
                    kept: true,
                },
            ],
        };
    }

    #[test]
    fn reroll_lowest_once_keep_highest_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::RerollOnce, SetSel::Lowest, Some(1)),
            SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15, 16]
        let (rolls, idxs) = roll(3, 20, &mut output_db);
        // dbg!(&rolls, &idxs);
        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::RerollOnce, SetSel::Lowest, Some(1)),
                SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
            ],
            values: vec![
                Die {
                    sides: 20,
                    values: vec![idxs[0], idxs[2]],
                    kept: true,
                },
                Die {
                    sides: 20,
                    values: vec![idxs[1]],
                    kept: false,
                },
            ],
        };
    }

    #[test]
    fn explode_greater_than_10_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::Explode, SetSel::Greater, Some(10)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15, 16, 17, 5]
        let (rolls, idxs) = roll(5, 20, &mut output_db);
        // dbg!(&rolls, &idxs);
        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::Explode, SetSel::Greater, Some(10)),
            ],
            values: idxs.into_iter()
                .map(|idx| Die { sides: 20, values: vec![idx], kept: true })
                .collect(),
        };
    }

    #[test]
    fn reroll_add_5_dice() {
        let mut db = default_db();
        let input_dice = Dice::new(Some(2), Some(20), vec![
            SetOperation::new(SetOp::RerollAdd, SetSel::Number, Some(5)),
        ], &mut db);

        let mut output_db = default_db();

        // [5, 15, 16]
        let (rolls, idxs) = roll(3, 20, &mut output_db);
        // dbg!(&rolls, &idxs);
        let mut output_dice = Dice {
            count: Some(2),
            sides: Some(20),
            ops: vec![
                SetOperation::new(SetOp::RerollAdd, SetSel::Number, Some(5)),
            ],
            values: idxs.into_iter()
                .map(|idx| Die { sides: 20, values: vec![idx], kept: true })
                .collect(),
        };
    }

    #[test]
    fn keep_highest_set() {
        let mut db = default_db();
        let tmp = Expr::dice(Some(2), Some(20), vec![], &mut db);
        let input_set = Set {
            items: vec![
                db.alloc(Expression::new(tmp)),
                db.alloc(Expression::new(Expr::literal(Some(7)))),
                db.alloc(Expression::new(Expr::literal(Some(25)))),
            ],
            ops: vec![
                SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
            ],
        };

        let (rolls, idxs) = roll(2, 20, &mut db);

        let mut output_set = Set {
            items: vec![
                db.alloc(Expression {
                    expr: Expr::Dice(Dice {
                        count: Some(2),
                        sides: Some(20),
                        ops: Vec::new(),
                        values: idxs.into_iter()
                            .map(|idx| Die { sides: 20, values: vec![idx], kept: true })
                            .collect()
                    }),
                    kept: false,
                }),
                db.alloc(Expression {
                    expr: Expr::literal(Some(7)),
                    kept: false,
                }),
                db.alloc(Expression {
                    expr: Expr::literal(Some(25)),
                    kept: true,
                }),
            ],
            ops: vec![
                SetOperation::new(SetOp::Keep, SetSel::Highest, Some(1)),
            ],
        };
    }
}