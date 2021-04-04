mod errors;
use errors::{ValidationError, ValidationErrorKind};


use crate::Literal;
use syntax::SyntaxNode;


pub fn validate(node: &SyntaxNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for node in node.descendants() {
        if let Some(literal) = Literal::cast(node) {
            validate_literal(literal, &mut errors)
        }
    }

    errors
}


fn validate_literal(literal: Literal, errors: &mut Vec<ValidationError>) {
    if literal.parse().is_none() {
        errors.push(ValidationError {
            kind: ValidationErrorKind::NumberLiteralTooLarge,
            range: literal.0.first_token().unwrap().text_range(),
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;
    use text_size::TextRange;

    fn check(input: &str, expected_errors: &[(ValidationErrorKind, StdRange<u32>)]) {
        let parse = parser::parse(input);

        let expected_errors: Vec<_> = expected_errors
            .iter()
            .map(|(kind, range)| ValidationError {
                kind: *kind,
                range: {
                    let start = range.start.into();
                    let end = range.end.into();
                    TextRange::new(start, end)
                },
            })
            .collect();

        assert_eq!(validate(&parse.syntax()), expected_errors);
    }

    #[test]
    fn validate_ok_literal() {
        check("123", &[]);
    }

    #[test]
    fn validate_too_large_literal() {
        check(
            "99999999999999999999",
            &[(ValidationErrorKind::NumberLiteralTooLarge, (0..20))],
        );
    }
}