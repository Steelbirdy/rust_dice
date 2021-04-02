use super::Parser;
use crate::lexer::SyntaxKind;
use std::cmp::min;


enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}


pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Number) => p.bump(),
        _ => {}
    }

    let op = match p.peek() {
        Some(SyntaxKind::Plus) => Op::Add,
        Some(SyntaxKind::Minus) => Op::Sub,
        Some(SyntaxKind::Star) => Op::Mul,
        Some(SyntaxKind::Slash) => Op::Div,
        _ => return,  // TODO: handle errors
    };

    let (left_binding_power, right_binding_power) = op.binding_power();

    if left_binding_power < minimum_binding_power {
        return;
    }

    // Eat the operator's token.
    p.bump();

    p.start_node_at(checkpoint, SyntaxKind::BinOp);
    expr_binding_power(p, right_binding_power);
    p.finish_node();
}


#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }
}