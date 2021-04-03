use logos::Logos;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[regex("[0-9]*d(%|[1-9][0-9]*)")]
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
}


impl TokenKind {
    pub const SET_OPERATORS: &'static [Self; 8] = &[
        Self::Keep,
        Self::Drop,
        Self::Reroll,
        Self::RerollOnce,
        Self::RerollAdd,
        Self::Explode,
        Self::Min,
        Self::Max,
    ];

    pub const SET_SELECTORS: &'static [Self; 5] = &[
        Self::Number,
        Self::Highest,
        Self::Lowest,
        Self::Greater,
        Self::Less,
    ];

    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::Dice => "dice",
            Self::Number => "number",
            Self::Plus => "'+'",
            Self::Minus => "'-'",
            Self::Star => "'*'",
            Self::Slash => "'/'",
            Self::Percent => "'%'",
            Self::LParen => "'('",
            Self::RParen => "')'",
            Self::Comma => "','",
            Self::Keep => "'k'",
            Self::Drop => "'p'",
            Self::Reroll => "'rr'",
            Self::RerollOnce => "'ro'",
            Self::RerollAdd => "'ra'",
            Self::Explode => "'e'",
            Self::Min => "'mi'",
            Self::Max => "'ma'",
            Self::Highest => "'h'",
            Self::Lowest => "'l'",
            Self::Greater => "'>'",
            Self::Less => "'<'",
            Self::Error => "an unrecognized token",
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        let token = lexer.next().unwrap();

        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces() {
        check("  ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_newline() {
        check("\n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_dice() {
        check("8d6", TokenKind::Dice);
    }

    #[test]
    fn lex_number() {
        check("123456", TokenKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_percent() {
        check("%", TokenKind::Percent);
    }

    #[test]
    fn lex_left_paren() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_paren() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_comma() {
        check(",", TokenKind::Comma);
    }

    #[test]
    fn lex_keep() {
        check("k", TokenKind::Keep);
    }

    #[test]
    fn lex_drop() {
        check("p", TokenKind::Drop);
    }

    #[test]
    fn lex_reroll() {
        check("rr", TokenKind::Reroll);
    }

    #[test]
    fn lex_reroll_once() {
        check("ro", TokenKind::RerollOnce);
    }

    #[test]
    fn lex_reroll_add() {
        check("ra", TokenKind::RerollAdd);
    }

    #[test]
    fn lex_explode() {
        check("e", TokenKind::Explode);
    }

    #[test]
    fn lex_min() {
        check("mi", TokenKind::Min);
    }

    #[test]
    fn lex_max() {
        check("ma", TokenKind::Max);
    }

    #[test]
    fn lex_highest() {
        check("h", TokenKind::Highest);
    }

    #[test]
    fn lex_lowest() {
        check("l", TokenKind::Lowest);
    }

    #[test]
    fn lex_greater() {
        check(">", TokenKind::Greater);
    }

    #[test]
    fn lex_less() {
        check("<", TokenKind::Less);
    }
}