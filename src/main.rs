// read the input
// tokenize
// construct asbtract syntax tree
// do the calculation

use std::{io, io::Write};

use crate::eval::evaluate_ast;

mod tokenize;
mod ast;
mod eval;

fn main() -> io::Result<()> {
    while let Ok(input) = get_input("Enter an expression: ") {
        if input.is_empty() {
            break
        }

        let result = evaluate_expression(&input);
        println!("Your expression evaluated to: {result}");
    }

    Ok(())
}

/// Helper function to get input from stdin with a query.
fn get_input(query: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    print!("{query}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer)
}

/// Evaluates the given expression
fn evaluate_expression(expr: &str) -> f64 {
    let tokens = tokenize::parse_expression(expr);
    let ast = ast::get_ast(&tokens);
    evaluate_ast(ast)
}

