use std::fmt;
use syntax::SyntaxKind;
use text_size::TextRange;


#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();

        for (idx, expected_kind) in self.expected.iter().enumerate() {
            if idx == 0 {
                write!(f, "{}", expected_kind)?;
            } else if idx == num_expected - 1 {
                if num_expected == 2 {
                    write!(f, " or {}", expected_kind)?;
                } else {
                    write!(f, ", or {}", expected_kind)?;
                }
            } else {
                write!(f, ", {}", expected_kind)?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {}", found)?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;

    fn check(expected: Vec<SyntaxKind>, found: Option<SyntaxKind>,
             range: StdRange<u32>, output: &str)
    {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![SyntaxKind::Number],
            Some(SyntaxKind::Slash),
            13..14,
            "error at 13..14: expected number, but found '/'",
        );
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![SyntaxKind::RParen],
            None,
            5..6,
            "error at 5..6: expected ')'",
        );
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                SyntaxKind::Number,
                SyntaxKind::Dice,
                SyntaxKind::Minus,
                SyntaxKind::LParen,
            ],
            Some(SyntaxKind::Reroll),
            100..102,
            "error at 100..102: expected number, dice, '-', or '(', but found 'rr'",
        );
    }

    #[test]
    fn two_expected_did_find() {
        check(
            vec![SyntaxKind::Plus, SyntaxKind::Minus],
            Some(SyntaxKind::Greater),
            0..1,
            "error at 0..1: expected '+' or '-', but found '>'",
        );
    }
}