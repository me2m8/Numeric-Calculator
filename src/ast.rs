use crate::errors::*;
use crate::tokenize::*;

// The Abstract Syntax Tree represent the order of operations. Operators farther down will be evaluated first.
#[derive(Debug, Clone)]
pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

/// creates the AST from the tokens.
pub fn get_ast(tokens: &[Token]) -> Result<ASTNode, EvalError> {
    construct_ast(tokens)
}

/// Recursively prases the tokens into the AST.
fn construct_ast(tokens: &[Token]) -> Result<ASTNode, EvalError> {
    if tokens.len() == 1 {
        match tokens[0] {
            Token::Number(n) => {
                return Ok(ASTNode {
                    token: Token::Number(n),
                    children: vec![],
                })
            }
            Token::Constant(n) => {
                return Ok(ASTNode {
                    token: Token::Constant(n),
                    children: vec![],
                })
            }
            Token::InnerExpression(ref inner) => return construct_ast(inner),
            Token::Function(ref func) => {
                let (function, args) = func.clone();
                if args.len() != function.argument_count {
                    return Err(EvalError::InvalidArgumentCount {
                        expected: function.argument_count,
                        got: args.len(),
                    });
                }

                let mut children = vec![];

                for arg in args {
                    let ast = construct_ast(&arg)?;
                    children.push(ast);
                }

                return Ok(ASTNode {
                    token: Token::Function(func.clone()),
                    children,
                });
            }
            _ => {
                dbg!(&tokens[0]);
                return Err(EvalError::InvalidExpression(
                    "Invalid expression".to_string(),
                ))
            }
        }
    }

    // Additive operators are placed into the AST first, which means they'll be evaluated last.
    if let Some((i, Token::Operator(Operator::Additive(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Additive(_))))
    {
        let left = construct_ast(&tokens[..i])?;
        let right = construct_ast(&tokens[i + 1..])?;
        return Ok(ASTNode {
            token: Token::Operator(Operator::Additive(*op)),
            children: vec![left, right],
        });
    }

    // Multiplicative operators are placed into the AST last which means they'll be evaluated first
    if let Some((i, Token::Operator(Operator::Multiplicative(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Multiplicative(_))))
    {
        let left = construct_ast(&tokens[..i])?;
        let right = construct_ast(&tokens[i + 1..])?;
        return Ok(ASTNode {
            token: Token::Operator(Operator::Multiplicative(*op)),
            children: vec![left, right],
        });
    }

    if let Some((i, Token::Operator(Operator::Exponential))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Exponential)))
    {
        let left = construct_ast(&tokens[..i])?;
        let right = construct_ast(&tokens[i + 1..])?;
        return Ok(ASTNode {
            token: Token::Operator(Operator::Exponential),
            children: vec![left, right],
        });
    }

    // At this point, only implicit multiplication is left.
    let left = construct_ast(&tokens[..1])?;
    let right = construct_ast(&tokens[1..])?;
    return Ok(ASTNode {
        token: Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
        children: vec![left, right],
    });
}
