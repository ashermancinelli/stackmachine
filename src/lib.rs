pub mod stackmachine;

#[cfg(test)]
pub mod tests {

    use super::stackmachine::Function;
    use super::stackmachine::Op;
    use super::stackmachine::StackMachine;

    #[test]
    pub fn test_add() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(1i32)),
            (Op::Const, Some(1i32)),
            (Op::Add, None),
        ]);

        assert_eq!(Some(2), sm.pop());
    }

    #[test]
    pub fn test_sub() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(1i32)),
            (Op::Const, Some(1i32)),
            (Op::Sub, None),
        ]);

        assert_eq!(Some(0), sm.pop());
    }

    #[test]
    pub fn test_mul() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(2i32)),
            (Op::Const, Some(3i32)),
            (Op::Mul, None),
        ]);

        assert_eq!(Some(6), sm.pop());
    }

    #[test]
    pub fn test_div() {
        let mut sm = StackMachine::new(2u32.pow(8));
        sm.execute(vec![
            (Op::Const, Some(6i32)),
            (Op::Const, Some(3i32)),
            (Op::Div, None),
        ]);

        assert_eq!(Some(0), sm.pop());
    }

    #[test]
    pub fn test_call() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.function_table = vec![Function::new(vec![(Op::Add, None)])];

        sm.execute(vec![
            (Op::Const, Some(6i32)),
            (Op::Const, Some(3i32)),
            (Op::Call, Some(0)), // External adding func
        ]);

        assert_eq!(Some(9), sm.pop());
    }

    #[test]
    fn test_fork() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Const, Some(6i32)),
            (Op::Const, Some(3i32)),
            (Op::Fork, None),
            (Op::Child, None),
        ]);

        assert_eq!(Some(0), sm.pop());
    }

    /*
    #[test]
    fn test_fork_branch() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Fork,      None), // Fork should push `0` to parent
            (Op::GetPid, None),
            (Op::If, None),
            (Op::Const, Some(1)),
            (Op::Else, None),
            (Op::Const,     Some(2)), // Should be called
            (Op::EndIf, None),
        ]);

        assert_eq!(Some(2), sm.pop());
    }
    */

    #[test]
    fn test_if_true() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Const, Some(1i32)),
            (Op::If, None),
            (Op::Const, Some(3i32)),
            (Op::EndIf, None),
        ]);

        assert_eq!(3, sm.pop().unwrap());
    }

    #[test]
    fn test_if_false() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Const, Some(7i32)),
            (Op::Const, Some(0i32)),
            (Op::If, None),
            (Op::Const, Some(3i32)),
            (Op::EndIf, None),
        ]);

        assert_eq!(Some(7i32), sm.pop());
    }

    #[test]
    fn test_if_true_nested() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Const, Some(1i32)),
            (Op::If, None),
            (Op::Const, Some(1i32)),
            (Op::If, None),
            (Op::Const, Some(7i32)),
            (Op::EndIf, None),
            (Op::EndIf, None),
        ]);

        assert_eq!(Some(7i32), sm.pop());
    }

    #[test]
    fn test_if_false_nested() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Const, Some(1i32)),
            (Op::If, None),
            (Op::Const, Some(3i32)),
            (Op::Const, Some(0i32)),
            (Op::If, None),
            (Op::Const, Some(7i32)),
            (Op::EndIf, None),
            (Op::EndIf, None),
        ]);

        assert_eq!(Some(3i32), sm.pop());
    }
}
