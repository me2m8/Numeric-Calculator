use crate::{errors::EvalError, keywords::*};

/// Enum representing a symbol/token in the expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Constant(f64),
    Operator(Operator),

    Separator,

    Function((Function, Vec<Vec<Token>>)),
    Keyword(String),

    /// Holds the tokens from an expression inside parentheses.
    InnerExpression(Vec<Token>),
}

/// Enum representing an arithmetic operator.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    Additive(Additive),
    Multiplicative(Multiplicative),
    Exponential,
}

/// Enum representing the additive operators, add and subtract.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Additive {
    Add,
    Subtract,
}

/// Enum representing the multiplicative operators, multiply and divide.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Multiplicative {
    Multiply,
    Divide,
}

pub fn parse_expression(expression: &str) -> Result<Vec<Token>, EvalError> {
    let tokens_pass_1 = parse_tokens(expression)?;
    let tokens = parse_keywords(&tokens_pass_1)?;

    Ok(tokens)
}

/// Parses the expression string into an an array of tokens, representing numbers, operators and
/// expressions inside parentheses
pub fn parse_tokens(expression: &str) -> Result<Vec<Token>, EvalError> {
    // A newline is appended at the end so that the end doesnt end abruptly, allowing numbers to be
    // properly parsed. There is usually a newline at the end of the input, but if there isn't, this
    // will make sure that the expression is properly parsed.
    let expression = String::from(expression) + "\n";

    // the expression stack is used to keep track of the current scope. expressions inside parentheses
    // are parsed into their own token to make constructing the ast easier.
    let mut expr_stack: Vec<Vec<Token>> = vec![Vec::new()];
    let mut top_of_stack = 0;

    // Vec used to store digits for parsing numbers
    let mut num = String::new();

    // Vec used to store letters for parsing keywords like constants and functions
    let mut kword = String::new();

    for c in expression.chars() {
        // If a number has been found, add the char to the num vec and continue to the next char.
        // Repeat until another symbol is found to get the entire number.
        if let '0'..='9' | '.' = c {
            num.push(c);
            continue;
        }

        // When something else is found, parse the digits into a float and clear the num vec.
        if !num.is_empty() {
            let n = num.parse::<f64>().unwrap();
            expr_stack[top_of_stack].push(Token::Number(n));
            num.clear();
        }

        // If a letter is found, add it to the kword vec and continue to the next char.
        if let 'A'..='Z' | 'a'..='z' = c {
            kword.push(c);
            continue;
        }

        // When something else is found, add the keyword to the current scope and clear the kword vec.
        if !kword.is_empty() {
            expr_stack[top_of_stack].push(Token::Keyword(kword.to_string()));
            kword.clear();
        }

        // Matches the different symbols and either adds them to the current scope.
        match c {
            #[rustfmt::skip]
            '+' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Additive(Additive::Add))),
            #[rustfmt::skip]
            '-' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Additive(Additive::Subtract))),
            #[rustfmt::skip]
            '*' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Multiplicative(Multiplicative::Multiply))),
            #[rustfmt::skip]
            '/' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Multiplicative(Multiplicative::Divide))),
            #[rustfmt::skip]
            '^' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Exponential)),

            #[rustfmt::skip]
            ',' |
            ';' => expr_stack[top_of_stack]
                .push(Token::Separator),

            // If an open parentheses is found, add a vec to the stack and make everything add to
            // that stack vec instead.
            '(' => {
                expr_stack.push(Vec::new());
                top_of_stack += 1;
            }
            // If a closing parentheses is found add a token containing the tokens inside the
            // parentheses.
            ')' => {
                // If there was no opening parentheses, treat it as if there was one.
                if top_of_stack == 0 {
                    expr_stack.insert(0, Vec::new());
                    top_of_stack += 1;
                }

                // Add the scope at the top of the stack as a token in the lower scope.
                if let Some(inner) = expr_stack.pop() {
                    top_of_stack -= 1;
                    expr_stack[top_of_stack].push(Token::InnerExpression(inner))
                }
            }
            _ => (),
        }

        let top_len = expr_stack[top_of_stack].len();

        if top_len > 1 {
            match (
                &expr_stack[top_of_stack][top_len - 2],
                &expr_stack[top_of_stack][top_len - 1],
            ) {
                (Token::Number(_), Token::Number(_)) => {
                    return Err(EvalError::InvalidExpression(
                        "Two numbers in a row".to_string(),
                    ))
                }
                (Token::Operator(_), Token::Operator(_)) => {
                    return Err(EvalError::InvalidExpression(
                        "Two operators in a row".to_string(),
                    ))
                }
                (Token::Operator(_), Token::Separator) => {
                    return Err(EvalError::InvalidExpression(
                        "Operator followed by separator".to_string(),
                    ))
                }
                (Token::Separator, Token::Operator(_)) => {
                    return Err(EvalError::InvalidExpression(
                        "Separator followed by operator".to_string(),
                    ))
                }
                (Token::Separator, Token::Separator) => {
                    return Err(EvalError::InvalidExpression(
                        "Two separators in a row".to_string(),
                    ))
                }
                _ => (),
            }
        }
    }

    // If there was no closing parentheses, collapse all inner scopes until one scope is left.
    while top_of_stack > 0 {
        let top_expr = expr_stack.pop().unwrap();
        top_of_stack -= 1;

        expr_stack[top_of_stack].push(Token::InnerExpression(top_expr));
    }

    // Return the list of tokens.
    Ok(expr_stack.pop().unwrap())
}

fn parse_keywords(expression: &[Token]) -> Result<Vec<Token>, EvalError> {
    let mut output = expression.to_vec();

    let inner_expr = expression.iter().enumerate().filter_map(|(i, t)| match t {
        Token::InnerExpression(expr) => Some((i, expr)),
        _ => None,
    });

    for (i, inner) in inner_expr {
        let expr = parse_keywords(inner)?;
        output[i] = Token::InnerExpression(expr);
    }


    let keywords = expression.iter().enumerate().filter_map(|(i, t)| match t {
        Token::Keyword(kword) => Some((i, kword)),
        _ => None,
    });

    let mut offset = 0;

    for (i, kword) in keywords {
        let kword_type = (FUNCTIONS.get(kword), CONSTANTS.get(kword));
        match kword_type {
            (Some(f), None) => {
                let Token::InnerExpression(expr) = &output[i + 1 - offset] else {
                    return Err(EvalError::NoFunctionArguments(kword.to_string()));
                };

                let args = expr
                    .split(|t| matches!(t, Token::Separator))
                    .map(|v| v.to_vec())
                    .collect::<Vec<Vec<Token>>>();

                output[i - offset] = Token::Function((*f, args));
                output.remove(i - offset + 1);
                offset += 1;
            }
            (None, Some(c)) => {
                output[i - offset] = Token::Constant(*c);
            }
            (Some(_), Some(_)) => return Err(EvalError::ProgramIsStupid),
            (None, None) => {
                return Err(EvalError::UnknownKeyword(kword.to_string()));
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        let expression_1 = "123 + 456";
        let tokens_1 = parse_expression(expression_1).unwrap();
        let expression_2 = "7 + 45193";
        let tokens_2 = parse_expression(expression_2).unwrap();

        assert_eq!(tokens_1, vec![Token::Number(123.0), Token::Operator(Operator::Additive(Additive::Add)), Token::Number(456.0)]);
        assert_eq!(tokens_2, vec![Token::Number(7.0), Token::Operator(Operator::Additive(Additive::Add)), Token::Number(45193.0)]);
    }

    #[test]
    fn parse_additive() {
        let expression = "0 + 0 - 0";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(0.0),
                Token::Operator(Operator::Additive(Additive::Add)),
                Token::Number(0.0),
                Token::Operator(Operator::Additive(Additive::Subtract)),
                Token::Number(0.0),
            ]
        )
    }

    #[test]
    fn parse_multiplicative() {
        let expression = "0 * 0 / 1";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(0.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Number(0.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Divide)),
                Token::Number(1.0),
            ]
        )
    }

    #[test]
    fn parse_parentheses() {
        let expression = "()";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(tokens, vec![Token::InnerExpression(vec![])]);
    }

    #[test]
    fn parse_additive_compound() {
        let expression = "123 + 456 - 789";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Operator(Operator::Additive(Additive::Add)),
                Token::Number(456.0),
                Token::Operator(Operator::Additive(Additive::Subtract)),
                Token::Number(789.0),
            ]
        )
    }

    #[test]
    fn parse_multiplicative_compound() {
        let expression = "123 * 456 / 789";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Number(456.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Divide)),
                Token::Number(789.0),
            ]
        )
    }

    #[test]
    fn parse_parentheses_compound() {
        let expression = "123 * (456 + 789)";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::InnerExpression(vec![
                    Token::Number(456.0),
                    Token::Operator(Operator::Additive(Additive::Add)),
                    Token::Number(789.0)
                ])
            ]
        );
    }

    #[test]
    fn parse_parentheses_unopened() {
        let expression = "123 + 456) * 789";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::InnerExpression(vec![
                    Token::Number(123.0),
                    Token::Operator(Operator::Additive(Additive::Add)),
                    Token::Number(456.0),
                ]),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Number(789.0),
            ]
        )
    }

    #[test]
    fn parse_parentheses_unclosed() {
        let expression = "123 * (456 + 789\n";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::InnerExpression(vec![
                    Token::Number(456.0),
                    Token::Operator(Operator::Additive(Additive::Add)),
                    Token::Number(789.0)
                ])
            ]
        )
    }

    #[test]
    fn parse_functions() {
        let expression = "sin(123) + log(2, 16)\n";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Function((
                    Function {
                        name: "sin",
                        argument_count: 1,
                    },
                    vec![vec![Token::Number(123.0)]]
                )),
                Token::Operator(Operator::Additive(Additive::Add)),
                Token::Function((
                    Function {
                        name: "log",
                        argument_count: 2,
                    },
                    vec![vec![Token::Number(2.0)], vec![Token::Number(16.0)]]
                )),
            ]
        )
    }

    #[test]
    fn parse_constants() {
        let expression = "pi e\n";
        let tokens = parse_expression(expression).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Constant(std::f64::consts::PI),
                Token::Constant(std::f64::consts::E),
            ]
        )
    }
}
