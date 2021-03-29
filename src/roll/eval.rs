pub use crate::ast::Op;


pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Eq, PartialEq)]
pub enum EvalError {
    ZeroDivision,
    ZeroSides,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EvalNode {
    BinaryOp { op: Op, left: Box<EvalNode>, right: Box<EvalNode> },
    UnaryOp { op: Op, inner: Box<EvalNode> },
    Parens { inner: Box<EvalNode> },
    Number(i32),
    Dice { num: i32, sides: i32, rolls: Vec<DiceRoll> },
    Set(Vec<EvalNode>)
}

#[derive(Debug, Eq, PartialEq)]
pub struct DiceRoll {
    value: i32,
    kept: bool,
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

    #[allow(non_snake_case)]
    pub fn Neg(inner: EvalNode) -> Self {
        EvalNode::UnaryOp { op: Op::Neg, inner: Box::new(inner) }
    }

    pub fn value(&self) -> EvalResult<i32> {
        match self {
            EvalNode::BinaryOp { op, left, right } => {
                let l = left.value()?;
                let r = right.value()?;

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
            EvalNode::Parens { inner } => {
                Ok(inner.value()?)
            }
            EvalNode::UnaryOp { op, inner } => {
                let inner = inner.value()?;

                match op {
                    Op::Neg => {
                        Ok(-inner)
                    }
                    _ => unreachable!()
                }
            }
            EvalNode::Number(x) => {
                Ok(*x)
            }
            EvalNode::Dice { num: _, sides: _, rolls } => {
                Ok(rolls
                    .iter()
                    .map(|r| if r.kept { r.value } else { 0 })
                    .sum())
            }
            EvalNode::Set(items) => {
                Ok(items
                    .iter()
                    .map(|n| n.value().unwrap())
                    .sum())
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
            EvalNode::Parens { inner } => {
                format!("({})", inner.to_string())
            }
            EvalNode::UnaryOp { op, inner } => {
                match op {
                    Op::Neg => {
                        format!("-{}", inner.to_string())
                    },
                    _ => unreachable!(),
                }
            }
            EvalNode::Number(x) => {
                format!("{}", x)
            }
            EvalNode::Dice { num, sides, rolls } => {
                let rolls_str: String = rolls
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}d{} ({})", num, sides, rolls_str)
            }
            EvalNode::Set(items) => {
                if items.len() == 1 {
                    format!("({},)", items[0].to_string())
                } else {
                    let set_str: String = items
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("({})", set_str)
                }
            }
        }
    }
}

impl DiceRoll {
    pub fn new(value: i32, kept: bool) -> Self {
        DiceRoll { value, kept }
    }

    pub fn build(rolls: Vec<i32>) -> Vec<Self> {
        rolls
            .into_iter()
            .map(|value| DiceRoll { value, kept: true })
            .collect()
    }

    pub fn to_string(&self) -> String {
        if self.kept {
            format!("{}", self.value)
        } else {
            format!("~~{}~~", self.value)
        }
    }
}