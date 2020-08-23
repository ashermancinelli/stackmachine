#[derive(Clone, PartialEq, Copy, Debug)]
#[allow(dead_code)]
#[repr(u16)]
pub enum Op {
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    PrintStr,
    Pop,
    Push,
    PushStr,
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
    Include,
}

pub type Function = fn(&mut Vec<i32>);

#[cfg(test)]
mod function_tests {
    use super::Function;
    use super::Op;
    use crate::stackmachine::StackMachine;

    #[test]
    fn test_function_table() {
        let mut sm = StackMachine::new(2u32);

        sm.function_table.insert(
            "fn".to_string(),
            vec![(Op::Add, None)]
            );

        sm.execute(vec![
            (Op::Const, Some(3i32)),
            (Op::Const, Some(2i32)),
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Call, None), // External adding func
        ]);

        assert_eq!(Some(2 + 3), sm.pop());
    }

    fn sum_n(stack: &mut Vec<i32>) {
        if let Some(mut nargs) = stack.pop() {
            let mut total = 0;
            while nargs > 0 {
                match stack.pop() {
                    Some(v) => {
                        total += v;
                    }
                    None => {
                        stack.push(-1);
                        return;
                    }
                }
                nargs -= 1;
            }
            stack.push(total);
        } else {
            stack.push(-1);
        }
    }

    #[test]
    fn test_ext_function() {
        let mut sm = StackMachine::new(2u32);
        sm.ext_functions = vec![sum_n];
        sm.push(2);
        sm.push(2);
        sm.push(2);
        sm.push(3);
        sm.execute(vec![(Op::CallExt, Some(0))]);
        assert_eq!(sm.last(), Some(6));
    }

    #[test]
    fn test_fn_define() {
        let mut sm = StackMachine::new(2u32);

        sm.execute(vec![
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Function, None), // start function definition
            (Op::Add, None),
            (Op::EndFunction, None),
        ]);

        assert!(sm.function_table.contains_key("fn"));
    }

    #[test]
    fn test_fn_define_call() {
        let mut sm = StackMachine::new(2u32);

        sm.execute(vec![
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Function, None), // start function definition
            (Op::Add, None),
            (Op::EndFunction, None),
            (Op::Const, Some(3i32)),
            (Op::Const, Some(2i32)),
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Call, None), // External adding func
        ]);

        assert_eq!(sm.last(), Some(3 + 2));
    }
}
