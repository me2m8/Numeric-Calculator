use crate::{ast::{self, *}, errors::EvalError, tokenize::{self, *}};

/// Evaluates the given expression
pub fn evaluate_expression(expr: &str) -> Result<f64, EvalError> {
    let tokens = tokenize::parse_expression(expr)?;
    let ast = ast::get_ast(&tokens)?;
    Ok(evaluate_ast(ast))
}

/// Recursively collapses the ast and evaluated each node, then returns the resulting number.
pub fn evaluate_ast(mut ast: ASTNode) -> f64 {
    if let Token::Number(n) | Token::Constant(n) = ast.token {
        return n;
    }

    if let Token::Function(ref func) = ast.token {
        let mut args = vec![];
        for child in ast.children {
            args.push(evaluate_ast(child));
        }

        match func.0.name {
            "sin" => return args[0].sin(),
            "cos" => return args[0].cos(),
            "tan" => return args[0].tan(),
            "arcsin" => return args[0].asin(),
            "arccos" => return args[0].acos(),
            "arctan" => return args[0].atan(),
            "log" => return args[1].log(args[0]),
            "ln" => return args[0].ln(),
            "sqrt" => return args[0].sqrt(),
            _ => unreachable!("Unknown function"),
        }
    }

    if let Token::Operator(ref op) = ast.token {

        let left = evaluate_ast(ast.children.remove(0));
        let right= evaluate_ast(ast.children.remove(0));

        return match op {
            Operator::Additive(op) => match op {
                Additive::Add => left + right,
                Additive::Subtract => left - right,
            },
            Operator::Multiplicative(op) => match op {
                Multiplicative::Multiply => left * right,
                Multiplicative::Divide => left / right,
            },
            Operator::Exponential => left.powf(right),
        }
    }

    unreachable!("Invalid AST")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin_pi() {
        let expression = "sin(pi) + 2 * 3";

         let res = evaluate_expression(expression).unwrap();

         assert_eq!(res, 6.0);
    }

    #[test]
    fn test_trig_inverse() {
        let expression = "arcsin(sin(1)) + arccos(cos(1)) + arctan(tan(1))";

        let res = evaluate_expression(expression).unwrap();

        assert_eq!(res, 3.0);
    }

    #[test] 
    fn test_log() {
        let expression = "log(10, 100)";

        let res = evaluate_expression(expression).unwrap();

        assert_eq!(res, 2.0);
    }

    #[test]
    fn test_sqrt() {
        let expression = "sqrt(4)";

        let res = evaluate_expression(expression).unwrap();

        assert_eq!(res, 2.0);
    }

    #[test]
    fn test_complex_expression() {
        let expression = "cos(pi) + 2 * 3 + 4 / 2";

        let res = evaluate_expression(expression).unwrap();

        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_complex_expression_2() {
        let expression = "(cos(pi) + 2)(sin(3pi/6)) + sin(e)^2 + cos(e)^2";

        let res = evaluate_expression(expression).unwrap();

        assert_eq!(res, 2.0);
    }
}
