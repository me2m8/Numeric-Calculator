use crate::tokenize::*;
use crate::errors::*;

// The Abstract Syntax Tree represent the order of operations. Operators farther down will be evaluated first.
#[derive(Debug, Clone)]
pub struct ASTNode {
    pub token: Token,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl ASTNode {
    pub fn pop_left(&mut self) -> Option<Box<ASTNode>> {
        self.left.take()
    }

    pub fn pop_right(&mut self) -> Option<Box<ASTNode>> {
        self.right.take()
    }
}

/// creates the AST from the tokens.
pub fn get_ast(tokens: &[Token]) -> Result<ASTNode, EvalError> {
    construct_ast(tokens)
}

/// Recursively prases the tokens into the AST. 
fn construct_ast(tokens: &[Token]) -> Result<ASTNode, EvalError> {
    // Base cases
    match tokens.len() {
        // This is only possible in the case you have a leading or trailing operator.
        0 => {
            return Err(EvalError::InvalidExpression)
        }
        // If there is only one token left, its either an inner expression or a number.
        1 => {
            return match &tokens[0] {
                Token::Number(n) => Ok(ASTNode {
                    token: Token::Number(*n),
                    left: None,
                    right: None,
                }),
                Token::InnerExpression(expr) => construct_ast(expr),
                _ => {
                    dbg!(tokens);
                    return Err(EvalError::InvalidExpression)
                }
            }
        }
        // If there are 2 tokens left, the only valid pair is a number followed by a parenthesised
        // expression.
        2 => {
            if let (Token::Number(n), Token::InnerExpression(expr)) = (&tokens[0], &tokens[1]) {
                let left = Some(Box::new(ASTNode {
                    token: Token::Number(*n),
                    left: None,
                    right: None,
                }));
                let right = Some(Box::new(construct_ast(expr)?));
                return Ok(ASTNode {
                    token: Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                    left,
                    right,
                });
            } else {
                return Err(EvalError::InvalidExpression);
            }
        }
        _ => {}
    };

    // Additive operators are placed into the AST first, which means they'll be evaluated last. 
    if let Some((i, Token::Operator(Operator::Additive(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Additive(_))))
    {
        let left = Some(Box::new(construct_ast(&tokens[0..i])?));
        let right = Some(Box::new(construct_ast(&tokens[i + 1..])?));
        return Ok(ASTNode {
            token: Token::Operator(Operator::Additive(*op)),
            left,
            right,
        });
    }

    // Multiplicative operators are placed into the AST last which means they'll be evaluated first
    if let Some((i, Token::Operator(Operator::Multiplicative(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Multiplicative(_))))
    {
        let left = Some(Box::new(construct_ast(&tokens[0..i])?));
        let right = Some(Box::new(construct_ast(&tokens[i + 1..])?));
        return Ok(ASTNode {
            token: Token::Operator(Operator::Multiplicative(*op)),
            left,
            right,
        });
    }

    // This should be impossible
    dbg!(tokens);
    unreachable!("Invalid expression")
}
