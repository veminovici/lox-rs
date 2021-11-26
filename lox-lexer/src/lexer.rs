use std::iter::Peekable;
use std::str::{Chars, FromStr};

use crate::{Span, Token};

const CHAR_NEWLINE: char = '\n';

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
        None
    }

    /// Creates a new 'eof' token while updating the context
    fn mk_eof_token(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);

        self.eof_generated = true; // mark that we reaced the end of stream
        let s1 = self.span.complete(); // complete the span
        let t = Token::new_eof(s1); // create a new token

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

        let a = ctx.read_char().unwrap();
        assert_eq!('a', a);

        let b = ctx.read_char().unwrap();
        assert_eq!('b', b);

        let c = ctx.read_char().unwrap();
        assert_eq!('c', c);

        let e = ctx.read_char();
        assert!(e.is_none());
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
