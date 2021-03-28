use rand::{
    distributions::Uniform,
    prelude::{
        Distribution,
        StdRng,
    },
    SeedableRng,
};
use crate::ast::{
    Node,
    Op
};


pub const TEST_SEED: u64 = 10353;


pub type EvalResult<T> = Result<T, ExprError>;

pub struct Expression {
    head: Option<Node>,
    seed: Option<u64>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExprError {
    ZeroDivision,
    ZeroSides,
}


impl Expression {
    pub fn new(head: Node) -> Self {
        Expression { head: Some(head), seed: None }
    }

    pub fn from_seed(head: Node, seed: u64) -> Self {
        Expression { head: Some(head), seed: Some(seed) }
    }

    pub fn eval(&self) -> EvalResult<i32> {
        let mut rng: StdRng = if let Some(seed) = self.seed { StdRng::seed_from_u64(seed) } else { StdRng::from_entropy() };
        Self::eval_recursive(&self.head.as_ref().expect("Head not initialized"), &mut rng)
    }

    fn eval_recursive(head: &Node, mut rng: &mut StdRng) -> EvalResult<i32> {
        let mut l: Option<i32> = None;
        let mut r: Option<i32> = None;

        if let Some(left) = &head.left {
            l = Some(Self::eval_recursive(left, &mut rng).unwrap());
        }

        if let Some(right) = &head.right {
            r = Some(Self::eval_recursive(right, &mut rng).unwrap());
        }

        let l = if let Some(x) = l { x } else { 0 };
        let r = if let Some(x) = r { x } else { 0 };

        match head.op {
            Op::Add => {
                Ok(l + r)
            }
            Op::Sub => {
                Ok(l - r)
            }
            Op::Mul => {
                Ok(l * r)
            }
            Op::Div => {
                if r == 0 {
                    Err(ExprError::ZeroDivision)
                } else {
                    Ok(l / r)
                }
            }
            Op::Number(x) => {
                Ok(x)
            }
            Op::Dice { num, sides } => {
                if sides == 0 {
                    Err(ExprError::ZeroSides)
                } else {
                    Ok(compute_dice(num, sides, rng)
                        .unwrap()
                        .iter()
                        .sum())
                }
            }
        }
    }
}

fn compute_dice(num: i32, sides: i32, rng: &mut StdRng) -> EvalResult<Vec<i32>> {
    if num == 0 {
        Ok(vec![])
    } else {
        let distr = Uniform::new_inclusive(1, sides);
        Ok(distr.sample_iter(rng)
            .take(num as usize)
            .collect()
        )
    }
}