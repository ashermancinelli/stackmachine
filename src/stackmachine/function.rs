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

pub type Function = fn(&mut Vec<i32>);

#[cfg(test)]
mod function_tests {
    use super::Op;
    use super::Function;
    use crate::stackmachine::StackMachine;

    #[test]
    fn test_function_table() {
        let mut sm = StackMachine::new(2u32);
        sm.function_table = vec![vec![(Op::Const, Some(3))]];

        sm.execute(vec![(Op::Call, Some(0))]);

        assert_eq!(Some(3), sm.pop());
    }

    fn sumN(stack: &mut Vec<i32>) {
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
        }
        else {
            stack.push(-1);
        }
    }

    #[test]
    fn test_ext_function() {
        let mut sm = StackMachine::new(2u32);
        sm.ext_functions = vec![sumN];
        sm.push(2);
        sm.push(2);
        sm.push(2);
        sm.push(3);
        sm.execute(vec![
            (Op::CallExt, Some(0)),
        ]);
        assert_eq!(sm.last(), Some(6));
    }
}
