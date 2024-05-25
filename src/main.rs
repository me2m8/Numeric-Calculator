// read the input
// tokenize
// construct asbtract syntax tree
// do the calculation

use std::{io, io::Write};

use errors::EvalError;

use crate::eval::evaluate_ast;

mod ast;
mod errors;
mod eval;
mod tokenize;

fn main() -> io::Result<()> {
    while let Ok(input) = get_input("Enter an expression: ") {
        if input.is_empty() {
            break;
        }

        let result = evaluate_expression(&input);
        match result {
            Ok(result) => println!("Your expression evaluated to: {}", result),
            Err(EvalError::InvalidExpression) => println!("Invalid expression, please try again."),
            Err(EvalError::InvalidCharacter(c)) => println!("Invalid character: {}", c),
        }
    }

    Ok(())
}

/// Helper function to get input from stdin with a query.
fn get_input(query: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    print!("{query}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buffer)?;

    // Remove the newline character from the end of the buffer and return the string
    Ok(buffer.strip_suffix('\n').unwrap_or(&buffer).to_string())
}

/// Evaluates the given expression
fn evaluate_expression(expr: &str) -> Result<f64, EvalError> {
    let tokens = tokenize::parse_expression(expr)?;
    let ast = ast::get_ast(&tokens)?;
    Ok(evaluate_ast(ast))
}
