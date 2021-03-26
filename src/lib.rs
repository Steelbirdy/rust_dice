pub mod ast;


#[cfg(test)]
mod test_ast {
    use super::ast::{
        Child,
        Node,
        Op,
    };

    fn boxed_child(op: Op, left: Child, right: Child) -> Child {
        Some(Box::new(Node { op, left, right }))
    }

    #[test]
    fn test_node_constructors() {
        //  3d6 / 2 * 1d4 - 1 + 1d20
        assert_eq!(
            Node::Add(
                Node::Sub(
                    Node::Mul(
                        Node::Div(
                            Node::Dice(3, 6),
                            Node::Number(2),
                        ),
                        Node::Dice(1, 4),
                    ),
                    Node::Number(1),
                ),
                Node::Dice(1, 20),
            ),
            Node {
                op: Op::Add,
                left: boxed_child(
                    Op::Sub,
                    boxed_child(
                        Op::Mul,
                        boxed_child(
                            Op::Div,
                            boxed_child(Op::Dice { num: 3, sides: 6 }, None, None),
                            boxed_child(Op::Number(2), None, None),
                        ),
                        boxed_child(Op::Dice { num: 1, sides: 4 }, None, None),
                    ),
                    boxed_child(Op::Number(1), None, None),
                ),
                right: boxed_child(Op::Dice { num: 1, sides: 20 }, None, None),
            }
        )
    }
}