
mod stackmachine;

#[cfg(test)]
pub mod tests {

    use super::stackmachine::StackMachine;
    type Op = super::stackmachine::Operations;

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
}
