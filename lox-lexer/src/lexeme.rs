use std::fmt::{Debug, Display};
use std::string::String;

/// Represents the lexemes supported by the language.
#[derive(Clone, PartialEq)]
pub enum Lexeme {
    //
    // Single-char lexemes
    //
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
    /// Left brace
    LeftBrace,
    /// Right brace
    RightBrace,
    /// Comma
    Comma,
    /// Dot
    Dot,
    /// Minus
    Minus,
    /// Plus
    Plus,
    /// Semicolon
    Semicolon,
    /// Slash
    Slash,
    /// Start
    Star,
    //
    // One or two characters lexemes
    //
    /// Bang
    Bang,
    /// BangEqual
    BangEqual,
    /// Equal
    Equal,
    /// EqualEqual
    EqualEqual,
    /// Greater
    Greater,
    /// GreaterEqual
    GreaterEqual,
    /// Less
    Less,
    /// LessEqual
    LessEqual,
    //
    // Literals lexemes
    //
    /// Identity
    Identifier(String),
    /// String
    String(String),
    /// Number
    Number(f64),
    /// Comment
    Comment(String),
    //
    // Keywords lexemes
    //
    /// And
    And,
    /// Class
    Class,
    /// Else
    Else,
    /// False
    False,
    /// Fun
    Fun,
    /// For
    For,
    /// If
    If,
    /// Nil
    Nil,
    /// Or
    Or,
    /// Print
    Print,
    /// Return
    Return,
    /// Super
    Super,
    /// This
    This,
    /// True
    True,
    /// Var
    Var,
    /// While
    While,
    //
    // Other lexemes
    //
    /// Whitespace
    Whitespace(String),
    /// New line
    NewLine,
    /// EOF
    Eof,
}

use Lexeme::*;

impl Debug for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParen => write!(f, "L_PAREN"),
            RightParen => write!(f, "R_PAREN"),
            LeftBrace => write!(f, "L_BRACE"),
            RightBrace => write!(f, "R_BRACE"),
            Comma => write!(f, "COMMA"),
            Dot => write!(f, "DOT"),
            Minus => write!(f, "MINUS"),
            Plus => write!(f, "PLUS"),
            Semicolon => write!(f, "SEMICOLON"),
            Slash => write!(f, "SLASH"),
            Star => write!(f, "STAR"),
            Bang => write!(f, "BANG"),
            BangEqual => write!(f, "BANG_EQUAL"),
            Equal => write!(f, "EQUAL"),
            EqualEqual => write!(f, "EQUAL_EQUAL"),
            Greater => write!(f, "GREATER"),
            GreaterEqual => write!(f, "GREATER_EQUAL"),
            Less => write!(f, "LESS"),
            LessEqual => write!(f, "LESS_EQUAL"),
            Identifier(i) => write!(f, "IDENTITY({})", i),
            String(string) => write!(f, "STRING({}", string),
            Number(number) => write!(f, "NUMBER({})", number),
            Comment(comment) => write!(f, "COMMENT({})", comment),
            And => write!(f, "AND"),
            Class => write!(f, "CLASS"),
            Else => write!(f, "ELSE"),
            False => write!(f, "FALSE"),
            Fun => write!(f, "FUN"),
            For => write!(f, "FOR"),
            If => write!(f, "IF"),
            Nil => write!(f, "NIL"),
            Or => write!(f, "OR"),
            Print => write!(f, "PRINT"),
            Return => write!(f, "RETURN"),
            Super => write!(f, "SUPER"),
            This => write!(f, "THIS"),
            True => write!(f, "TRUE"),
            Var => write!(f, "VAR"),
            While => write!(f, "WHILE"),
            Whitespace(ws) => write!(f, "WHITESPACE({})", ws),
            NewLine => write!(f, "NEW_LINE"),
            Eof => write!(f, "EOF"),
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Identifier(i) => write!(f, "id({})", i),
            String(string) => write!(f, "str({}", string),
            Number(number) => write!(f, "num({})", number),
            Comment(comment) => write!(f, "cmt({})", comment),
            And => write!(f, "and"),
            Class => write!(f, "class"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            Fun => write!(f, "fun"),
            For => write!(f, "for"),
            If => write!(f, "if"),
            Nil => write!(f, "nil"),
            Or => write!(f, "or"),
            Print => write!(f, "print"),
            Return => write!(f, "return"),
            Super => write!(f, "super"),
            This => write!(f, "this"),
            True => write!(f, "true"),
            Var => write!(f, "var"),
            While => write!(f, "while"),
            Whitespace(ws) => write!(f, "ws({})", ws),
            NewLine => write!(f, "nl"),
            Eof => write!(f, "eof"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexeme;

    fn test_lexeme(l: Lexeme) {
        let s = format!("{}", l);
        assert!(!s.is_empty());

        let s = format!("{:?}", l);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_debug_left_parenthesis() {
        test_lexeme(Lexeme::LeftParen);
    }

    #[test]
    fn test_debug_right_parenthesis() {
        test_lexeme(Lexeme::RightParen);
    }

    #[test]
    fn test_debug_left_brace() {
        test_lexeme(Lexeme::LeftBrace);
    }

    #[test]
    fn test_debug_right_brace() {
        test_lexeme(Lexeme::RightBrace);
    }

    #[test]
    fn test_debug_comma() {
        test_lexeme(Lexeme::Comma);
    }

    #[test]
    fn test_debug_dot() {
        test_lexeme(Lexeme::Dot);
    }

    #[test]
    fn test_debug_plus() {
        test_lexeme(Lexeme::Plus);
    }

    #[test]
    fn test_debug_minus() {
        test_lexeme(Lexeme::Minus);
    }

    #[test]
    fn test_debug_semicolon() {
        test_lexeme(Lexeme::Semicolon);
    }

    #[test]
    fn test_debug_slash() {
        test_lexeme(Lexeme::Slash);
    }

    #[test]
    fn test_debug_star() {
        test_lexeme(Lexeme::Star);
    }

    #[test]
    fn test_debug_bang() {
        test_lexeme(Lexeme::Bang);
    }

    #[test]
    fn test_debug_bang_equal() {
        test_lexeme(Lexeme::BangEqual);
    }

    #[test]
    fn test_debug_equal() {
        test_lexeme(Lexeme::Equal);
    }

    #[test]
    fn test_debug_equal_equal() {
        test_lexeme(Lexeme::EqualEqual);
    }

    #[test]
    fn test_debug_greater() {
        test_lexeme(Lexeme::Greater);
    }

    #[test]
    fn test_debug_greater_equal() {
        test_lexeme(Lexeme::GreaterEqual);
    }

    #[test]
    fn test_debug_less() {
        test_lexeme(Lexeme::Less);
    }

    #[test]
    fn test_debug_less_equal() {
        test_lexeme(Lexeme::LessEqual);
    }

    #[test]
    fn test_debug_identifier() {
        test_lexeme(Lexeme::Identifier("hello".to_string()));
    }

    #[test]
    fn test_debug_string() {
        test_lexeme(Lexeme::String("hello".to_string()));
    }

    #[test]
    fn test_debug_comment() {
        test_lexeme(Lexeme::Comment("hello".to_string()));
    }

    #[test]
    fn test_debug_number() {
        test_lexeme(Lexeme::Number(12.3));
    }

    #[test]
    fn test_debug_and() {
        test_lexeme(Lexeme::And);
    }

    #[test]
    fn test_debug_class() {
        test_lexeme(Lexeme::Class);
    }

    #[test]
    fn test_debug_else() {
        test_lexeme(Lexeme::Else);
    }

    #[test]
    fn test_debug_false() {
        test_lexeme(Lexeme::False);
    }

    #[test]
    fn test_debug_for() {
        test_lexeme(Lexeme::For);
    }

    #[test]
    fn test_debug_fun() {
        test_lexeme(Lexeme::Fun);
    }

    #[test]
    fn test_debug_if() {
        test_lexeme(Lexeme::If);
    }

    #[test]
    fn test_debug_nil() {
        test_lexeme(Lexeme::Nil);
    }

    #[test]
    fn test_debug_or() {
        test_lexeme(Lexeme::Or);
    }

    #[test]
    fn test_debug_print() {
        test_lexeme(Lexeme::Print);
    }

    #[test]
    fn test_debug_return() {
        test_lexeme(Lexeme::Return);
    }

    #[test]
    fn test_debug_super() {
        test_lexeme(Lexeme::Super);
    }

    #[test]
    fn test_debug_this() {
        test_lexeme(Lexeme::This);
    }

    #[test]
    fn test_debug_true() {
        test_lexeme(Lexeme::True);
    }

    #[test]
    fn test_debug_var() {
        test_lexeme(Lexeme::Var);
    }

    #[test]
    fn test_debug_while() {
        test_lexeme(Lexeme::While);
    }

    #[test]
    fn test_debug_whitespace() {
        test_lexeme(Lexeme::Whitespace("  ".to_string()));
    }

    #[test]
    fn test_debug_newline() {
        test_lexeme(Lexeme::NewLine);
    }

    #[test]
    fn test_debug_eof() {
        test_lexeme(Lexeme::Eof);
    }
}
