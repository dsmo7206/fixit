use super::tokens::{BinaryOperator, InfixToken, PostfixToken, StackToken};
use std::{error::Error, fmt};

/// An error during infix to postfix conversion.
#[derive(Debug, PartialEq, Eq)]
pub enum ConvertError {
    /// The number of [`GroupStart`](InfixToken::GroupStart) and [`GroupEnd`](InfixToken::GroupEnd) tokens did not match.
    UnbalancedGroups(i32),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::UnbalancedGroups(_) => write!(f, "Unbalanced groups"),
        }
    }
}

impl Error for ConvertError {}

/// Converts an iterator of [`InfixToken`] to a [`Vec`] of [`PostfixToken`].
///
/// Note that the only error checking this function performs is to make sure
/// that the number of group starts and finishes is equal. This is checked to avoid
/// executing `unreachable` code.
///
/// Critically, this function will not check that the `tokens` argument is a valid
/// infix token stream, particularly that each [`BinaryOp`](InfixToken::BinaryOp)
/// has two adjacent [`Operand`](InfixToken::Operand)s.
///
/// Returns `Ok(...)` on success, otherwise returns a [`ConvertError`].
///
/// # Errors
///
/// See [`ConvertError`].
pub fn convert<Operand, BinaryOp, I>(
    tokens: I,
) -> Result<Vec<PostfixToken<Operand, BinaryOp>>, ConvertError>
where
    I: IntoIterator<Item = InfixToken<Operand, BinaryOp>>,
    BinaryOp: BinaryOperator,
{
    let mut result = vec![];
    let mut stack: Vec<StackToken<BinaryOp>> = vec![];
    let mut group_depth = 0;

    tokens.into_iter().for_each(|token| match token {
        InfixToken::Operand(name) => result.push(PostfixToken::Operand(name)),
        InfixToken::BinaryOp(op) => {
            while stack
                .last()
                .map_or(false, |last| stack_to_result(last, &op))
            {
                // Safe to `unwrap` because `stack.last()` returned `Some`
                result.push(stack.pop().unwrap().into());
            }

            stack.push(StackToken::BinaryOp(op));
        }
        InfixToken::GroupStart => {
            stack.push(StackToken::GroupStart);
            group_depth += 1;
        }
        InfixToken::GroupEnd => {
            while let Some(last) = stack.pop() {
                match last {
                    StackToken::BinaryOp(op) => result.push(PostfixToken::BinaryOp(op)),
                    StackToken::GroupStart => break,
                }
            }
            group_depth -= 1;
        }
    });

    match group_depth {
        0 => {
            result.extend(stack.into_iter().rev().map(Into::into));
            Ok(result)
        }
        group_depth => Err(ConvertError::UnbalancedGroups(group_depth)),
    }
}

fn stack_to_result<BinaryOp>(last: &StackToken<BinaryOp>, op: &BinaryOp) -> bool
where
    BinaryOp: BinaryOperator,
{
    match last {
        StackToken::BinaryOp(last_op) => last_op.precedence() >= op.precedence(),
        StackToken::GroupStart => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{convert, BinaryOperator, ConvertError, InfixToken, PostfixToken};

    #[derive(Debug, PartialEq, Eq)]
    enum TestBinaryOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl BinaryOperator for TestBinaryOp {
        fn precedence(&self) -> u8 {
            match self {
                TestBinaryOp::Add => 1,
                TestBinaryOp::Sub => 1,
                TestBinaryOp::Mul => 2,
                TestBinaryOp::Div => 2,
            }
        }
    }

    #[test]
    fn test_ok_1() {
        let infix_tokens = vec![
            InfixToken::Operand("m"),
            InfixToken::BinaryOp(TestBinaryOp::Mul),
            InfixToken::Operand("n"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::GroupStart,
            InfixToken::Operand("p"),
            InfixToken::BinaryOp(TestBinaryOp::Sub),
            InfixToken::Operand("q"),
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("r"),
        ];

        assert_eq!(
            convert(infix_tokens).unwrap(),
            vec![
                PostfixToken::Operand("m"),
                PostfixToken::Operand("n"),
                PostfixToken::BinaryOp(TestBinaryOp::Mul),
                PostfixToken::Operand("p"),
                PostfixToken::Operand("q"),
                PostfixToken::BinaryOp(TestBinaryOp::Sub),
                PostfixToken::BinaryOp(TestBinaryOp::Add),
                PostfixToken::Operand("r"),
                PostfixToken::BinaryOp(TestBinaryOp::Add),
            ]
        );
    }

    #[test]
    fn test_ok_2() {
        let infix_tokens = vec![
            InfixToken::Operand("a"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("b"),
            InfixToken::BinaryOp(TestBinaryOp::Mul),
            InfixToken::Operand("c"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("d"),
        ];

        assert_eq!(
            convert(infix_tokens).unwrap(),
            vec![
                PostfixToken::Operand("a"),
                PostfixToken::Operand("b"),
                PostfixToken::Operand("c"),
                PostfixToken::BinaryOp(TestBinaryOp::Mul),
                PostfixToken::BinaryOp(TestBinaryOp::Add),
                PostfixToken::Operand("d"),
                PostfixToken::BinaryOp(TestBinaryOp::Add)
            ]
        );
    }

    #[test]
    fn test_ok_3() {
        let infix_tokens = vec![
            InfixToken::GroupStart,
            InfixToken::GroupStart,
            InfixToken::Operand("a"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("b"),
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Sub),
            InfixToken::Operand("c"),
            InfixToken::BinaryOp(TestBinaryOp::Mul),
            InfixToken::GroupStart,
            InfixToken::Operand("d"),
            InfixToken::BinaryOp(TestBinaryOp::Div),
            InfixToken::Operand("e"),
            InfixToken::GroupEnd,
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("f"),
        ];

        assert_eq!(
            convert(infix_tokens).unwrap(),
            vec![
                PostfixToken::Operand("a"),
                PostfixToken::Operand("b"),
                PostfixToken::BinaryOp(TestBinaryOp::Add),
                PostfixToken::Operand("c"),
                PostfixToken::Operand("d"),
                PostfixToken::Operand("e"),
                PostfixToken::BinaryOp(TestBinaryOp::Div),
                PostfixToken::BinaryOp(TestBinaryOp::Mul),
                PostfixToken::BinaryOp(TestBinaryOp::Sub),
                PostfixToken::Operand("f"),
                PostfixToken::BinaryOp(TestBinaryOp::Add),
            ]
        );
    }

    #[test]
    fn test_bad_1() {
        let infix_tokens = vec![
            InfixToken::GroupStart,
            InfixToken::GroupStart,
            InfixToken::Operand("a"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("b"),
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Sub),
            InfixToken::Operand("c"),
            InfixToken::BinaryOp(TestBinaryOp::Mul),
            InfixToken::GroupStart,
            InfixToken::Operand("d"),
            InfixToken::BinaryOp(TestBinaryOp::Div),
            InfixToken::Operand("e"),
            InfixToken::GroupEnd,
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("f"),
            InfixToken::GroupStart, // Extra
        ];

        let result = convert(infix_tokens).unwrap_err();

        assert_eq!(result, ConvertError::UnbalancedGroups(1));
        assert_eq!(result.to_string(), "Unbalanced groups");
    }

    #[test]
    fn test_bad_2() {
        let infix_tokens = vec![
            InfixToken::GroupStart,
            InfixToken::GroupStart,
            InfixToken::Operand("a"),
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("b"),
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Sub),
            InfixToken::Operand("c"),
            InfixToken::BinaryOp(TestBinaryOp::Mul),
            InfixToken::GroupStart,
            InfixToken::Operand("d"),
            InfixToken::BinaryOp(TestBinaryOp::Div),
            InfixToken::Operand("e"),
            InfixToken::GroupEnd,
            InfixToken::BinaryOp(TestBinaryOp::Add),
            InfixToken::Operand("f"),
            // Missing GroupEnd
        ];

        let result = convert(infix_tokens).unwrap_err();

        assert_eq!(result, ConvertError::UnbalancedGroups(1));
        assert_eq!(result.to_string(), "Unbalanced groups");
    }
}
