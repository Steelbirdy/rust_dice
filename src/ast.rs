pub type Child = Option<Box<Node>>;


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
}