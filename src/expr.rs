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
pub use crate::eval::EvalNode;


pub const TEST_SEED: u64 = 10353;

pub type ExprResult<T> = Result<T, ExprError>;

#[derive(Debug, Eq, PartialEq)]
pub enum ExprError {
    ZeroSides,
}

pub struct Expression {
    head: Option<Node>,
    seed: Option<u64>,
}


impl Expression {
    pub fn new(head: Node) -> Self {
        Expression { head: Some(head), seed: None }
    }

    pub fn from_seed(head: Node, seed: u64) -> Self {
        Expression { head: Some(head), seed: Some(seed) }
    }

    pub fn eval(&self) -> ExprResult<EvalNode> {
        let mut rng: StdRng = if let Some(seed) = self.seed { StdRng::seed_from_u64(seed) } else { StdRng::from_entropy() };
        Self::eval_recursive(&self.head.as_ref().expect("Head not initialized"), &mut rng)
    }

    fn eval_recursive(head: &Node, mut rng: &mut StdRng) -> ExprResult<EvalNode> {
        match head {
            Node::Set { set, ops: _ } => {
                Ok(
                    EvalNode::Set(set
                        .iter()
                        .map(|n| Self::eval_recursive(n.as_ref(), &mut rng).unwrap())
                        .collect::<Vec<EvalNode>>()))
            }
            Node::Node(head) => {
                match &head.op {
                    Op::Parens => {
                        let inner = Self::eval_recursive(head.left.as_ref().unwrap(), &mut rng);
                        Ok(EvalNode::Parens { inner: Box::new(inner.unwrap()) })
                    }
                    Op::Neg => {
                        let inner = Self::eval_recursive(head.right.as_ref().unwrap(), &mut rng);
                        Ok(EvalNode::UnaryOp { op: Op::Neg, inner: Box::new(inner.unwrap()) })
                    }
                    Op::Number(x) => {
                        Ok(EvalNode::Number(*x))
                    }
                    &Op::Dice { num, sides, ops } => {
                        if sides == 0 {
                            Err(ExprError::ZeroSides)
                        } else {
                            let rolls = compute_dice(num, sides, &mut rng);
                            Ok(EvalNode::Dice { num, sides, rolls: rolls.unwrap() })
                        }
                    }
                    _ => {
                        let left = Self::eval_recursive(head.left.as_ref().unwrap(), &mut rng);
                        let right = Self::eval_recursive(head.right.as_ref().unwrap(), &mut rng);
                        Ok(EvalNode::BinaryOp { op: head.op, left: Box::new(left.unwrap()), right: Box::new(right.unwrap()) })
                    }
                }
            }
        }
    }
}

fn compute_dice(num: i32, sides: i32, rng: &mut StdRng) -> ExprResult<Vec<i32>> {
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