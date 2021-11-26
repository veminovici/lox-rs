use std::iter::Peekable;
use std::str::{Chars, FromStr};

use crate::{Lexeme, Span, Token};

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
const CHAR_BANG: char = '!';
const CHAR_EQUAL: char = '=';
const CHAR_GREATER: char = '>';
const CHAR_LESS: char = '<';
const CHAR_SLASH: char = '/';

const CHAR_WHITESPACE: char = ' ';
const CHAR_CARRIAGE_RETURN: char = '\r';
const CHAR_TAB: char = '\t';

const CHAR_DOUBLE_QUOTE: char = '"';

const CHAR_0: char = '0';
const CHAR_9: char = '9';

const CHAR_LOWERCASE_A: char = 'a';
const CHAR_LOWERCASE_Z: char = 'z';
const CHAR_UPPERCASE_A: char = 'A';
const CHAR_UPPERCASE_Z: char = 'Z';
const CHAR_UNDERSCORE: char = '_';

static KEYWORDS: &[(&str, Lexeme)] = &[
    ("and", Lexeme::And),
    ("class", Lexeme::Class),
    ("else", Lexeme::Else),
    ("false", Lexeme::False),
    ("for", Lexeme::For),
    ("fun", Lexeme::Fun),
    ("if", Lexeme::If),
    ("nil", Lexeme::Nil),
    ("or", Lexeme::Or),
    ("print", Lexeme::Print),
    ("return", Lexeme::Return),
    ("super", Lexeme::Super),
    ("this", Lexeme::This),
    ("true", Lexeme::True),
    ("var", Lexeme::Var),
    ("while", Lexeme::While),
];

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

    #[inline]
    fn is_whitespace(c: char) -> bool {
        c == CHAR_WHITESPACE || c == CHAR_TAB || c == CHAR_CARRIAGE_RETURN
    }

    #[inline]
    fn is_digit(c: char) -> bool {
        c >= CHAR_0 && c <= CHAR_9
    }

    #[inline]
    fn is_alpha(c: char) -> bool {
        c >= CHAR_LOWERCASE_A && c <= CHAR_LOWERCASE_Z
            || c >= CHAR_UPPERCASE_A && c <= CHAR_UPPERCASE_Z
            || c == CHAR_UNDERSCORE
    }

    #[inline]
    fn is_alphanum(c: char) -> bool {
        Context::is_alpha(c) || Context::is_digit(c)
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

    /// Consumes a character only if it is equal with a given char.
    fn read_char_if(&mut self, c: char) -> bool {
        if let Some(c1) = self.source.peek() {
            if c == *c1 {
                let _ = self.read_char();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Read until the end of the line
    fn read_line(&mut self) -> String {
        let mut buffer = String::new();

        for c in &mut self.source {
            self.span.incr_col();
            buffer.push(c);

            if c == CHAR_NEWLINE {
                self.span.incr_line();
                break;
            }
        }

        buffer
    }

    /// Reads the sequence of whitespaces.
    fn read_ws(&mut self, first_ws: char) -> String {
        let mut buffer = format!("{}", first_ws);

        while let Some(maybe_ws) = self.source.peek().copied() {
            if Context::is_whitespace(maybe_ws) {
                buffer.push(maybe_ws);
                self.read_char();
            } else {
                break;
            }
        }

        buffer
    }

    /// Reads a string separated by the quotes.
    fn read_string(&mut self) -> Option<String> {
        let mut buffer = String::new();
        let mut string_terminated = false;

        for c in &mut self.source {
            self.span.incr_col();
            if c == CHAR_NEWLINE {
                self.span.incr_line();
            }

            if c == CHAR_DOUBLE_QUOTE {
                string_terminated = true;
                break;
            }

            buffer.push(c);
        }

        if string_terminated {
            Some(buffer)
        } else {
            None
        }
    }

    /// Reads a number in float format.
    fn read_number(&mut self, first_digit: char) -> Option<f64> {
        let mut buffer = format!("{}", first_digit);

        // Read leading digits
        while let Some(maybe_digit) = self.source.peek().copied() {
            if Context::is_digit(maybe_digit) {
                buffer.push(maybe_digit);
                self.read_char();
            } else {
                break;
            }
        }

        // Try reading "." and the rest of the digits
        if let Some(maybe_dot) = self.source.peek().copied() {
            if maybe_dot == CHAR_DOT {
                buffer.push(maybe_dot);
                self.read_char();

                let mut read_additional_digits = false;

                while let Some(maybe_digit) = self.source.peek().copied() {
                    if Context::is_digit(maybe_digit) {
                        buffer.push(maybe_digit);
                        self.read_char();
                        read_additional_digits = true;
                    } else {
                        break;
                    }
                }

                // Lox does not support leading or trailing dot in
                // number literals. This is not a valid number
                // literal, if we encountered no digits after ".".
                // Also note: we have to error here, because we
                // already consumed at least the "." from the input
                // and would have to "return" it if we didn't match
                // something. Fortunately there is nothing in Lox yet
                // that would require us to recover (e.g. methods on
                // numbers -> "4.sqrt()")
                if !read_additional_digits {
                    return None;
                }
            }
        }

        if let Ok(number) = f64::from_str(&buffer) {
            Some(number)
        } else {
            None
        }
    }

    /// Reads an identifier
    fn read_identifier(&mut self, first_alpha: char) -> String {
        let mut buffer = format!("{}", first_alpha);

        while let Some(maybe_alphanumeric) = self.source.peek() {
            if Context::is_alphanum(*maybe_alphanumeric) {
                buffer.push(*maybe_alphanumeric);
                self.read_char();
            } else {
                break;
            }
        }

        buffer
    }

    /// Reads a token which starts with a given character.
    fn read_token_with_char(&mut self, c: char) -> Option<Token> {
        match c {
            CHAR_LEFT_PAREN => self.mk_left_parenthesis(),
            CHAR_RIGHT_PAREN => self.mk_right_parenthesis(),
            CHAR_LEFT_BRACE => self.mk_left_brace(),
            CHAR_RIGHT_BRACE => self.mk_right_brace(),
            CHAR_COMMA => self.mk_comma(),
            CHAR_DOT => self.mk_dot(),
            CHAR_PLUS => self.mk_plus(),
            CHAR_MINUS => self.mk_minus(),
            CHAR_SEMICOLON => self.mk_semicolon(),
            CHAR_STAR => self.mk_star(),
            CHAR_BANG => self.mk_bang_or_bang_equal(),
            CHAR_EQUAL => self.mk_equal_or_equal_equal(),
            CHAR_GREATER => self.mk_greater_or_greater_equal(),
            CHAR_LESS => self.mk_less_or_less_equal(),
            CHAR_SLASH => self.mk_slash_or_comment(),
            CHAR_NEWLINE => self.mk_newline(),
            CHAR_DOUBLE_QUOTE => self.mk_string(),
            ws if Context::is_whitespace(ws) => self.mk_whitespace(ws),
            d if Context::is_digit(d) => self.mk_number(d),
            a if Context::is_alpha(a) => self.mk_identifier_or_keyword(a),
            unexpected => panic!("Unknown char {}", unexpected),
        }
    }

    /// Creates a 'left parenthesis' token.
    fn mk_left_parenthesis(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();

        debug_assert!(s.is_one_char());
        let t = Token::new_left_parenthesis(s);

        Some(t)
    }

    /// Creates a 'right parenthesis' token.
    fn mk_right_parenthesis(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_right_parenthesis(s);

        Some(t)
    }

    /// Creates a 'left brace' token.
    fn mk_left_brace(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_left_brace(s);

        Some(t)
    }

    /// Creates a 'right brace' token.
    fn mk_right_brace(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_right_brace(s);

        Some(t)
    }

    /// Creates a 'comma' token.
    fn mk_comma(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_comma(s);

        Some(t)
    }

    /// Creates a 'dot' token.
    fn mk_dot(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_dot(s);

        Some(t)
    }

    /// Creates a 'plus' token.
    fn mk_plus(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_plus(s);

        Some(t)
    }

    /// Creates a 'minus' token.
    fn mk_minus(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_minus(s);

        Some(t)
    }

    /// Creates a 'semicolon' token.
    fn mk_semicolon(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_semicolon(s);

        Some(t)
    }

    /// Creates a 'star' token.
    fn mk_star(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_star(s);

        Some(t)
    }

    /// Creates a 'bang' or 'bang-equal' token.
    fn mk_bang_or_bang_equal(&mut self) -> Option<Token> {
        if self.read_char_if(CHAR_EQUAL) {
            self.mk_bang_equal()
        } else {
            self.mk_bang()
        }
    }

    /// Creates a 'bang' token.
    fn mk_bang(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_bang(s);

        Some(t)
    }

    /// Creates a 'bang-equal' token.
    fn mk_bang_equal(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_two_chars());

        let s = self.span.complete();
        let t = Token::new_bang_equal(s);

        Some(t)
    }

    /// Creates a 'equal' or 'equal-equal' token.
    fn mk_equal_or_equal_equal(&mut self) -> Option<Token> {
        if self.read_char_if(CHAR_EQUAL) {
            self.mk_equal_equal()
        } else {
            self.mk_equal()
        }
    }

    /// Creates a 'equal' token.
    fn mk_equal(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_equal(s);

        Some(t)
    }

    /// Creates a 'equal-equal' token.
    fn mk_equal_equal(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_two_chars());

        let s = self.span.complete();
        let t = Token::new_equal_equal(s);

        Some(t)
    }

    /// Creates a 'greater' or 'greater-equal' token.
    fn mk_greater_or_greater_equal(&mut self) -> Option<Token> {
        if self.read_char_if(CHAR_EQUAL) {
            self.mk_greater_equal()
        } else {
            self.mk_greater()
        }
    }

    /// Creates a 'greater' token.
    fn mk_greater(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_greater(s);

        Some(t)
    }

    /// Creates a 'greater-equal' token.
    fn mk_greater_equal(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_two_chars());

        let s = self.span.complete();
        let t = Token::new_greater_equal(s);

        Some(t)
    }

    /// Creates a 'less' or 'less-equal' token.
    fn mk_less_or_less_equal(&mut self) -> Option<Token> {
        if self.read_char_if(CHAR_EQUAL) {
            self.mk_less_equal()
        } else {
            self.mk_less()
        }
    }

    /// Creates a 'less' token.
    fn mk_less(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_less(s);

        Some(t)
    }

    /// Creates a 'less-equal' token.
    fn mk_less_equal(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_two_chars());

        let s = self.span.complete();
        let t = Token::new_less_equal(s);

        Some(t)
    }

    /// Creates a 'slash' or 'comment' token.
    fn mk_slash_or_comment(&mut self) -> Option<Token> {
        if self.read_char_if(CHAR_SLASH) {
            self.mk_comment()
        } else {
            self.mk_slash()
        }
    }

    /// Creates a 'slash' token.
    fn mk_slash(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let s = self.span.complete();
        let t = Token::new_slash(s);

        Some(t)
    }

    /// Creates a 'comment' token.
    fn mk_comment(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_two_chars());

        let comment = self.read_line();

        let s = self.span.complete();
        let t = Token::new_comment(&comment, s);

        Some(t)
    }

    /// Creates a 'newline' token.
    fn mk_newline(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_multi_line());

        let s = self.span.complete();
        let t = Token::new_newline(s);

        Some(t)
    }

    /// Creates a 'whitespace' token
    fn mk_whitespace(&mut self, first_char: char) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let ws = self.read_ws(first_char);

        let s = self.span.complete();
        let t = Token::new_whitespace(&ws, s);

        Some(t)
    }

    /// Creates a 'string' token
    fn mk_string(&mut self) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let string = self.read_string().unwrap();

        let s = self.span.complete();
        let t = Token::new_string(&string, s);

        Some(t)
    }

    /// Creates a 'number' token
    fn mk_number(&mut self, first_digit: char) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let number = self.read_number(first_digit).unwrap();

        let s = self.span.complete();
        let t = Token::new_number(number, s);

        Some(t)
    }

    /// Creates a 'identifier' token
    fn mk_identifier_or_keyword(&mut self, first_char: char) -> Option<Token> {
        debug_assert!(!self.eof_generated);
        debug_assert!(self.span.is_one_char());

        let i = self.read_identifier(first_char);

        let s = self.span.complete();

        let srch = KEYWORDS.binary_search_by_key(&&*i, |&(k, _)| k);

        let token = match srch {
            Ok(index) => Token::new(KEYWORDS[index].1.clone(), s),
            Err(_) => Token::new_identifier(&i, s),
        };

        Some(token)
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

    fn read_and_ignore(ctx: &mut Context) {
        let _ = ctx.read_char().unwrap();
        ctx.span.complete();
    }

    #[test]
    fn test_read_char() {
        let mut ctx = Context::new("abc");
        ctx.span.check_span_len(0);

        let a = ctx.read_char().unwrap();
        assert_eq!('a', a);
        ctx.span.check_span_len(1);

        let b = ctx.read_char().unwrap();
        assert_eq!('b', b);
        ctx.span.check_span_len(2);

        let c = ctx.read_char().unwrap();
        assert_eq!('c', c);
        ctx.span.check_span_len(3);

        let e = ctx.read_char();
        ctx.span.check_span_len(3);
        assert!(e.is_none());
    }

    #[test]
    fn test_read_left_parenthesis() {
        let source = format!("_{}_", CHAR_LEFT_PAREN);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_LEFT_PAREN, c);

        let tkn = ctx.mk_left_parenthesis().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::LeftParen, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_right_parenthesis() {
        let source = format!("_{}_", CHAR_RIGHT_PAREN);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_RIGHT_PAREN, c);

        let tkn = ctx.mk_right_parenthesis().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::RightParen, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_left_brace() {
        let source = format!("_{}_", CHAR_LEFT_BRACE);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_LEFT_BRACE, c);

        let tkn = ctx.mk_left_brace().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::LeftBrace, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_right_brace() {
        let source = format!("_{}_", CHAR_RIGHT_BRACE);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_RIGHT_BRACE, c);

        let tkn = ctx.mk_right_brace().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::RightBrace, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_comma() {
        let source = format!("_{}_", CHAR_COMMA);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_COMMA, c);

        let tkn = ctx.mk_comma().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Comma, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_dot() {
        let source = format!("_{}_", CHAR_DOT);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_DOT, c);

        let tkn = ctx.mk_dot().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Dot, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_plus() {
        let source = format!("_{}_", CHAR_PLUS);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_PLUS, c);

        let tkn = ctx.mk_plus().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Plus, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_minus() {
        let source = format!("_{}_", CHAR_MINUS);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_MINUS, c);

        let tkn = ctx.mk_minus().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Minus, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_semicolon() {
        let source = format!("_{}_", CHAR_SEMICOLON);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_SEMICOLON, c);

        let tkn = ctx.mk_semicolon().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Semicolon, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_star() {
        let source = format!("_{}_", CHAR_STAR);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_STAR, c);

        let tkn = ctx.mk_star().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Star, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_bang() {
        let source = format!("_{}_", CHAR_BANG);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_BANG, c);

        let tkn = ctx.mk_bang_or_bang_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Bang, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_bang_equal() {
        let source = format!("_{}{}_", CHAR_BANG, CHAR_EQUAL);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_BANG, c);

        let tkn = ctx.mk_bang_or_bang_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_two_chars());
        assert_eq!(Lexeme::BangEqual, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_equal() {
        let source = format!("_{}_", CHAR_EQUAL);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_EQUAL, c);

        let tkn = ctx.mk_equal_or_equal_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Equal, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_equal_equal() {
        let source = format!("_{}{}_", CHAR_EQUAL, CHAR_EQUAL);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_EQUAL, c);

        let tkn = ctx.mk_equal_or_equal_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_two_chars());
        assert_eq!(Lexeme::EqualEqual, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_greater() {
        let source = format!("_{}_", CHAR_GREATER);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_GREATER, c);

        let tkn = ctx.mk_greater_or_greater_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Greater, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_greater_equal() {
        let source = format!("_{}{}_", CHAR_GREATER, CHAR_EQUAL);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_GREATER, c);

        let tkn = ctx.mk_greater_or_greater_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert_eq!(Lexeme::GreaterEqual, tkn.lexeme);
        assert!(tkn.span.is_two_chars());

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_less() {
        let source = format!("_{}_", CHAR_LESS);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_LESS, c);

        let tkn = ctx.mk_less_or_less_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Less, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_less_equal() {
        let source = format!("_{}{}_", CHAR_LESS, CHAR_EQUAL);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_LESS, c);

        let tkn = ctx.mk_less_or_less_equal().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert_eq!(Lexeme::LessEqual, tkn.lexeme);
        assert!(tkn.span.is_two_chars());

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_slash() {
        let source = format!("_{}_", CHAR_SLASH);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_SLASH, c);

        let tkn = ctx.mk_slash_or_comment().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_char());
        assert_eq!(Lexeme::Slash, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_comment() {
        let source = format!("_{}{}_", CHAR_SLASH, CHAR_SLASH);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_SLASH, c);

        let tkn = ctx.mk_slash_or_comment().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        let cmnt = "_".to_string();
        assert_eq!(Lexeme::Comment(cmnt), tkn.lexeme);

        // Read the _ character
        let c = ctx.read_char();
        assert!(c.is_none());
    }

    #[test]
    fn test_read_newline() {
        let source = format!("_{}_", CHAR_NEWLINE);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_NEWLINE, c);

        let tkn = ctx.mk_newline().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_multi_line());
        assert_eq!(Lexeme::NewLine, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_whitespace() {
        let source = format!("_{}{}{}_", CHAR_WHITESPACE, CHAR_TAB, CHAR_CARRIAGE_RETURN);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_WHITESPACE, c);

        let tkn = ctx.mk_whitespace(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(
            Lexeme::Whitespace(format!(
                "{}{}{}",
                CHAR_WHITESPACE, CHAR_TAB, CHAR_CARRIAGE_RETURN
            )),
            tkn.lexeme
        );

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_string() {
        let source = format!("_{}test{}_", CHAR_DOUBLE_QUOTE, CHAR_DOUBLE_QUOTE);
        let mut ctx = Context::new(source.as_str());

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert_eq!(CHAR_DOUBLE_QUOTE, c);

        let tkn = ctx.mk_string().unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::String("test".to_string()), tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_number() {
        let source = "_12.3_";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_digit(c));

        let tkn = ctx.mk_number(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Number(12.3), tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_and() {
        let source = ".and.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::And, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_class() {
        let source = ".class.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Class, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_else() {
        let source = ".else.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Else, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_false() {
        let source = ".false.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::False, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_for() {
        let source = ".for.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::For, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_fun() {
        let source = ".fun.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Fun, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_if() {
        let source = ".if.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::If, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_nil() {
        let source = ".nil.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Nil, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_or() {
        let source = ".or.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Or, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_print() {
        let source = ".print.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Print, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_return() {
        let source = ".return.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Return, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_super() {
        let source = ".super.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Super, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_this() {
        let source = ".this.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::This, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_true() {
        let source = ".true.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::True, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_var() {
        let source = ".var.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Var, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_while() {
        let source = ".while.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::While, tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }

    #[test]
    fn test_read_identifier() {
        let source = ".abc.";
        let mut ctx = Context::new(source);

        read_and_ignore(&mut ctx);

        let c = ctx.read_char().unwrap();
        assert!(Context::is_alpha(c));

        let tkn = ctx.mk_identifier_or_keyword(c).unwrap();

        assert_eq!(1, tkn.span.start_col.0);
        assert!(tkn.span.is_one_line());
        assert_eq!(Lexeme::Identifier("abc".to_string()), tkn.lexeme);

        // Read the _ character
        read_and_ignore(&mut ctx);
    }
}
