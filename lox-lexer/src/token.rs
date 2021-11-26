use std::fmt::{Debug, Display};

use super::lexeme::Lexeme;
use super::span::Span;

/// Represents a token generated by the lexer.
pub struct Token {
    pub(crate) lexeme: Lexeme,
    pub(crate) span: Span,
}

impl Token {
    fn new(l: Lexeme, s: Span) -> Self {
        Token { lexeme: l, span: s }
    }

    /// Creates a new 'left parenthesis' token
    #[inline]
    pub fn new_left_paren(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::LeftParen, s)
    }

    /// Creates a new 'right parenthesis' token
    #[inline]
    pub fn new_right_paren(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::RightParen, s)
    }

    /// Creates a new 'left brace' token
    #[inline]
    pub fn new_left_brace(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::LeftBrace, s)
    }

    /// Creates a new 'right brace' token
    #[inline]
    pub fn new_right_brace(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::RightBrace, s)
    }

    /// Creates a new 'comma' token
    #[inline]
    pub fn new_comma(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Comma, s)
    }

    /// Creates a new 'dot' token.
    #[inline]
    pub fn new_dot(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Dot, s)
    }

    /// Creates a new 'plus' token.
    #[inline]
    pub fn new_plus(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Plus, s)
    }

    /// Creates a new 'minus' token.
    #[inline]
    pub fn new_minus(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Minus, s)
    }

    /// Creates a new 'semicolon' token.
    #[inline]
    pub fn new_semicolon(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Semicolon, s)
    }

    /// Creates a new 'slash' token.
    #[inline]
    pub fn new_slash(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Slash, s)
    }

    /// Creates a new 'star' token.
    #[inline]
    pub fn new_star(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Star, s)
    }

    /// Creates a new 'bang' token.
    #[inline]
    pub fn new_bang(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Bang, s)
    }

    /// Creates a new 'bang-equal' token.
    #[inline]
    pub fn new_bang_equal(s: Span) -> Self {
        debug_assert!(s.is_two_chars());
        Self::new(Lexeme::BangEqual, s)
    }

    /// Creates a new 'equal' token.
    #[inline]
    pub fn new_equal(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Equal, s)
    }

    /// Cteayes a mew 'equal-equal' token.
    #[inline]
    pub fn new_equal_equal(s: Span) -> Self {
        debug_assert!(s.is_two_chars());
        Self::new(Lexeme::EqualEqual, s)
    }

    /// Creates a new 'greater' token.
    #[inline]
    pub fn new_greater(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Greater, s)
    }

    /// Cteayes a mew 'greater-equal' token.
    #[inline]
    pub fn new_greater_equal(s: Span) -> Self {
        debug_assert!(s.is_two_chars());
        Self::new(Lexeme::GreaterEqual, s)
    }

    /// Creates a new 'less' token.
    #[inline]
    pub fn new_less(s: Span) -> Self {
        debug_assert!(s.is_one_char());
        Self::new(Lexeme::Less, s)
    }

    /// Creates a new 'less-equal' token.
    #[inline]
    pub fn new_less_equal(s: Span) -> Self {
        debug_assert!(s.is_two_chars());
        Self::new(Lexeme::LessEqual, s)
    }

    /// Createsa new 'identifier' token.
    #[inline]
    pub fn new_identifier(i: &str, s: Span) -> Self {
        Self::new(Lexeme::Identifier(i.to_string()), s)
    }

    /// Creates a new 'string' token.
    #[inline]
    pub fn new_string(str: &str, s: Span) -> Self {
        Self::new(Lexeme::String(str.to_string()), s)
    }

    /// Creates a new 'number' token.
    #[inline]
    pub fn new_number(number: f64, s: Span) -> Self {
        Self::new(Lexeme::Number(number), s)
    }

    /// Creates a new 'comment' token.
    #[inline]
    pub fn new_comment(c: &str, s: Span) -> Self {
        Self::new(Lexeme::Comment(c.to_string()), s)
    }

    /// Creates a new 'and' token.
    #[inline]
    pub fn new_and(s: Span) -> Self {
        debug_assert!(s.is_n_chars(3));
        Self::new(Lexeme::And, s)
    }

    /// Creates a new 'class' token.
    #[inline]
    pub fn new_class(s: Span) -> Self {
        debug_assert!(s.is_n_chars(5));
        Self::new(Lexeme::Class, s)
    }

    /// Creates a new 'else' token.
    #[inline]
    pub fn new_else(s: Span) -> Self {
        debug_assert!(s.is_n_chars(4));
        Self::new(Lexeme::Else, s)
    }

    /// Creates a new 'false' token.
    #[inline]
    pub fn new_false(s: Span) -> Self {
        debug_assert!(s.is_n_chars(5));
        Self::new(Lexeme::False, s)
    }

    /// Creates a new 'fun' token.
    #[inline]
    pub fn new_fun(s: Span) -> Self {
        debug_assert!(s.is_n_chars(3));
        Self::new(Lexeme::Fun, s)
    }

    /// Creates a new 'for' token.
    #[inline]
    pub fn new_for(s: Span) -> Self {
        debug_assert!(s.is_n_chars(3));
        Self::new(Lexeme::For, s)
    }

    /// Creates a new 'if' token.
    #[inline]
    pub fn new_if(s: Span) -> Self {
        debug_assert!(s.is_n_chars(2));
        Self::new(Lexeme::If, s)
    }

    /// Creates a new 'nil' token.
    #[inline]
    pub fn new_nil(s: Span) -> Self {
        debug_assert!(s.is_n_chars(3));
        Self::new(Lexeme::Nil, s)
    }

    /// Creates a new 'or' token.
    #[inline]
    pub fn new_or(s: Span) -> Self {
        debug_assert!(s.is_n_chars(2));
        Self::new(Lexeme::Or, s)
    }

    /// Creates a new 'print' token.
    #[inline]
    pub fn new_print(s: Span) -> Self {
        debug_assert!(s.is_n_chars(5));
        Self::new(Lexeme::Print, s)
    }

    /// Creates a new 'return' token.
    #[inline]
    pub fn new_return(s: Span) -> Self {
        debug_assert!(s.is_n_chars(6));
        Self::new(Lexeme::Return, s)
    }

    /// Creates a new 'super' token.
    #[inline]
    pub fn new_super(s: Span) -> Self {
        debug_assert!(s.is_n_chars(5));
        Self::new(Lexeme::Super, s)
    }

    /// Creates a new 'this' token.
    #[inline]
    pub fn new_this(s: Span) -> Self {
        debug_assert!(s.is_n_chars(4));
        Self::new(Lexeme::This, s)
    }

    /// Creates a new 'this' token.
    #[inline]
    pub fn new_true(s: Span) -> Self {
        debug_assert!(s.is_n_chars(4));
        Self::new(Lexeme::True, s)
    }

    /// Creates a new 'var' token.
    #[inline]
    pub fn new_var(s: Span) -> Self {
        debug_assert!(s.is_n_chars(3));
        Self::new(Lexeme::Var, s)
    }

    /// Creates a new 'while' token.
    #[inline]
    pub fn new_while(s: Span) -> Self {
        debug_assert!(s.is_n_chars(5));
        Self::new(Lexeme::While, s)
    }

    /// Creates a new 'whitespace' token.
    #[inline]
    pub fn new_whitespace(ws: &str, s: Span) -> Self {
        Self::new(Lexeme::Whitespace(ws.to_string()), s)
    }

    /// Creates a new 'new_line' token.
    #[inline]
    pub fn new_newline(s: Span) -> Self {
        Self::new(Lexeme::NewLine, s)
    }

    /// Creates a new 'eof' token.
    #[inline]
    pub fn new_eof(s: Span) -> Self {
        Self::new(Lexeme::Eof, s)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} [{:?}]", self.lexeme, self.span)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{:?}]", self.lexeme, self.span)
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::Column;

    use super::*;
    use crate::span::*;

    #[test]
    fn test_new_left_paren() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_left_paren(s);
        assert_eq!(Lexeme::LeftParen, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_right_paren() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_right_paren(s);
        assert_eq!(Lexeme::RightParen, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_left_brace() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_left_brace(s);
        assert_eq!(Lexeme::LeftBrace, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_right_brace() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_right_brace(s);
        assert_eq!(Lexeme::RightBrace, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_comma() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_comma(s);
        assert_eq!(Lexeme::Comma, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_dot() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_dot(s);
        assert_eq!(Lexeme::Dot, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_plus() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_plus(s);
        assert_eq!(Lexeme::Plus, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_minus() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_minus(s);
        assert_eq!(Lexeme::Minus, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_semicolon() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_semicolon(s);
        assert_eq!(Lexeme::Semicolon, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_slash() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_slash(s);
        assert_eq!(Lexeme::Slash, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_star() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_star(s);
        assert_eq!(Lexeme::Star, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_bang() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_bang(s);
        assert_eq!(Lexeme::Bang, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_bang_equal() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col();
        let t = Token::new_bang_equal(s);
        assert_eq!(Lexeme::BangEqual, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_equal() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_equal(s);
        assert_eq!(Lexeme::Equal, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_equal_equal() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col();
        let t = Token::new_equal_equal(s);
        assert_eq!(Lexeme::EqualEqual, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_greater() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_greater(s);
        assert_eq!(Lexeme::Greater, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_greater_equal() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col();
        let t = Token::new_greater_equal(s);
        assert_eq!(Lexeme::GreaterEqual, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_less() {
        let s = Span::new(Line(10), Column(100));
        let t = Token::new_less(s);
        assert_eq!(Lexeme::Less, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_less_equal() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col();
        let t = Token::new_less_equal(s);
        assert_eq!(Lexeme::LessEqual, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_identfier() {
        let mut s = Span::new(Line(10), Column(100));
        let i = "abc";
        s.incr_col_n(i.len());

        let t = Token::new_identifier(i, s);
        assert_eq!(Lexeme::Identifier(i.to_string()), t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_string() {
        let mut s = Span::new(Line(10), Column(100));
        let string = "abc";
        s.incr_col_n(string.len());

        let t = Token::new_string(string, s);
        assert_eq!(Lexeme::String(string.to_string()), t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_comment() {
        let mut s = Span::new(Line(10), Column(100));
        let comment = "abc";
        s.incr_col_n(comment.len());

        let t = Token::new_comment(comment, s);
        assert_eq!(Lexeme::Comment(comment.to_string()), t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_number() {
        let mut s = Span::new(Line(10), Column(100));
        let number = 10.;
        s.incr_col_n(2);

        let t = Token::new_number(number, s);
        assert_eq!(Lexeme::Number(number), t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_and() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(2);
        let t = Token::new_and(s);
        assert_eq!(Lexeme::And, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_class() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(4);
        let t = Token::new_class(s);
        assert_eq!(Lexeme::Class, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_else() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(3);
        let t = Token::new_else(s);
        assert_eq!(Lexeme::Else, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_false() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(4);
        let t = Token::new_false(s);
        assert_eq!(Lexeme::False, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_fun() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(2);
        let t = Token::new_fun(s);
        assert_eq!(Lexeme::Fun, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_for() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(2);
        let t = Token::new_for(s);
        assert_eq!(Lexeme::For, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_if() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(1);
        let t = Token::new_if(s);
        assert_eq!(Lexeme::If, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_nil() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(2);
        let t = Token::new_nil(s);
        assert_eq!(Lexeme::Nil, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_or() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(1);
        let t = Token::new_or(s);
        assert_eq!(Lexeme::Or, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_print() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(4);
        let t = Token::new_print(s);
        assert_eq!(Lexeme::Print, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_return() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(5);
        let t = Token::new_return(s);
        assert_eq!(Lexeme::Return, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_super() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(4);
        let t = Token::new_super(s);
        assert_eq!(Lexeme::Super, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_this() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(3);
        let t = Token::new_this(s);
        assert_eq!(Lexeme::This, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_true() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(3);
        let t = Token::new_true(s);
        assert_eq!(Lexeme::True, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_var() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(2);
        let t = Token::new_var(s);
        assert_eq!(Lexeme::Var, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_while() {
        let mut s = Span::new(Line(10), Column(100));
        s.incr_col_n(4);
        let t = Token::new_while(s);
        assert_eq!(Lexeme::While, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_whitespace() {
        let mut s = Span::new(Line(10), Column(100));
        let i = "abc";
        s.incr_col_n(i.len());

        let t = Token::new_whitespace(i, s);
        assert_eq!(Lexeme::Whitespace(i.to_string()), t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_newline() {
        let s = Span::new(Line(10), Column(100));

        let t = Token::new_newline(s);
        assert_eq!(Lexeme::NewLine, t.lexeme);
        assert_eq!(s, t.span);
    }

    #[test]
    fn test_new_eof() {
        let s = Span::new(Line(10), Column(100));

        let t = Token::new_eof(s);
        assert_eq!(Lexeme::Eof, t.lexeme);
        assert_eq!(s, t.span);
    }
}
