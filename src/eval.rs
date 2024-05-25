use crate::{ast::*, tokenize::*};

/// Recursively collapses the ast and evaluated each node, then returns the resulting number. 
pub fn evaluate_ast(mut ast: ASTNode) -> f64 {
    if let Token::Number(n) = ast.token {
        return n
    }

    let Token::Operator(op) = ast.token else {
        unreachable!()
    };
    let left = *ast.pop_left().unwrap();
    let right = *ast.pop_right().unwrap();

    match op {
        Operator::Additive(Additive::Add) => evaluate_ast(left) + evaluate_ast(right),
        Operator::Additive(Additive::Subtract) => evaluate_ast(left) - evaluate_ast(right),
        Operator::Multiplicative(Multiplicative::Multiply) => {
            evaluate_ast(left) * evaluate_ast(right)
        }
        Operator::Multiplicative(Multiplicative::Divide) => {
            evaluate_ast(left) / evaluate_ast(right)
        }
    }
}
