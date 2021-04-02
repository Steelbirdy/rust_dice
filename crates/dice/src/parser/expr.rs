use super::Parser;
use crate::lexer::SyntaxKind;


pub(super) fn expr(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Number) => p.bump(),
        _ => {}
    }
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