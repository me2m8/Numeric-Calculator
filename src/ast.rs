use crate::tokenize::*;

#[derive(Debug)]
pub struct Expression {
    pub raw: String,
    pub tokens: Vec<Token>,
    pub ast: Option<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub token: Token,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub struct AST {
    pub root: ASTNode,
}

pub fn get_ast(tokens: &[Token]) -> AST {
    let root = construct_ast(tokens);
    AST { root }
}

fn construct_ast(tokens: &[Token]) -> ASTNode {
    match tokens.len() {
        0 => {
            panic!("Invalid expression");
        }
        1 => {
            return match &tokens[0] {
                Token::Number(n) => ASTNode {
                    token: Token::Number(*n),
                    left: None,
                    right: None,
                },
                Token::InnerExpression(expr) => construct_ast(expr),
                _ => {
                    dbg!(tokens);
                    panic!("Invalid expression")
                }
            }
        }
        2 => {
            if let (Token::Number(n), Token::InnerExpression(expr)) = (&tokens[0], &tokens[1]) {
                let left = Some(Box::new(ASTNode {
                    token: Token::Number(*n),
                    left: None,
                    right: None,
                }));
                let right = Some(Box::new(construct_ast(expr)));
                return ASTNode {
                    token: Token::Operator(Operator::Multiplicative(Multiplicative::Multiply)),
                    left,
                    right,
                };
            } else {
                panic!("Invalid expression")
            }
        }
        _ => {}
    };

    if let Some((i, Token::Operator(Operator::Additive(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Additive(_))))
    {
        let left = Some(Box::new(construct_ast(&tokens[0..i])));
        let right = Some(Box::new(construct_ast(&tokens[i + 1..])));
        return ASTNode {
            token: Token::Operator(Operator::Additive(*op)),
            left,
            right,
        };
    }

    if let Some((i, Token::Operator(Operator::Multiplicative(op)))) = tokens
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Token::Operator(Operator::Multiplicative(_))))
    {
        let left = Some(Box::new(construct_ast(&tokens[0..i])));
        let right = Some(Box::new(construct_ast(&tokens[i + 1..])));
        return ASTNode {
            token: Token::Operator(Operator::Multiplicative(*op)),
            left,
            right,
        };
    }

    dbg!(tokens);
    panic!("Invalid expression")
}
