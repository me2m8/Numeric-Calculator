/// Enum representing a symbol/token in the expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),

    /// Holds the tokens from an expression inside parentheses.
    InnerExpression(Vec<Token>),
}

/// Enum representing an arithmetic operator.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    Additive(Additive),
    Multiplicative(Multiplicative),
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

/// Parses the expression string into an an array of tokens, representing numbers, operators and
/// expressions inside parentheses
pub fn parse_expression(expression: &str) -> Vec<Token> {
    // A space is appended at the end so that the end doesnt end abruptly, allowing numbers to be
    // properly parsed. 
    let expression = String::from(expression) + "\0";

    // the expression stack is used to keep track of the current scope. expressions inside parentheses
    // are parsed into their own token to make constructing the ast easier. 
    let mut expr_stack: Vec<Vec<Token>> = vec![Vec::new()];
    let mut top_of_stack = 0;

    // Vec used to store digits for parsing numbers
    let mut num = String::new();

    for c in expression.chars() {
        // If a number has been found, add the char to the num vec and continue.
        // Repeat until another symbol is found to get the entire number.
        if let '0'..='9' = c {
            num.push(c);
            continue;
        }

        // When something else is found, parse the digits into a float and clear the num vec.
        if !num.is_empty() {
            let n = num.parse::<f64>().unwrap();
            expr_stack[top_of_stack].push(Token::Number(n));
            num.clear();
        }

        // Matches the different symbols and either adds them to the current scope.
        match c {
            '+' => {
                expr_stack[top_of_stack].push(Token::Operator(Operator::Additive(Additive::Add)))
            }
            '-' => expr_stack[top_of_stack]
                .push(Token::Operator(Operator::Additive(Additive::Subtract))),

            '*' => expr_stack[top_of_stack].push(Token::Operator(Operator::Multiplicative(
                Multiplicative::Multiply,
            ))),
            '/' => expr_stack[top_of_stack].push(Token::Operator(Operator::Multiplicative(
                Multiplicative::Divide,
            ))),

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
    }

    // If there was no closing parentheses, collapse all inner scopes until one scope is left.
    while top_of_stack > 0 {
        let top_expr = expr_stack.pop().unwrap();
        top_of_stack -= 1;

        expr_stack[top_of_stack].push(Token::InnerExpression(top_expr));
    }

    // Return the list of tokens. 
    expr_stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        let expression_1 = "123 456";
        let tokens_1 = parse_expression(expression_1);
        let expression_2 = "7 45193";
        let tokens_2 = parse_expression(expression_2);

        assert_eq!(tokens_1, vec![Token::Number(123.0), Token::Number(456.0)]);
        assert_eq!(tokens_2, vec![Token::Number(7.0), Token::Number(45193.0)]);
    }

    #[test]
    fn parse_additive() {
        let expression = "+ -";
        let tokens = parse_expression(expression);

        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Additive(Additive::Add)),
                Token::Operator(Operator::Additive(Additive::Subtract))
            ]
        )
    }

    #[test]
    fn parse_multiplicative() {
        let expression = "* /";
        let tokens = parse_expression(expression);

        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Operator(Operator::Multiplicative(Multiplicative::Divide))
            ]
        )
    }

    #[test]
    fn parse_parentheses() {
        let expression = "()";
        let tokens = parse_expression(expression);

        assert_eq!(tokens, vec![Token::InnerExpression(vec![])]);
    }

    #[test]
    fn parse_additive_compound() {
        let expression = "123 + 456 - 789";
        let tokens = parse_expression(expression);

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
        let tokens = parse_expression(expression);

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
        let tokens = parse_expression(expression);

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
        let tokens = parse_expression(expression);

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
        let expression = "123 * (456 + 789";
        let tokens = parse_expression(expression);

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
}
