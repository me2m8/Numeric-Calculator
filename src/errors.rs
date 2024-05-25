
#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidExpression,
    InvalidCharacter(char),
    // InvalidToken,
    // InvalidOperator,
    // InvalidNumber,
    // InvalidParentheses,
}
