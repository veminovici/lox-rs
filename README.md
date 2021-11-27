# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Lox... 
A collection of rust crates for an interpreter.

</br>

![GitHub top language](https://img.shields.io/github/languages/top/veminovici/lox-rs)
[![CI Pipeline](https://github.com/veminovici/lox-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/veminovici/lox-rs/actions/workflows/ci.yml)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/lox-rs)](https://github.com/veminovici/lox-rs)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/lox-rs)](https://github.com/veminovici/lox-rs)
[![GitHub issues](https://img.shields.io/github/issues/veminovici/lox-rs)](https://github.com/veminovici/lox-rs/issues)

</br>

## Lexer
The project contains the **lox-lexer** crate. The crate implements the **Lexer** structure which allows the caller to parse a string source and get back an iterator which gives you access to a collection of **tokens**. 

### Span
The **Span** structure encapsulates information about the location of a lexeme in the source stream such *start* line and column and *end* line and column.

### Lexeme & Token
The **Lexeme** is an enumeration type which represents the list of supported *lexemes* in the language. The **Token** structure just pairs together a *Lexeme* with its location, the *span*.

```rust
pub enum Lexeme {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,
    Identifier(String), String(String), Number(f64), Comment(String), Whitepsace(String),
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, NewLine, Eof,
}

pub struct Token {
    lexeme: Lexeme,
    span: Span,
}
```

### Lexer
The **Lexer** contains the scanning logic. The manin structure is **Context**, which keeps a scanning context: the *source string*, the current *span*. 
Couple of helper structures, **Lexer** and **LexerIter** are provided, so the list of tokens is obtain as a simple *Iterator*.

```rust
let lxr: LexerIter = "var x = \"test\"".into();
lxr.for_each(|tkn| println!("{:?}", tkn));

// VAR [1:0-3], WHITESPACE( ) [1:3-4], IDENTITY(language) [1:4-12], EQUAL [1:12-13], 
// NEW_LINE [1:13-2:0], STRING(lox [2:0-5], SEMICOLON [2:5-6], EOF [2:6-6]
```

</br>

## Project Status

[![Github Actions](https://buildstats.info/github/chart/veminovici/lox-rs)](https://github.com/veminovici/lox-rs)

### Test Coverage
In order to see the test coverage numbers, you can run while in the project root directory:

```bash
chmod +x ./cov.sh
./cov.sh
```

## Resources & Credits
- [Crafting interpreters](http://craftinginterpreters.com/)
- [Test coverage](https://vladfilippov.com/blog/rust-code-coverage-tools/)

</br>

## Contact

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
