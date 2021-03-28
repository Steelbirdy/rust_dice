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
    Node as ASTNode,
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

    fn number(input: Node) -> ParseResult<i32> {
        input
            .as_str()
            .trim()
            .parse()
            .map_err(|e| input.error(e))
    }

    fn dice(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [number(num), number(sides)] => ASTNode::Dice(num, sides),
            [number(sides)] => ASTNode::Dice(1, sides),
        ))
    }

    fn set(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [] => ASTNode::Set(vec![]),
            [expr(items)..] => ASTNode::Set(items.map(|n| Box::new(n)).collect::<Vec<Box<ASTNode>>>()),
        ))
    }

    #[prec_climb(term, BINARY_PREC_CLIMBER)]
    fn expr(left: ASTNode, op: Node, right: ASTNode) -> ParseResult<ASTNode> {
        match op.as_rule() {
            Rule::add => Ok(ASTNode::Add(left, right)),
            Rule::sub => Ok(ASTNode::Sub(left, right)),
            Rule::mul => Ok(ASTNode::Mul(left, right)),
            Rule::div => Ok(ASTNode::Div(left, right)),
            _ => Err(op.error(format!("Rule {:?} isn't an operator", right)))?
        }
    }

    fn term(input: Node) -> ParseResult<ASTNode> {
        Ok(match_nodes!(input.into_children();
            [number(x)] => ASTNode::Number(x),
            [dice(x)] => x,
            [parens(x)] => x,
            [set(x)] => x,
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