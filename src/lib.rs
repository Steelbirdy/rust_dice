pub mod ast;


#[cfg(test)]
mod test_ast {
    use super::ast::{
        TEST_SEED,
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
        assert_eq!( //  3d6 / 2 * 1d4 - 1 + 1d20
            Node::Add(
                Node::Sub(
                    Node::Mul(
                        Node::Div(
                            Node::Dice(3, 6),
                            Node::Number(2)),
                        Node::Dice(1, 4)),
                    Node::Number(1)),
                Node::Dice(1, 20)),
            Node {
                op: Op::Add,
                left: boxed_child(
                    Op::Sub,
                    boxed_child(
                        Op::Mul,
                        boxed_child(
                            Op::Div,
                            boxed_child(Op::Dice { num: 3, sides: 6 }, None, None),
                            boxed_child(Op::Number(2), None, None)),
                        boxed_child(Op::Dice { num: 1, sides: 4 }, None, None)),
                    boxed_child(Op::Number(1), None, None)),
                right: boxed_child(Op::Dice { num: 1, sides: 20 }, None, None),
            }
        )
    }

    #[test]
    fn test_expression_eval_no_dice() {
        assert_eq!( // 2
                    eval(Node::Number(2)).unwrap(), 2);

        assert_eq!( // 1 + 1
                    eval(Node::Add(Node::Number(1), Node::Number(1))).unwrap(), 2);

        assert_eq!( // 4 / 2 * 3 - (-5) + 1 = 12
                    eval(
                        Node::Add(
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
                        )).unwrap(), 12);
    }

    #[test]
    fn test_expression_eval_with_dice() {
        assert_eq!( // 1d20 (14)
                    eval(Node::Dice(1, 20)).unwrap(), 14);

        assert_eq!( // 6d6 (23)
                    eval(Node::Dice(6, 6)).unwrap(), 23);

        assert_eq!( // 4d12 (27) + 4
                    eval(
                        Node::Add(
                            Node::Dice(4, 12),
                            Node::Number(4),
                        )).unwrap(), 31);

        assert_eq!( // 2d10 (10) + 2 * 3d4 (8) - 5
                    eval(
                        Node::Add(
                            Node::Dice(2, 10),
                            Node::Sub(
                                Node::Mul(
                                    Node::Number(2),
                                    Node::Dice(3, 4),
                                ),
                                Node::Number(5),
                            ),
                        )
                    ).unwrap(), 21);
    }
}