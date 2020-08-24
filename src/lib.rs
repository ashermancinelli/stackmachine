pub mod stackmachine;

#[cfg(test)]
pub mod tests {

    use super::stackmachine::builder::Builder;
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

        sm.function_table
            .insert("fn".to_string(), vec![(Op::Add, None)]);

        sm.execute(vec![
            (Op::Const, Some(3i32)),
            (Op::Const, Some(2i32)),
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Call, None), // External adding func
        ]);

        assert_eq!(Some(3 + 2), sm.pop());
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
    /*
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
    fn test_function_table() {
        let mut sm = StackMachine::new(2u32);

        sm.function_table
            .insert("fn".to_string(), vec![(Op::Add, None)]);

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
    fn test_fn_call() {
        let mut sm = StackMachine::new(2u32);

        sm.execute(vec![
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Function, None), // start function definition
            (Op::Add, None),
            (Op::EndFunction, None),
            (Op::Const, Some(1i32)), // arguments to function `fn`
            (Op::Const, Some(2i32)),
            (Op::Const, Some(0i32)), // Char codes for 'fn'
            (Op::Const, Some(110i32)),
            (Op::Const, Some(102i32)),
            (Op::Call, None), // Call function
        ]);

        assert!(sm.function_table.contains_key("fn"));
        assert_eq!(sm.last(), Some(1 + 2));
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
    fn test_ext_fn_call() {
        let mut sm = StackMachine::new(2u32);

        sm.ext_functions.insert("sum_n".to_string(), sum_n);

        sm.execute(vec![
            (Op::Const, Some(2)),
            (Op::Const, Some(2)),
            (Op::Const, Some(3)), // previous 3 values are to be summed
            (Op::Const, Some(3)), // number of values to sum
            (Op::Const, Some(0)), // Char codes for `sum_n`
            (Op::Const, Some(110)),
            (Op::Const, Some(95)),
            (Op::Const, Some(109)),
            (Op::Const, Some(117)),
            (Op::Const, Some(115)),
            (Op::CallExt, None), // External summing func
        ]);

        assert_eq!(sm.last(), Some(3 + 2 + 2));
    }

    #[test]
    #[ignore]
    /*
     * Will have to deal with strings to test how this is parsed into each
     * individual `const` call
     */
    fn test_pushstr() {
        let _code = r#"
        # Push the string onto the stack
        pushstr abc
        "#;
    }

    #[test]
    #[ignore]
    /*
     * Test argument handling of final binary
     */
    fn test_cli() {}

    #[test]
    #[ignore]
    /*
     * Call subprocs to run the final binary on test `.sm` files
     */
    fn test_binary_run() {}
}
