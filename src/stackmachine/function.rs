pub mod op {
    #[derive(Clone, PartialEq, Debug)]
    #[allow(dead_code)]
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
        Child,
    }
}

#[derive(Clone)]
pub struct Function {
    pub code: Vec<(op::Operation, Option<i32>)>,
}

impl Function {
    pub fn new(code: Vec<(op::Operation, Option<i32>)>) -> Function {
        return Function { code: code };
    }
}

#[cfg(test)]
mod function_tests {
    use super::op::Operation as Op;
    use super::Function;
    use crate::stackmachine::StackMachine;

    #[test]
    fn test_function_new() {
        let mut sm = StackMachine::new(2u32);
        sm.function_table = vec![Function::new(vec![(Op::Const, Some(3))])];

        sm.execute(vec![(Op::Call, Some(0))]);

        assert_eq!(Some(3), sm.pop());
    }
}
