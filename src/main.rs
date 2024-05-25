// read the input
// tokenize
// construct asbtract syntax tree
// do the calculation

use std::{io, io::Write};

mod tokenize;
mod ast;

fn main() -> io::Result<()> {
    let input = get_input("Enter the expression: ")?;

    let tokens = tokenize::parse_expression(&input);

    dbg!(&tokens);

    let ast = ast::get_ast(&tokens);

    dbg!(ast.root);

    Ok(())
}

fn get_input(query: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    print!("{query}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buffer).unwrap();

    Ok(buffer)
}

// 2 + 3 * 4 * 5

// 4 + 5 * (1 + 2 * 3 + 4) + 6
//
// 4 +                     + 6
//     5 *                
//         (             )
//          1 +       + 4
//              2 * 3
