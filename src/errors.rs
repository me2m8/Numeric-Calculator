#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidExpression(String),
    InvalidCharacter(char),
    UnknownKeyword(String),
    NoFunctionArguments(String),
    InvalidArgumentCount { expected: usize, got: usize },
    ProgramIsStupid,
    // InvalidToken,
    // InvalidOperator,
    // InvalidNumber,
    // InvalidParentheses,
}
