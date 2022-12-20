#![deny(missing_docs)]
//! Converts infix tokens to postfix tokens, otherwise known as <https://en.wikipedia.org/wiki/Reverse_Polish_notation>.

mod algorithm;
mod tokens;

pub use algorithm::{convert, ConvertError};
pub use tokens::{BinaryOperator, InfixToken, PostfixToken};
