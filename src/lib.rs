mod ast;
mod eval;
mod expr;
mod parse;


#[cfg(test)]
mod test_ast {
    use super::ast::{
        Child,
        InnerNode,
        Node,
        Op,
    };

    fn boxed_child(op: Op, left: Child, right: Child) -> Child {
        Some(Box::new(Node::Node(InnerNode { op, left, right })))
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
                    Node::Node(InnerNode {
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
                    })
        )
    }
}


#[cfg(test)]
mod test_expr {
    use super::ast::Node;
    use super::expr::{
        TEST_SEED,
        EvalNode,
        ExprResult,
        ExprError,
        Expression,
    };
    use super::test_parse::boxed;

    fn eval(head: Node) -> ExprResult<EvalNode> {
        Expression::from_seed(head, TEST_SEED).eval()
    }

    #[test]
    fn test_eval_single_num() {
        assert_eq!( // 2
                    eval(Node::Number(2)).unwrap().value().unwrap(), 2);
    }

    #[test]
    fn test_eval_single_num_op() {
        assert_eq!( // 1 + 1
                    eval(Node::Add(Node::Number(1), Node::Number(1))).unwrap().value().unwrap(), 2);
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
                        )).unwrap().value().unwrap(), 12);
    }

    #[test]
    fn test_expression_eval_with_dice() {
        assert_eq!( // 1d20 (14)
                    eval(Node::Dice(1, 20)).unwrap().value().unwrap(), 14);

        assert_eq!( // 6d6 (23)
                    eval(Node::Dice(6, 6)).unwrap().value().unwrap(), 23);

        assert_eq!( // 4d12 (27) + 4
                    eval(
                        Node::Add(
                            Node::Dice(4, 12),
                            Node::Number(4),
                        )).unwrap().value().unwrap(), 31);

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
                    ).unwrap().value().unwrap(), 21);
    }

    #[test]
    fn test_zero_sides_err() {
        assert_eq!( // 1d0
                    eval(Node::Dice(1, 0)).unwrap_err(),
                    ExprError::ZeroSides,
        )
    }

    #[test]
    fn test_expr_not_seeded() {
        assert!( // 1d20 (?) + 3
                 Expression::new(
                     Node::Add(
                         Node::Dice(1, 20),
                         Node::Number(3))).eval().is_ok()
        )
    }

    #[test]
    fn test_eval_dice_unary_minus() {
        assert_eq!( // -3d20 (30)
                    eval(Node::Neg(Node::Dice(3, 20))).unwrap().value().unwrap(),
                    -30);
    }

    #[test]
    fn test_eval_set_unary_minus() {
        assert_eq!( // -(1d6, -2)
                    eval(Node::Neg(Node::Set(boxed(vec![
                        Node::Dice(1, 6),
                        Node::Number(-2),
                    ])))).unwrap().value().unwrap(),
                    -2);
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

    pub(crate) fn boxed(input_vec: Vec<Node>) -> Vec<Box<Node>> {
        input_vec.into_iter().map(|n| Box::new(n)).collect()
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
                    Node::Parens(Node::Number(2)));
    }

    #[test]
    fn test_parse_parens_single_dice() {
        assert_eq!( // (1d10)
                    parse("(1d10)").unwrap(),
                    Node::Parens(Node::Dice(1, 10)));
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

    #[test]
    fn test_parse_empty_set() {
        assert_eq!(
            parse("()").unwrap(),
            Node::Set(vec![]));
    }

    #[test]
    fn test_parse_set_one_element() {
        assert_eq!(parse("(1,)").unwrap(),
                   Node::Set(boxed(vec![
                       Node::Number(1)
                   ])))
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(parse("(1, 2d6, 5d4, -3 + 1d8)").unwrap(),
                   Node::Set(boxed(vec![
                       Node::Number(1),
                       Node::Dice(2, 6),
                       Node::Dice(5, 4),
                       Node::Add(Node::Number(-3), Node::Dice(1, 8)),
                   ])))
    }

    #[test]
    fn test_parse_number_unary_plus() {
        assert_eq!(parse("+1").unwrap(), Node::Number(1));
    }

    #[test]
    fn test_parse_dice_unary_plus() {
        assert_eq!(parse("+2d4").unwrap(), Node::Dice(2, 4));
    }

    #[test]
    fn test_parse_set_unary_plus() {
        assert_eq!(
            parse("+(1, 1d6)").unwrap(),
            Node::Set(boxed(vec![
                Node::Number(1),
                Node::Dice(1, 6),
            ])))
    }

    #[test]
    fn test_parse_number_unary_minus() {
        assert_eq!(
            parse("-1").unwrap(),
            Node::Number(-1));
    }

    #[test]
    fn test_parse_dice_unary_minus() {
        assert_eq!(
            parse("-2d4").unwrap(),
            Node::Neg(Node::Dice(2, 4)));
    }

    #[test]
    fn test_parse_set_unary_minus() {
        assert_eq!(
            parse("-(1, 3d6, 4 - 1d4)").unwrap(),
            Node::Neg(Node::Set(boxed(vec![
                Node::Number(1),
                Node::Dice(3, 6),
                Node::Sub(Node::Number(4), Node::Dice(1, 4))
            ]))));
    }

    #[test]
    fn test_parse_unary_op_precedence() {
        assert_eq!(
            parse("-2 * -1d4 - -2d6").unwrap(),
            Node::Sub(
                Node::Mul(
                    Node::Number(-2),
                    Node::Neg(Node::Dice(1, 4))),
                Node::Neg(Node::Dice(2, 6))));
    }
}


#[cfg(test)]
mod test_eval {
    use super::eval::{
        EvalError,
        EvalNode,
    };

    #[test]
    fn test_display() {
        assert_eq!(
            EvalNode::Add(
                EvalNode::Sub(
                    EvalNode::Number(1),
                    EvalNode::Dice { num: 3, sides: 6, rolls: vec![1, 4, 2] }),
                EvalNode::Mul(
                    EvalNode::Dice { num: 1, sides: 20, rolls: vec![15] },
                    EvalNode::Div(
                        EvalNode::Number(6),
                        EvalNode::Number(2))))
                .to_string(),
            "1 - 3d6 (1, 4, 2) + 1d20 (15) * 6 / 2");
    }

    #[test]
    fn test_display_empty_set() {
        assert_eq!(EvalNode::Set(vec![]).to_string(), "()");
    }

    #[test]
    fn test_display_one_length_set() {
        assert_eq!(EvalNode::Set(vec![EvalNode::Number(3)]).to_string(), "(3,)");
    }

    #[test]
    fn test_display_set() {
        assert_eq!(EvalNode::Set(vec![
            EvalNode::Number(2),
            EvalNode::Add(
                EvalNode::Set(vec![EvalNode::Number(-1)]),
                EvalNode::Dice { num: 3, sides: 6, rolls: vec![3, 2, 1] },
            ),
            EvalNode::Dice { num: 1, sides: 20, rolls: vec![20] },
        ]).to_string(), "(2, (-1,) + 3d6 (3, 2, 1), 1d20 (20))");
    }

    #[test]
    fn test_display_unary_minus() {
        assert_eq!(EvalNode::Neg(
            EvalNode::Set(vec![
                EvalNode::Number(2),
                EvalNode::Dice { num: 1, sides: 6, rolls: vec![3] }
            ])).to_string(), "-(2, 1d6 (3))");
    }

    #[test]
    fn test_value() {
        assert_eq!(
            EvalNode::Add(
                EvalNode::Sub(
                    EvalNode::Number(1),
                    EvalNode::Dice { num: 3, sides: 6, rolls: vec![1, 4, 2] }),
                EvalNode::Mul(
                    EvalNode::Dice { num: 1, sides: 20, rolls: vec![15] },
                    EvalNode::Div(
                        EvalNode::Number(6),
                        EvalNode::Number(2))))
                .value().unwrap(),
            39);
    }

    #[test]
    fn test_empty_set_value() {
        assert_eq!(EvalNode::Set(vec![]).value().unwrap(), 0);
    }

    #[test]
    fn test_set_value() {
        assert_eq!(EvalNode::Set(vec![
            EvalNode::Number(-3),
            EvalNode::Dice { num: 1, sides: 20, rolls: vec![16] },
            EvalNode::Add(EvalNode::Number(1), EvalNode::Dice { num: 1, sides: 12, rolls: vec![6] })
        ]).value().unwrap(), 20);
    }

    #[test]
    fn test_unary_minus_value() {
        assert_eq!(EvalNode::Neg(
            EvalNode::Dice { num: 2, sides: 4, rolls: vec![2, 3] }).value().unwrap(), -5);
    }

    #[test]
    fn test_zero_division_err() {
        assert_eq!(
            EvalNode::Div(EvalNode::Number(1), EvalNode::Number(0)).value().unwrap_err(),
            EvalError::ZeroDivision,
        )
    }
}


#[cfg(test)]
mod test_pipeline {
    use super::{
        expr::{
            Expression,
            TEST_SEED,
        },
        parse::parse,
    };

    fn run(input_str: &str, expected_value: i32, expected_str: &str) {
        let result = Expression::from_seed(parse(input_str).unwrap(), TEST_SEED).eval().unwrap();
        assert_eq!(result.value().unwrap(), expected_value);
        assert_eq!(result.to_string(), expected_str);
    }

    #[test]
    fn test_single_number() {
        run("3", 3, "3");
    }

    #[test]
    fn test_single_dice() {
        run("2d4", 4, "2d4 (3, 1)");
    }

    #[test]
    fn test_add() {
        run("1d20 + 5", 19, "1d20 (14) + 5");
    }

    #[test]
    fn test_sub() {
        run("2d10 - 3", 7, "2d10 (7, 3) - 3");
    }

    #[test]
    fn test_mul() {
        run("4 * 6d6", 92, "4 * 6d6 (4, 2, 4, 5, 3, 5)");
    }

    #[test]
    fn test_div() {
        run("4 / 1d6", 1, "4 / 1d6 (4)");
    }

    #[test]
    fn test_compound_expr() {
        run("1d20 - 1d4 * 2 + 7", 19, "1d20 (14) - 1d4 (1) * 2 + 7");
    }

    #[test]
    fn test_parens() {
        run("1d20 * (2 + 1d4)", 42, "1d20 (14) * (2 + 1d4 (1))");
    }
}