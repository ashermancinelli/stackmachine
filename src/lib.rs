pub mod stackmachine;

#[cfg(test)]
pub mod tests {

    use super::stackmachine::builder::Builder;
    use super::stackmachine::function::Function;
    use super::stackmachine::function::Op;
    use super::stackmachine::reader;
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

        sm.function_table = vec![vec![(Op::Add, None)]];

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

    #[test]
    #[ignore]
    fn test_fork_branch() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![
            (Op::Fork, None), // Fork should push `0` to parent
            (Op::Child, None),
            (Op::If, None),
            (Op::Const, Some(1)),
            (Op::Else, None),
            (Op::Const, Some(2)), // Should be called
            (Op::EndIf, None),
        ]);

        assert_eq!(Some(2), sm.pop());
    }

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
            (Op::Const, Some(0i32)),
            (Op::If, None),
            (Op::Const, Some(3i32)),
            (Op::EndIf, None),
        ]);

        assert_eq!(0, sm.stack.len());
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

    #[test]
    fn test_not_succeeds() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![(Op::Const, Some(0i32)), (Op::Not, None)]);

        assert_eq!(Some(1i32), sm.pop());
    }

    #[test]
    fn test_not_fails() {
        let mut sm = StackMachine::new(2u32.pow(8));

        sm.execute(vec![(Op::Const, Some(1i32)), (Op::Not, None)]);

        assert_eq!(Some(0i32), sm.pop());
    }

    #[test]
    /**
     * Should perhaps write out to a file an example so not to rely
     * on the examples remaining exactly the same
     */
    fn test_read() {
        let _ = reader::read(&String::from("examples/adder.sm"));
    }

    #[test]
    fn test_integration_builder() {
        let mut builder = Builder::new(2u32.pow(16));
        builder.r#const(5).r#const(2).mul().execute();
        assert_eq!(Some(10i32), builder.sm.last());
    }

    #[test]
    fn test_pushstr() {

    }

    #[test]
    #[ignore]
    fn test_ext_function_() {
        // let mut sm = StackMachine::new(2u32.pow(8));
        // sm.execute(vec![(Op::Const, Some(1i32)), (Op::Not, None)]);
    }

    #[test]
    #[ignore]
    fn test_cli() {}

    #[test]
    #[ignore]
    fn test_binary_run() {}
}
