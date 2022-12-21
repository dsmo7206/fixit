#![deny(missing_docs)]
//! Converts infix tokens to postfix tokens, otherwise known as <https://en.wikipedia.org/wiki/Reverse_Polish_notation>.
//!
//! This library does not do parsing or execution of expressions, but only filters/reorders tokens to allow stack-based execution.
//! The (possibly less efficient) alternative to stack-based execution usually involves walking an expression tree.
//!
//! # Examples
//! ```
//! use fixit::{BinaryOperator, InfixToken, convert};
//!
//! // Define your own operator
//! enum MyBinaryOp {
//!     Add,
//!     Sub,
//!     Mul,
//!     Div,
//! }
//!
//! // Assign a precedence to each operator
//! impl BinaryOperator for MyBinaryOp {
//!     fn precedence(&self) -> u8 {
//!         match self {
//!             MyBinaryOp::Add => 1,
//!             MyBinaryOp::Sub => 1,
//!             MyBinaryOp::Mul => 2,
//!             MyBinaryOp::Div => 2,
//!         }
//!     }
//! }
//!
//! // Parse and cache the postfix expression
//! fn parse_and_cache_expr(expr: &str) {
//!     // Parsing an expression like "a + b * c + d" should give:
//!     // vec![
//!     //     InfixToken::Operand("a"), InfixToken::BinaryOp(MyBinaryOp::Add), InfixToken::Operand("b"),
//!     //     InfixToken::BinaryOp(MyBinaryOp::Mul),
//!     //     InfixToken::Operand("c"), InfixToken::BinaryOp(MyBinaryOp::Add), InfixToken::Operand("d")
//!     // ]
//!     let infix_tokens = parse_expr(expr); // Could be done via crates `nom`, `pest`, or others
//!
//!     // Convert to postfix. The result will contain:
//!     // vec![
//!     //     PostfixToken::Operand("a"),
//!     //     PostfixToken::Operand("b"),
//!     //     PostfixToken::Operand("c"),
//!     //     PostfixToken::BinaryOp(MyBinaryOp::Mul),
//!     //     PostfixToken::BinaryOp(MyBinaryOp::Add),
//!     //     PostfixToken::Operand("d"),
//!     //     PostfixToken::BinaryOp(MyBinaryOp::Add)
//!     // ]
//!     let postfix_tokens = convert(infix_tokens);
//!
//!     // We don't aim to provide a full explanation of postfix expression here, but
//!     // the expression can now be evaluated by iterating over postfix tokens,
//!     // pushing and popping values off a stack.
//! }
//! ```

mod algorithm;
mod tokens;

pub use algorithm::{convert, ConvertError};
pub use tokens::{BinaryOperator, InfixToken, PostfixToken};
