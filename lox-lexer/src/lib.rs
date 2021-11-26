//! A crate for nyxx lexer
//!
#![deny(missing_docs)]
#![deny(unreachable_code)]

mod lexeme;
mod span;

pub use crate::lexeme::*;
pub use crate::span::*;
