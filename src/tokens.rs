/// A input token, presumably from a parsed stream defining an arithmetic-like expression in human-readable ("infix") order.
///
/// Note that there is no `UnaryOp` arm; it is assumed that unary operators have already been encoded as a single [`Operand`](InfixToken::Operand).
#[derive(Debug, PartialEq, Eq)]
pub enum InfixToken<Operand, BinaryOp> {
    /// An operand that may be operated on. This might be a scalar, a list, a function call, etc.
    Operand(Operand),
    /// A binary operator, for example an addition of two values.
    BinaryOp(BinaryOp),
    /// An abstract start of a group.
    /// Corresponds to a left parenthesis (`(`) in a typical arithmetic expression.
    GroupStart,
    /// An abstract end of a group.
    /// Corresponds to a right parenthesis (`)`) in a typical arithmetic expression.
    GroupEnd,
}

/// An output token.
///
/// This is the subset of an [`InfixToken`] excluding the [`GroupStart`](InfixToken::GroupStart`) and [`GroupEnd`](InfixToken::GroupEnd) arms.
///
/// Those arms are unnecessary in a postfix expression because operators are immediately applied to the top values on the evaluation stack.
#[derive(Debug, PartialEq, Eq)]
pub enum PostfixToken<Operand, BinaryOp> {
    /// An operand that may be operated on. This might be a scalar, a list, a function call, etc.
    Operand(Operand),
    /// A binary operator, for example an addition of two values.
    BinaryOp(BinaryOp),
}

/// Trait required for values used as the `BinaryOp` arm of an [`InfixToken`] or [`PostfixToken`].
///
/// # Example
///
/// ```
/// enum MyBinaryOp {
///     Add,
///     Sub,
///     Mul,
///     Div,
/// }
///
///
/// impl fixit::BinaryOperator for MyBinaryOp {
///     fn precedence(&self) -> u8 {
///         match self {
///             MyBinaryOp::Add => 1,
///             MyBinaryOp::Sub => 1,
///             MyBinaryOp::Mul => 2,
///             MyBinaryOp::Div => 2,
///         }
///     }
/// }
/// ```
pub trait BinaryOperator {
    /// The precedence of the operator, i.e. how tightly it is bound to its two operands.
    /// Higher values mean operators that are applied first.
    ///
    /// For example, in a typical arithmetic expression, the multiplication operator
    /// has a higher precedence than the addition operator.
    ///
    /// Actual precedence values do not matter, only their relative values.
    fn precedence(&self) -> u8;
}

pub(crate) enum StackToken<BinaryOp> {
    BinaryOp(BinaryOp),
    GroupStart,
}

impl<Operand, BinaryOp> From<StackToken<BinaryOp>> for PostfixToken<Operand, BinaryOp> {
    fn from(value: StackToken<BinaryOp>) -> Self {
        match value {
            StackToken::BinaryOp(op) => PostfixToken::BinaryOp(op),
            StackToken::GroupStart => unreachable!("Unbalanced groups"),
        }
    }
}
