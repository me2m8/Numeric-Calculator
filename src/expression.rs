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

impl Expression {
    pub fn new(raw: String) -> Expression {
        let tokens = parse_expression(&raw);
        Expression {
            raw,
            tokens,
            ast: None,
        }
    }

    pub fn get_ast(&self) {}
}

// Recursively construct ast
// find + or -
//      construct_ast with values on the left
//      construct_ast with tokens on the right
// find * or /
//      construct_ast with values on the left
//      construct_ast with values on the right
