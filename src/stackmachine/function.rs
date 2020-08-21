pub mod op {
    #[derive(Clone, PartialEq, Debug)]
    #[allow(dead_code)]
    #[repr(u16)]
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
        Not,
        GT,
        LT,
        GTE,
        LTE,
        r#Eq,
        EndIf,
        Function,
        EndFunction,
        GetPid,
        Child,
        Debug,
    }
}

#[derive(Clone)]
pub struct Function {
    pub code: Vec<(op::Operation, Option<i32>)>,
}

impl Function {

    // In the future, I need to add more configuration options and struct 
    // members for creating a
    // function to better replicate a function table.
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
