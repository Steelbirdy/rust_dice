mod lexer;
mod parser;
mod syntax;

#[cfg(test)]
mod tests {

    #[test]
    fn test_sanity() {
        assert_eq!(2 + 2, 4);
    }
}