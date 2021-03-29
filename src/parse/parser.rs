use pest::{
    prec_climber as pcl,
    prec_climber::PrecClimber,
};
use pest_consume::{
    Error,
    match_nodes,
    Parser,
};
use crate::ast::{
    InnerNode,
    Node as ASTNode,
    Op,
    SetOps,
    SetSelector,
};

pub type ParseResult<T> = Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;


#[derive(Parser)]
#[grammar = "dice.pest"]
pub struct DiceParser;


lazy_static::lazy_static! {
    static ref BINARY_PREC_CLIMBER: PrecClimber<Rule> = PrecClimber::new(
        vec![
            pcl::Operator::new(Rule::add, pcl::Assoc::Left) |
            pcl::Operator::new(Rule::sub, pcl::Assoc::Left),

            pcl::Operator::new(Rule::mul, pcl::Assoc::Left) |
            pcl::Operator::new(Rule::div, pcl::Assoc::Left),
        ]
    );
}


#[pest_consume::parser]
impl DiceParser {
    #[allow(non_snake_case)]
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn add(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn sub(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn number(input: Node) -> ParseResult<i32> {
        input
            .as_str()
            .trim()
            .parse()
            .map_err(|e| input.error(e))
    }

    fn dice_expr(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [dice(x), set_operation(ops)..] => {
                match x {
                    ASTNode::Node(InnerNode { op: Op::Dice { num, sides, ops: _ }, left: None, right: None }) => {
                        ASTNode::Dice(num, sides, Some(SetOps::build(ops.collect())))
                    }
                    _ => unreachable!(),
                }
            },
            [dice(x)] => x,
        ))
    }

    fn dice(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [number(num), number(sides)] => ASTNode::Dice(num, sides, None),
            [number(sides)] => ASTNode::Dice(1, sides, None),
        ))
    }

    fn set_expr(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [set(x), set_operation(ops)..] => {
                match x {
                    ASTNode::Set { set, ops: _ } => {
                        ASTNode::Set { set, ops: SetOps::build(ops.collect()) }
                    }
                    _ => unreachable!(),
                }
            },
            [set(x)] => x,
        ))
    }

    fn set(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [] => ASTNode::Set { set: vec![], ops: SetOps::default() },
            [expr(items)..] => {
                ASTNode::Set { set: items
                    .map(|n| Box::new(n))
                    .collect::<Vec<Box<ASTNode>>>(),
                               ops: SetOps::default()
               }},
        ))
    }

    fn set_operation(input: Node) -> ParseResult<(String, SetSelector)> {
        let sel = input.children().single().unwrap();
        let sel: SetSelector = DiceParser::set_selector(sel).unwrap();

        let id: String = input.to_string()[0..1].to_string();
        Ok((id, sel))
    }

    fn set_selector(input: Node) -> ParseResult<SetSelector> {
        let n = input.children().single().unwrap();
        let n = DiceParser::number(n).unwrap();

        Ok(match &input.as_str()[0..1] {
            "h" => SetSelector::Highest(n),
            "l" => SetSelector::Lowest(n),
            _ => SetSelector::Literal(n),
        })
    }

    #[prec_climb(unary_term, BINARY_PREC_CLIMBER)]
    fn expr(left: ASTNode, op: Node, right: ASTNode) -> ParseResult<ASTNode> {
        match op.as_rule() {
            Rule::add => Ok(ASTNode::Add(left, right)),
            Rule::sub => Ok(ASTNode::Sub(left, right)),
            Rule::mul => Ok(ASTNode::Mul(left, right)),
            Rule::div => Ok(ASTNode::Div(left, right)),
            _ => Err(op.error(format!("Rule {:?} isn't an operator", right)))?
        }
    }

    fn unary_term(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [add(_), term(x)] => x,
            [sub(_), number(x)] => ASTNode::Number(-x),
            [sub(_), term(x)] => ASTNode::Neg(x),
            [term(x)] => x,
        ))
    }

    fn term(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [number(x)] => ASTNode::Number(x),
            [dice_expr(x)] => x,
            [parens(x)] => x,
            [set_expr(x)] => x,
        ))
    }

    fn parens(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [expr(x)] => ASTNode::Parens(x),
        ))
    }

    fn calculation(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [expr(x), EOI(_)] => x,
        ))
    }
}


pub fn parse(input_str: &str) -> ParseResult<ASTNode> {
    let inputs = DiceParser::parse(Rule::calculation, input_str)?;
    let input = inputs.single()?;

    DiceParser::calculation(input)
}