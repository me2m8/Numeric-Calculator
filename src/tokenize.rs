#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(Operator),
    InnerExpression(Vec<Token>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    Additive(Additive),
    Multiplicative(Multiplicative),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Additive {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Multiplicative {
    Multiply,
    Divide,
}

pub fn parse_expression(expression: &str) -> Vec<Token> {
    let mut expr_stack: Vec<Vec<Token>> = vec![Vec::new()];
    let mut top_of_stack;
    let mut num = String::new();

    for c in expression.chars() {
        top_of_stack = expr_stack.len() - 1;

        if let '0'..='9' = c {
            num.push(c);
            continue;
        }

        if !num.is_empty() {
            let n = num.parse::<i32>().unwrap();
            expr_stack[top_of_stack].push(Token::Number(n));
            num.clear();
        }

        match c {
            '+' => {
                expr_stack[top_of_stack].push(Token::Operator(Operator::Additive(Additive::Plus)))
            }
            '-' => {
                expr_stack[top_of_stack].push(Token::Operator(Operator::Additive(Additive::Minus)))
            }

            '*' => expr_stack[top_of_stack].push(Token::Operator(Operator::Multiplicative(
                Multiplicative::Multiply,
            ))),
            '/' => expr_stack[top_of_stack].push(Token::Operator(Operator::Multiplicative(
                Multiplicative::Divide,
            ))),

            '(' => {
                expr_stack.push(Vec::new());
            }
            ')' => {
                if expr_stack.len() == 1 {
                    expr_stack.insert(0, Vec::new());
                }

                if let Some(inner) = expr_stack.pop() {
                    top_of_stack = expr_stack.len() - 1;
                    expr_stack[top_of_stack].push(Token::InnerExpression(inner))
                }
            }
            // '[' => tokens.push(Token::Bracket(Bracket::Square(SquareBracket::Left))),
            // ']' => tokens.push(Token::Bracket(Bracket::Square(SquareBracket::Right))),
            // '{' => tokens.push(Token::Bracket(Bracket::Squiggly(SquigglyBracket::Left))),
            // '}' => tokens.push(Token::Bracket(Bracket::Squiggly(SquigglyBracket::Left))),
            _ => (),
        }
    }

    top_of_stack = dbg!(expr_stack.len() - 1);

    if !num.is_empty() {
        let n = num.parse::<i32>().unwrap();
        expr_stack[top_of_stack].push(Token::Number(n));
        num.clear();
    }

    while expr_stack.len() != 1 {
        let top_expr = expr_stack.pop().unwrap();

        top_of_stack = dbg!(expr_stack.len() - 1);
        expr_stack[top_of_stack].push(Token::InnerExpression(top_expr));
    }

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

        assert_eq!(tokens_1, vec![Token::Number(123), Token::Number(456)]);
        assert_eq!(tokens_2, vec![Token::Number(7), Token::Number(45193)]);
    }

    #[test]
    fn parse_additive() {
        let expression = "+ -";
        let tokens = parse_expression(expression);

        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Additive(Additive::Plus)),
                Token::Operator(Operator::Additive(Additive::Minus))
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
                Token::Number(123),
                Token::Operator(Operator::Additive(Additive::Plus)),
                Token::Number(456),
                Token::Operator(Operator::Additive(Additive::Minus)),
                Token::Number(789),
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
                Token::Number(123),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Number(456),
                Token::Operator(Operator::Multiplicative(Multiplicative::Divide)),
                Token::Number(789),
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
                Token::Number(123),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::InnerExpression(vec![
                    Token::Number(456),
                    Token::Operator(Operator::Additive(Additive::Plus)),
                    Token::Number(789)
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
                    Token::Number(123),
                    Token::Operator(Operator::Additive(Additive::Plus)),
                    Token::Number(456),
                ]),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::Number(789),
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
                Token::Number(123),
                Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                Token::InnerExpression(vec![
                    Token::Number(456),
                    Token::Operator(Operator::Additive(Additive::Plus)),
                    Token::Number(789)
                ])
            ]
        )

    }
}
