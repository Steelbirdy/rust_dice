pub use crate::ast::Op;


pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Eq, PartialEq)]
pub enum EvalError {
    ZeroDivision,
    ZeroSides,
}


pub enum EvalNode {
    BinaryOp { op: Op, left: Box<EvalNode>, right: Box<EvalNode> },
    Number(i32),
    Dice { num: i32, sides: i32, rolls: Vec<i32> },
}

impl EvalNode {
    #[allow(non_snake_case)]
    pub fn Add(left: EvalNode, right: EvalNode) -> Self {
        EvalNode::BinaryOp { op: Op::Add, left: Box::new(left), right: Box::new(right) }
    }

    #[allow(non_snake_case)]
    pub fn Sub(left: EvalNode, right: EvalNode) -> Self {
        EvalNode::BinaryOp { op: Op::Sub, left: Box::new(left), right: Box::new(right) }
    }

    #[allow(non_snake_case)]
    pub fn Mul(left: EvalNode, right: EvalNode) -> Self {
        EvalNode::BinaryOp { op: Op::Mul, left: Box::new(left), right: Box::new(right) }
    }

    #[allow(non_snake_case)]
    pub fn Div(left: EvalNode, right: EvalNode) -> Self {
        EvalNode::BinaryOp { op: Op::Div, left: Box::new(left), right: Box::new(right) }
    }

    pub fn value(&self) -> EvalResult<i32> {
        match self {
            EvalNode::BinaryOp { op, left, right } => {
                let l = left.value().unwrap();
                let r = right.value().unwrap();

                match op {
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
                            Err(EvalError::ZeroDivision)
                        } else {
                            Ok(l / r)
                        }
                    }
                    _ => unreachable!()
                }
            }
            EvalNode::Number(x) => {
                Ok(*x)
            }
            EvalNode::Dice { num: _, sides: _, rolls } => {
                Ok(rolls.iter().sum())
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EvalNode::BinaryOp { op, left, right } => {
                let o = match op {
                    Op::Add => "+",
                    Op::Sub => "-",
                    Op::Mul => "*",
                    Op::Div => "/",
                    _ => unreachable!(),
                };

                format!("{} {} {}", left.to_string(), o, right.to_string())
            }
            EvalNode::Number(x) => {
                format!("{}", x)
            }
            EvalNode::Dice { num, sides, rolls } => {
                let rolls_str = format!("{:?}", rolls);
                format!("{}d{} ({})", num, sides, &rolls_str.as_str()[1..rolls_str.len() - 1])
            }
        }
    }
}