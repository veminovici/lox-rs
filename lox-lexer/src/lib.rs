//! A crate for nyxx lexer
//!
#![deny(missing_docs)]
#![deny(unreachable_code)]

mod lexeme;
mod lexer;
mod span;
mod token;

pub use crate::lexeme::*;
pub use crate::lexer::*;
pub use crate::span::*;
pub use crate::token::*;
