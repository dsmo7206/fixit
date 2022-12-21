# fixit

Converts infix tokens to postfix tokens, otherwise known as <https://en.wikipedia.org/wiki/Reverse_Polish_notation>.

This library does not do parsing or execution of expressions, but only filters/reorders tokens to allow stack-based execution.
The (possibly less efficient) alternative to stack-based execution usually involves walking an expression tree.

## Example
```rust
use fixit::{BinaryOperator, InfixToken, convert};

// Define your own operand: it could be anything.
type MyOperand = f32;

// Define your own operator.
enum MyBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

// Assign a precedence to each operator
impl BinaryOperator for MyBinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            MyBinaryOp::Add => 1,
            MyBinaryOp::Sub => 1,
            MyBinaryOp::Mul => 2,
            MyBinaryOp::Div => 2,
        }
    }
}

type MyInfixToken = InfixToken<MyOperand, MyBinaryOp>;

// The incoming `expr` tokens could have been parsed from a string via a crate like `nom` or `pest`.
// Parsing an expression like "a + b * c + d" should give:
// vec![
//     InfixToken::Operand("a"), InfixToken::BinaryOp(MyBinaryOp::Add), InfixToken::Operand("b"),
//     InfixToken::BinaryOp(MyBinaryOp::Mul),
//     InfixToken::Operand("c"), InfixToken::BinaryOp(MyBinaryOp::Add), InfixToken::Operand("d")
// ]
fn example(expr: Vec<MyInfixToken>) {
    // Convert to postfix. The result will contain:
    // vec![
    //     PostfixToken::Operand("a"),
    //     PostfixToken::Operand("b"),
    //     PostfixToken::Operand("c"),
    //     PostfixToken::BinaryOp(MyBinaryOp::Mul),
    //     PostfixToken::BinaryOp(MyBinaryOp::Add),
    //     PostfixToken::Operand("d"),
    //     PostfixToken::BinaryOp(MyBinaryOp::Add)
    // ]
    let postfix_tokens = convert(expr);

    // We don't aim to provide a full explanation of postfix expressions here, but
    // the expression can now be evaluated by iterating over postfix tokens,
    // pushing and popping values off a stack.
}
```

License: MIT OR Apache-2.0
