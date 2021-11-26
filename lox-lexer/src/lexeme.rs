use std::fmt::Debug;
use std::string::String;

/// Represents the lexemes supported by the language.
#[derive(Clone)]
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
    Identity(String),
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
            Identity(i) => write!(f, "IDENTITY({})", i),
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
