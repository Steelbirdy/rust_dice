use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};


pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}


#[derive(Debug, PartialEq)]
pub(crate) struct Token<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}


#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("d")]
    Dice,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(",")]
    Comma,

    #[token("k")]
    Keep,

    #[token("p")]
    Drop,

    #[token("rr")]
    Reroll,

    #[token("ro")]
    RerollOnce,

    #[token("ra")]
    RerollAdd,

    #[token("e")]
    Explode,

    #[token("mi")]
    Min,

    #[token("ma")]
    Max,

    #[token("h")]
    Highest,

    #[token("l")]
    Lowest,

    #[token(">")]
    Greater,

    #[token("<")]
    Less,

    #[error]
    Error,

    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token { kind, text: input }));
    }

    #[test]
    fn lex_spaces() {
        check("  ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_dice() {
        check("d", SyntaxKind::Dice);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_percent() {
        check("%", SyntaxKind::Percent);
    }

    #[test]
    fn lex_left_paren() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_paren() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_comma() {
        check(",", SyntaxKind::Comma);
    }

    #[test]
    fn lex_keep() {
        check("k", SyntaxKind::Keep);
    }

    #[test]
    fn lex_drop() {
        check("p", SyntaxKind::Drop);
    }

    #[test]
    fn lex_reroll() {
        check("rr", SyntaxKind::Reroll);
    }

    #[test]
    fn lex_reroll_once() {
        check("ro", SyntaxKind::RerollOnce);
    }

    #[test]
    fn lex_reroll_add() {
        check("ra", SyntaxKind::RerollAdd);
    }

    #[test]
    fn lex_explode() {
        check("e", SyntaxKind::Explode);
    }

    #[test]
    fn lex_min() {
        check("mi", SyntaxKind::Min);
    }

    #[test]
    fn lex_max() {
        check("ma", SyntaxKind::Max);
    }

    #[test]
    fn lex_highest() {
        check("h", SyntaxKind::Highest);
    }

    #[test]
    fn lex_lowest() {
        check("l", SyntaxKind::Lowest);
    }

    #[test]
    fn lex_greater() {
        check(">", SyntaxKind::Greater);
    }

    #[test]
    fn lex_less() {
        check("<", SyntaxKind::Less);
    }
}