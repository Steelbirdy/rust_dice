pub mod ast;

pub(crate) const TEST_SEED: u64 = 10353;


#[cfg(test)]
mod test_ast {
    use super::{
        TEST_SEED
    };
    use super::ast::{
        Child,
        Expression,
        Node,
        Op,
        Result,
    };

    fn boxed_child(op: Op, left: Child, right: Child) -> Child {
        Some(Box::new(Node { op, left, right }))
    }

    fn eval(head: Node) -> Result<i32> {
        Expression::from_seed(head, TEST_SEED).eval()
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

    #[test]
    fn test_expression_eval_no_dice() {
        assert_eq!(
            eval(Node::Number(2)).unwrap(),
            2
        );

        assert_eq!(
            eval(Node::Add(Node::Number(1), Node::Number(1))).unwrap(),
            2
        );

        assert_eq!(
            // 4 / 2 * 3 - (-5) + 1 = 12
            eval(Node::Add(
                Node::Sub(
                    Node::Mul(
                        Node::Div(
                            Node::Number(4),
                            Node::Number(2),
                        ),
                        Node::Number(3),
                    ),
                    Node::Number(-5),
                ),
                Node::Number(1),
            )).unwrap(),
            12
        )
    }
}