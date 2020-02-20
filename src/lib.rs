
mod stackmachine;

#[cfg(test)]
pub mod tests {

    use super::stackmachine::StackMachine;
    use super::stackmachine::Op;
    use super::stackmachine::Function;

    #[test]
    pub fn test_add() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(1u32)),
            (Op::Const, Some(1u32)),
            (Op::Add,   None),
        ]);

        assert_eq!(Some(2), sm.pop());
    }

    #[test]
    pub fn test_sub() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(1u32)),
            (Op::Const, Some(1u32)),
            (Op::Sub,   None),
        ]);

        assert_eq!(Some(0), sm.pop());
    }

    #[test]
    pub fn test_mul() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(2u32)),
            (Op::Const, Some(3u32)),
            (Op::Mul,   None),
        ]);

        assert_eq!(Some(6), sm.pop());
    }

    #[test]
    pub fn test_div() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(6u32)),
            (Op::Const, Some(3u32)),
            (Op::Div,   None),
        ]);

        assert_eq!(Some(0), sm.pop());
    }

    #[test]
    pub fn test_call() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.function_table = vec![
            Function::new(vec![
                (Op::Add,   None),
            ]),
        ];

        sm.execute(vec![
            (Op::Const,     Some(6u32)),
            (Op::Const,     Some(3u32)),
            (Op::CallExt,   Some(0)),
            (Op::Call,      Some(0)),       // External adding func
        ]);

        assert_eq!(Some(9), sm.pop());
    }

    #[test]
    fn test_fork() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.execute(vec![
            (Op::Const,     Some(6u32)),
            (Op::Const,     Some(3u32)),
            (Op::Fork,      None),
        ]);

        assert_eq!(0, sm.pop().unwrap());
    }

    #[test]
    fn test_if_true() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.execute(vec![
            (Op::Const,     Some(1u32)),
            (Op::If,        None),
            (Op::Const,     Some(3u32)),
            (Op::EndIf,     None),
        ]);

        assert_eq!(3, sm.pop().unwrap());
    }

    #[test]
    fn test_if_false() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.execute(vec![
            (Op::Const,     Some(7u32)),
            (Op::Const,     Some(0u32)),
            (Op::If,        None),
            (Op::Const,     Some(3u32)),
            (Op::EndIf,     None),
        ]);

        assert_eq!(Some(7u32), sm.pop());
    }

    #[test]
    fn test_if_true_nested() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.execute(vec![
            (Op::Const,     Some(1u32)),
            (Op::If,        None),
            (Op::Const,     Some(1u32)),
            (Op::If,        None),
            (Op::Const,     Some(7u32)),
            (Op::EndIf,     None),
            (Op::EndIf,     None),
        ]);

        assert_eq!(Some(7u32), sm.pop());
    }

    #[test]
    fn test_if_false_nested() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.ext_functions = vec![
            Box::new(| s: &StackMachine | {
                println!("Stack is {:?}", s.stack);
            }),
        ];

        sm.execute(vec![
            (Op::Const,     Some(1u32)),
            (Op::If,        None),
            (Op::Const,     Some(3u32)),
            (Op::Const,     Some(0u32)),
            (Op::If,        None),
            (Op::Const,     Some(7u32)),
            (Op::EndIf,     None),
            (Op::EndIf,     None),
        ]);

        assert_eq!(Some(3u32), sm.pop());
    }
}
