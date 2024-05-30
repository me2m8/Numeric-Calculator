use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Function {
    pub name: &'static str,
    pub argument_count: usize,
}

type FuncMap = HashMap<String, Function>;
type ConstMap = HashMap<String, f64>;

pub static FUNCTIONS: Lazy<FuncMap, fn() -> FuncMap> = Lazy::new(|| {
    HashMap::from(
        [
            Function {
                name: "sin",
                argument_count: 1,
            },
            Function {
                name: "cos",
                argument_count: 1,
            },
            Function {
                name: "tan",
                argument_count: 1,
            },
            Function {
                name: "arcsin",
                argument_count: 1,
            },
            Function {
                name: "arccos",
                argument_count: 1,
            },
            Function {
                name: "arctan",
                argument_count: 1,
            },
            Function {
                name: "log",
                argument_count: 2,
            },
            Function {
                name: "ln",
                argument_count: 1,
            },
            Function {
                name: "sqrt",
                argument_count: 1,
            },
        ]
        .map(|f| (f.name.to_string(), f)),
    )
});

pub static CONSTANTS: Lazy<ConstMap, fn() -> ConstMap> = Lazy::new(|| {
    HashMap::from([
        ("e".to_string(), std::f64::consts::E),
        ("pi".to_string(), std::f64::consts::PI),
    ])
});
