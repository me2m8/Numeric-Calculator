// read the input
// tokenize
// construct asbtract syntax tree
// do the calculation

use std::{io, io::Write};

#[derive(Debug)]
enum Token {
    Number(i32),
    Operator(Operator),
    Bracket(Bracket),
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Bracket {
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lsquiggly,
    Rsquiggly,
}

fn main() -> io::Result<()> {
    let input = get_input("Enter the expression: ")?;

    let tokens = parse_expression(&input);

    dbg!(tokens);

    Ok(())
}

fn get_input(query: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();

    print!("{query}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buffer).unwrap();

    Ok(buffer)
}

fn parse_expression(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut num = String::new();

    for c in expression.chars() {
        if let '0'..='9' = c {
            num.push(c);
            continue
        }

        if !num.is_empty() {
            let n = num.parse::<i32>().unwrap();
            tokens.push(Token::Number(n));
            num.clear();
        }

        match c {
            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            '*' => tokens.push(Token::Operator(Operator::Multiply)),
            '/' => tokens.push(Token::Operator(Operator::Divide)),
            '(' => tokens.push(Token::Bracket(Bracket::Lparen)),
            ')' => tokens.push(Token::Bracket(Bracket::Rparen)),
            '[' => tokens.push(Token::Bracket(Bracket::Lsquare)),
            ']' => tokens.push(Token::Bracket(Bracket::Rsquare)),
            '{' => tokens.push(Token::Bracket(Bracket::Lsquiggly)),
            '}' => tokens.push(Token::Bracket(Bracket::Rsquiggly)),
            _ => (),
        }
    }

    tokens
}

// 2 + 3 * 4 * 5

// 4 + 5 * (1 + 2 * 3 + 4)
