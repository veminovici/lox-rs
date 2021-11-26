//! A crate for a lexer
//!
#![deny(missing_docs)]
#![deny(unreachable_code)]

mod chars;
mod lexeme;
mod lexer;
mod span;
mod token;

pub use crate::lexeme::*;
pub use crate::lexer::*;
pub use crate::span::*;
pub use crate::token::*;
