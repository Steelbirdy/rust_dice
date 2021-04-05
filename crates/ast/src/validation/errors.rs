use std::fmt;
use text_size::TextRange;


#[derive(Debug, PartialEq)]
pub struct ValidationError {
    pub kind: ValidationErrorKind,
    pub range: TextRange,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error at {}..{}: {}",
               u32::from(self.range.start()),
               u32::from(self.range.end()),
               self.kind,
        )
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationErrorKind {
    NumberTooLarge,
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberTooLarge => write!(f,
                                           "number is larger than an integer's maximum value, {}", u64::MAX),
        }
    }
}