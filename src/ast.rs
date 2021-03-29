pub type Child = Option<Box<Node>>;


#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Node(InnerNode),
    Set { set: Vec<Box<Node>>, ops: SetOps },
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
    Neg,
    Parens,
    Number(i32),
    Dice { num: i32, sides: i32, ops: SetOps },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SetOps {
    pub keep: Option<SetSelector>,
    pub drop: Option<SetSelector>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetSelector {
    Literal(i32),
    Highest(i32),
    Lowest(i32),
}


impl Node {
    fn new(op: Op, left: Option<Node>, right: Option<Node>) -> Self {
        let left = if let Some(inner) = left { Some(Box::new(inner)) } else { None };
        let right = if let Some(inner) = right { Some(Box::new(inner)) } else { None };

        Node::Node(InnerNode { op, left, right })
    }

    #[allow(non_snake_case)]
    pub fn Add(left: Node, right: Node) -> Self {
        Node::new(Op::Add, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Sub(left: Node, right: Node) -> Self {
        Node::new(Op::Sub, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Mul(left: Node, right: Node) -> Self {
        Node::new(Op::Mul, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Div(left: Node, right: Node) -> Self {
        Node::new(Op::Div, Some(left), Some(right))
    }

    #[allow(non_snake_case)]
    pub fn Neg(inner: Node) -> Self {
        if let Node::Node(InnerNode { op: Op::Number(x), left: None, right: None }) = inner {
            Node::Number(-x)
        } else {
            Node::new(Op::Neg, None, Some(inner))
        }
    }

    #[allow(non_snake_case)]
    pub fn Parens(inner: Node) -> Self {
        Node::new(Op::Parens, Some(inner), None)
    }

    #[allow(non_snake_case)]
    pub fn Number(value: i32) -> Self {
        Node::new(Op::Number(value), None, None)
    }

    #[allow(non_snake_case)]
    pub fn Dice(num: i32, sides: i32, ops: Option<SetOps>) -> Self {
        let ops = ops.unwrap_or(SetOps::default());
        Node::new(Op::Dice { num, sides, ops }, None, None)
    }
}


impl Default for SetOps {
    fn default() -> Self {
        SetOps { keep: None, drop: None }
    }
}

impl SetOps {
    pub fn build(items: Vec<(String, SetSelector)>) -> Self {
        let mut ops = SetOps::default();

        for (chr, sel) in items {
            let chr = chr.as_str();
            match chr {
                "k" => {
                    ops.keep = Some(sel);
                }
                "l" => {
                    ops.drop = Some(sel);
                }
                _ => panic!("Invalid set operator `{}`", chr)
            }
        }

        let ops = ops;
        ops
    }
}