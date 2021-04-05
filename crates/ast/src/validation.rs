mod errors;
use errors::{ValidationError, ValidationErrorKind};


use crate::{Dice, Literal};
use syntax::SyntaxNode;
use text_size::{TextSize, TextRange};


pub fn validate(node: &SyntaxNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for node in node.descendants() {
        if let Some(dice) = Dice::cast(&node) {
            validate_dice(dice, &mut errors)
        } else if let Some(literal) = Literal::cast(&node) {
            validate_literal(literal, &mut errors)
        }
    }

    errors
}


fn validate_dice(dice: Dice, errors: &mut Vec<ValidationError>) {
    if dice.count().is_none() {
        errors.push(ValidationError {
            kind: ValidationErrorKind::NumberTooLarge,
            range: {
                let text = dice.0.first_token().unwrap();
                let start = text.text_range().start();
                let end = u32::from(start) + text.text().find('d').unwrap() as u32;

                TextRange::new(start, TextSize::from(end))
            }
        });
    }

    if dice.sides().is_none() {
        errors.push(ValidationError {
            kind: ValidationErrorKind::NumberTooLarge,
            range: {
                let text = dice.0.first_token().unwrap();
                let (start, end) = Some(text.text_range())
                    .map(|range| (range.start(), range.end()))
                    .unwrap();
                let start = u32::from(start) + 1 + text.text().find('d').unwrap() as u32;

                TextRange::new(TextSize::from(start), end)
            }
        })
    }
}


fn validate_literal(literal: Literal, errors: &mut Vec<ValidationError>) {
    if literal.parse().is_none() {
        errors.push(ValidationError {
            kind: ValidationErrorKind::NumberTooLarge,
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
            &[(ValidationErrorKind::NumberTooLarge, (0..20))],
        );
    }

    #[test]
    fn validate_ok_dice() {
        check("1d20", &[]);
    }

    #[test]
    fn validate_too_large_count_dice() {
        check(
            "99999999999999999999d20",
            &[(ValidationErrorKind::NumberTooLarge, (0..20))],
        );
    }

    #[test]
    fn validate_too_large_sides_dice() {
        check(
            "1d99999999999999999999kh2",
            &[(ValidationErrorKind::NumberTooLarge, (2..22))],
        );
    }

    #[test]
    fn validate_too_large_dice_op_num() {
        check(
            "1d100rr<99999999999999999999",
            &[(ValidationErrorKind::NumberTooLarge, (8..28))],
        );
    }
}