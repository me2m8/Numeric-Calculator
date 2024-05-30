// read the input
// tokenize
// construct asbtract syntax tree
// do the calculation

use std::{io, io::Write};

use errors::EvalError;

use crate::eval::*;

mod ast;
mod errors;
mod eval;
mod tokenize;
mod keywords;

fn main() -> io::Result<()> {
    while let Ok(input) = get_input("Enter an expression: ") {
        if input.is_empty() {
            break;
        }

        let result = evaluate_expression(&input);
        match result {
            Ok(result) => println!("Your expression evaluated to: {}", result),
            Err(EvalError::InvalidExpression(_)) => println!("Invalid expression, please try again."),
            Err(EvalError::InvalidCharacter(c)) => println!("Invalid character: {:#?}", c),
            Err(EvalError::UnknownKeyword(k)) => println!("Unknown keyword: {:#?}", k),
            Err(EvalError::NoFunctionArguments(k)) => println!("No function variablesfor function: {:#?}", k),
            Err(EvalError::InvalidArgumentCount { expected, got }) => println!("Invalid argument count, expected: {:#?}, got: {:#?}", expected, got),
            Err(EvalError::ProgramIsStupid) => println!("there is a collision between a function and a constant, program is stupid"),
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
    Ok(buffer)
}
