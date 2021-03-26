mod ast;


fn main() {
    assert_eq!(ast::Expression::new(ast::Node::Number(1)).eval().unwrap(), 1);
}