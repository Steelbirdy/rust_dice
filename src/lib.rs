mod ast;
mod expr;
mod parse;


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
}


#[cfg(test)]
mod test_expr {
    use super::ast::Node;
    use super::expr::{
        TEST_SEED,
        EvalResult,
        ExprError,
        Expression,
    };

    fn eval(head: Node) -> EvalResult<i32> {
        Expression::from_seed(head, TEST_SEED).eval()
    }

    #[test]
    fn test_eval_single_num() {
        assert_eq!( // 2
                    eval(Node::Number(2)).unwrap(), 2);
    }

    #[test]
    fn test_eval_single_num_op() {
        assert_eq!( // 1 + 1
                    eval(Node::Add(Node::Number(1), Node::Number(1))).unwrap(), 2);
    }

    #[test]
    fn test_eval_no_dice() {
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

    #[test]
    fn test_zero_division_err() {
        assert_eq!(
            eval(Node::Div(Node::Number(1), Node::Number(0))).unwrap_err(),
            ExprError::ZeroDivision,
        )
    }

    #[test]
    fn test_zero_sides_err() {
        assert_eq!(
            eval(Node::Dice(1, 0)).unwrap_err(),
            ExprError::ZeroSides,
        )
    }

    #[test]
    fn test_expr_not_seeded() {
        assert!( // 1d20 + 3
                 Expression::new(
                     Node::Add(
                         Node::Dice(1, 20),
                         Node::Number(3))).eval().is_ok()
        )
    }
}

#[cfg(test)]
mod test_parse {
    use super::ast::{
        Node,
    };

    use super::parse::{
        ParseResult,
    };


    fn parse(input_str: &str) -> ParseResult<Node> {
        super::parse::parse(input_str)
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse("1").unwrap(),
            Node::Number(1));
    }

    #[test]
    fn test_parse_dice() {
        assert_eq!( // 1d20
                    parse("1d20").unwrap(),
                    Node::Dice(1, 20));
    }

    #[test]
    fn test_parse_parens_single_num() {
        assert_eq!( // (2)
                    parse("(2)").unwrap(),
                    Node::Number(2));
    }

    #[test]
    fn test_parse_parens_single_dice() {
        assert_eq!( // (1d10)
                    parse("(1d10)").unwrap(),
                    Node::Dice(1, 10));
    }

    #[test]
    fn test_parse_implicit_dice_count_single_digit() {
        assert_eq!( // d6
                    parse("d6").unwrap(),
                    Node::Dice(1, 6));
    }

    #[test]
    fn test_parse_implicit_dice_count_multi_digit() {
        assert_eq!( // d12
                    parse("d12").unwrap(),
                    Node::Dice(1, 12));
    }

    #[test]
    fn test_parse_single_add() {
        assert_eq!( // 1 + 2d10
                    parse("1+ 2d10").unwrap(),
                    Node::Add(
                        Node::Number(1),
                        Node::Dice(2, 10)));
    }

    #[test]
    fn test_parse_single_sub() {
        assert_eq!( // 3d6 - 4
                    parse("3d6 -4").unwrap(),
                    Node::Sub(
                        Node::Dice(3, 6),
                        Node::Number(4)));
    }

    #[test]
    fn test_parse_single_mul() {
        assert_eq!( // 2 * 4d4
                    parse("2 * 4d4").unwrap(),
                    Node::Mul(
                        Node::Number(2),
                        Node::Dice(4, 4)));
    }

    #[test]
    fn test_parse_single_div() {
        assert_eq!( // 4 / 3
                    parse("4 / 3").unwrap(),
                    Node::Div(
                        Node::Number(4),
                        Node::Number(3)));
    }

    #[test]
    fn test_parse_precedence() {
        assert_eq!( // 2 * 3d12 - 5
                    parse("2*3d12-5").unwrap(),
                    Node::Sub(
                        Node::Mul(
                            Node::Number(2),
                            Node::Dice(3, 12)),
                        Node::Number(5)));
    }
}