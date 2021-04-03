pub mod parser {
    pub use parser::parse;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_sanity() {
        assert_eq!(2 + 2, 4);
    }
}