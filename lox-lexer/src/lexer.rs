use std::iter::Peekable;
use std::str::{Chars, FromStr};

use crate::{Span, Token};

const CHAR_NEWLINE: char = '\n';

const CHAR_LEFT_PAREN: char = '(';
const CHAR_RIGHT_PAREN: char = ')';
const CHAR_LEFT_BRACE: char = '{';
const CHAR_RIGHT_BRACE: char = '}';
const CHAR_COMMA: char = ',';
const CHAR_DOT: char = '.';
const CHAR_PLUS: char = '+';
const CHAR_MINUS: char = '-';
const CHAR_SEMICOLON: char = ';';
const CHAR_STAR: char = '*';

struct Context<'a> {
    source: Peekable<Chars<'a>>, // the source of characters
    span: Span,                  // the active span
    eof_generated: bool,         // flag indicating if the eof was generated or not
}

impl<'a> Context<'a> {
    /// Creates a new context from a source string.
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            span: Span::default(),
            eof_generated: false,
        }
    }

    /// Reads a new token from the source. the source is wrapped into a
    /// contenxt, which also can provide the span of the token.
    pub(crate) fn read(&mut self) -> Option<Token> {
        if self.eof_generated {
            None
        } else if let Some(c) = self.read_char() {
            self.read_token_with_char(c)
        } else {
            self.mk_eof_token()
        }
    }

    /// Updates the span once the character is read.
    /// If we have a regular character, only the column is incremented.
    /// If the character is a new line, then we increment the line.
    fn update_span(&mut self, c: char) {
        self.span.incr_col();
        if c == CHAR_NEWLINE {
            self.span.incr_line();
        }
    }

    /// Consumes a character from the source stream.
    fn read_char(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            self.update_span(c);
            Some(c)
        } else {
            None
        }
    }

    fn read_token_with_char(&mut self, c: char) -> Option<Token> {
        match c {
            CHAR_LEFT_PAREN => self.mk_left_parenthesis_token(),
            CHAR_RIGHT_PAREN => self.mk_right_parenthesis_token(),
            CHAR_LEFT_BRACE => self.mk_left_brace_token(),
            CHAR_RIGHT_BRACE => self.mk_right_brace_token(),
            CHAR_COMMA => self.mk_comma_token(),
            CHAR_DOT => self.mk_dot_token(),
            CHAR_PLUS => self.mk_plus_token(),
            CHAR_MINUS => self.mk_minus_token(),
            CHAR_SEMICOLON => self.mk_semicolon_token(),
            CHAR_STAR => self.mk_star_token(),
            _ => None,
        }
    }

    /// Creates a 'left parenthesis' token.
    fn mk_left_parenthesis_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_left_parenthesis(s);

        Some(t)
    }

    /// Creates a 'right parenthesis' token.
    fn mk_right_parenthesis_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_right_parenthesis(s);

        Some(t)
    }

    /// Creates a 'left brace' token.
    fn mk_left_brace_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_left_brace(s);

        Some(t)
    }

    /// Creates a 'right brace' token.
    fn mk_right_brace_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_right_brace(s);

        Some(t)
    }

    /// Creates a 'comma' token.
    fn mk_comma_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_comma(s);

        Some(t)
    }

    /// Creates a 'dot' token.
    fn mk_dot_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_dot(s);

        Some(t)
    }

    /// Creates a 'plus' token.
    fn mk_plus_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_plus(s);

        Some(t)
    }

    /// Creates a 'minus' token.
    fn mk_minus_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_minus(s);

        Some(t)
    }

    /// Creates a 'semicolon' token.
    fn mk_semicolon_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_semicolon(s);

        Some(t)
    }

    /// Creates a 'star' token.
    fn mk_star_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_star(s);

        Some(t)
    }

    /// Creates a new 'eof' token while updating the context
    fn mk_eof_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);

        self.eof_generated = true; // mark that we reaced the end of stream
        let s = self.span.complete(); // complete the span
        let t = Token::new_eof(s); // create a new token

        Some(t)
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexeme;

    use super::*;

    #[test]
    fn test_read_char() {
        let mut ctx = Context::new("abc");
        ctx.span.check_span_len(1);

        let a = ctx.read_char().unwrap();
        assert_eq!('a', a);
        ctx.span.check_span_len(2);

        let b = ctx.read_char().unwrap();
        assert_eq!('b', b);
        ctx.span.check_span_len(3);

        let c = ctx.read_char().unwrap();
        assert_eq!('c', c);
        ctx.span.check_span_len(4);

        let e = ctx.read_char();
        ctx.span.check_span_len(4);
        assert!(e.is_none());
    }

    #[test]
    fn test_mk_left_parenthesis() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_left_parenthesis_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::LeftParen, tkn.lexeme);
    }

    #[test]
    fn test_mk_right_parenthesis() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_right_parenthesis_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::RightParen, tkn.lexeme);
    }

    #[test]
    fn test_mk_left_brace() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_left_brace_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::LeftBrace, tkn.lexeme);
    }

    #[test]
    fn test_mk_right_brace() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_right_brace_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::RightBrace, tkn.lexeme);
    }

    #[test]
    fn test_mk_comma() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_comma_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Comma, tkn.lexeme);
    }

    #[test]
    fn test_mk_dot() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_dot_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Dot, tkn.lexeme);
    }

    #[test]
    fn test_mk_plus() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_plus_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Plus, tkn.lexeme);
    }

    #[test]
    fn test_mk_minus() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_minus_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Minus, tkn.lexeme);
    }

    #[test]
    fn test_mk_semicolon() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_semicolon_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Semicolon, tkn.lexeme);
    }

    #[test]
    fn test_mk_star() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_star_token().unwrap();

        assert!(!ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Star, tkn.lexeme);
    }

    #[test]
    fn test_mk_eof_token() {
        let mut ctx = Context::new("abc");
        let tkn = ctx.mk_eof_token().unwrap();

        assert!(ctx.eof_generated);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Eof, tkn.lexeme);
    }
}
