pub mod op {
    #[derive(Clone)]
    pub enum Operation {
        Const,
        Add,
        Sub,
        Mul,
        Div,
        Print,
        Noop,
        Block,
        Loop,
        Return,
        Break,
        CallExt,
        Call,
    }
}

#[derive(Clone)]
pub struct Function {
    pub code: Vec<(op::Operation, Option<u32>)>,
}

impl Function {
    pub fn new(code: Vec<(op::Operation, Option<u32>)>) -> Function {
        return Function {
            code: code,
        };
    }
}

#[cfg(test)]
mod function_tests {
    use super::op::Operation;
    use super::Function;

    #[test]
    fn test_function_new() {
        let f = Function::new(vec![
            (Operation::Const, Some(3)),
        ]);
    }
}
