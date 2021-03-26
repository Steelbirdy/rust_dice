pub type Child = Option<Box<Node>>;
pub type Result<T> = std::result::Result<T, ExprError>;


#[derive(Debug)]
pub enum ExprError {
    ZeroDivision,
    ZeroSides,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub op: Op,
    pub left: Child,
    pub right: Child,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Number(i32),
    Dice { num: i32, sides: i32 },
}

pub struct Expression {
    head: Option<Node>,
    seed: Option<u64>,
}


impl Node {
    fn new(op: Op, left: Option<Node>, right: Option<Node>) -> Self {
        let left = if let Some(inner) = left { Some(Box::new(inner)) } else { None };
        let right = if let Some(inner) = right { Some(Box::new(inner)) } else { None };

        Node { op, left, right }
    }

    #[allow(non_snake_case)]
    pub fn Add(left: Node, right: Node) -> Node {
        Node::new(Op::Add, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Sub(left: Node, right: Node) -> Node {
        Node::new(Op::Sub, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Mul(left: Node, right: Node) -> Node {
        Node::new(Op::Mul, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Div(left: Node, right: Node) -> Node {
        Node::new(Op::Div, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Number(value: i32) -> Node {
        Node::new(Op::Number(value), None, None)
    }

    #[allow(non_snake_case)]
    pub fn Dice(num: i32, sides: i32) -> Node {
        Node::new(Op::Dice { num, sides }, None, None)
    }
}


impl Expression {
    pub fn new(head: Node) -> Self {
        Expression { head: Some(head), seed: None }
    }

    pub fn from_seed(head: Node, seed: u64) -> Self {
        Expression { head: Some(head), seed: Some(seed) }
    }

    pub fn eval(&self) -> Result<i32> {
        Self::eval_recursive(&self.head.as_ref().expect("Head not initialized"))
    }

    fn eval_recursive(head: &Node) -> Result<i32> {
        let mut l: Option<i32> = None;
        let mut r: Option<i32> = None;

        if let Some(left) = &head.left {
            l = Some(Self::eval_recursive(left).unwrap());
        }

        if let Some(right) = &head.right {
            r = Some(Self::eval_recursive(right).unwrap());
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
            Op::Dice { .. } => todo!()
        }
    }
}