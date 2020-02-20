pub mod op {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Operation {
        Const,
        Add,
        Sub,
        Mul,
        Div,
        Print,
        Pop,
        Push,
        Noop,
        Block,
        Loop,
        Return,
        Break,
        CallExt,
        Call,
        Fork,
        If,
        Else,
        IfNot,
        IfGT,
        IfLT,
        IfGTE,
        IfLTE,
        IfEq,
        EndIf,
        Function,
        EndFunction,
        GetPid,
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
