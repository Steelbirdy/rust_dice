pub type Child = Option<Box<Node>>;


#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Node(InnerNode),
    Set(Vec<Box<Node>>),
}


#[derive(Debug, Eq, PartialEq)]
pub struct InnerNode {
    pub op: Op,
    pub left: Child,
    pub right: Child,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Parens,
    Number(i32),
    Dice { num: i32, sides: i32 },
}


impl Node {
    fn new(op: Op, left: Option<Node>, right: Option<Node>) -> Self {
        Node::Node(InnerNode::new(op, left, right))
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
    pub fn Parens(inner: Node) -> Node {
        Node::new(Op::Parens, Some(inner), None)
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


impl InnerNode {
    fn new(op: Op, left: Option<Node>, right: Option<Node>) -> Self {
        let left = if let Some(inner) = left { Some(Box::new(inner)) } else { None };
        let right = if let Some(inner) = right { Some(Box::new(inner)) } else { None };

        InnerNode { op, left, right }
    }

    #[allow(non_snake_case)]
    pub fn Add(left: Node, right: Node) -> InnerNode {
        InnerNode::new(Op::Add, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Sub(left: Node, right: Node) -> InnerNode {
        InnerNode::new(Op::Sub, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Mul(left: Node, right: Node) -> InnerNode {
        InnerNode::new(Op::Mul, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Div(left: Node, right: Node) -> InnerNode {
        InnerNode::new(Op::Div, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Parens(inner: Node) -> InnerNode {
        InnerNode::new(Op::Parens, Some(inner), None)
    }

    #[allow(non_snake_case)]
    pub fn Number(value: i32) -> InnerNode {
        InnerNode::new(Op::Number(value), None, None)
    }

    #[allow(non_snake_case)]
    pub fn Dice(num: i32, sides: i32) -> InnerNode {
        InnerNode::new(Op::Dice { num, sides }, None, None)
    }
}