use logos::Logos;
use std::mem;
use token::Token;

pub fn lex(text: &str) -> impl Iterator<Item = Token<'_>> {
    Lexer { inner: LexerTokenKind::lexer(text) }
}

struct Lexer<'a> {
    inner: logos::Lexer<'a, LexerTokenKind>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Token { text, kind: unsafe { mem::transmute(kind) } })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
#[repr(u16)]
enum LexerTokenKind {
    #[regex("[a-zA-Z_]+[a-zA-Z0-9_]*")]
    Ident,

    #[regex("[0-9]+")]
    Int,

    #[token("+")]
    Plus,

    #[token("-")]
    Hyphen,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[regex("[ \n]+")]
    Whitespace,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenKind;

    fn check(input: &str, expected_kind: TokenKind) {
        let mut tokens = lex(input);

        let token = tokens.next().unwrap();
        assert_eq!(token.kind, expected_kind);
        assert_eq!(token.text, input); // the token should span the entire input

        assert!(tokens.next().is_none()); // we should only get one token
    }

    #[test]
    fn lex_whitespace() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_lowercase_alphabetic_ident() {
        check("abc", TokenKind::Ident);
    }

    #[test]
    fn lex_uppercase_alphabetic_ident() {
        check("ABC", TokenKind::Ident);
    }

    #[test]
    fn lex_mixed_case_alphabetic_ident() {
        check("abCdEFg", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_ident() {
        check("abc123def", TokenKind::Ident);
    }

    #[test]
    fn lex_ident_with_underscores() {
        check("a_b_c", TokenKind::Ident);
    }

    #[test]
    fn lex_ident_starting_with_underscore() {
        check("__main__", TokenKind::Ident);
    }

    #[test]
    fn lex_int() {
        check("123", TokenKind::Int);
    }

    #[test]
    fn dont_lex_ident_starting_with_int() {
        let mut tokens = lex("92foo");

        assert_eq!(tokens.next(), Some(Token { text: "92", kind: TokenKind::Int }));
        assert_eq!(tokens.next(), Some(Token { text: "foo", kind: TokenKind::Ident }));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_hyphen() {
        check("-", TokenKind::Hyphen);
    }

    #[test]
    fn lex_asterisk() {
        check("*", TokenKind::Asterisk);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }
}