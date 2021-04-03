use logos::Logos;


#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex(" +")]
    Whitespace,

    #[regex("[0-9]*d[1-9][0-9]*")]
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lexer, Token};

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token { kind, text: input }));
    }

    #[test]
    fn lex_spaces() {
        check("  ", TokenKind::Whitespace);
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