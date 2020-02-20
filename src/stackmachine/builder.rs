use crate::stackmachine::Function;
use crate::stackmachine::Op;
use crate::stackmachine::StackMachine;

pub struct Builder {
    pub sm: StackMachine,
    pub code: Vec<(Op, Option<i32>)>,
}

impl Builder {
    pub fn new(memsize: u32) -> Builder {
        return Builder {
            sm: StackMachine {
                stack: Vec::new(),
                memory: Vec::with_capacity(memsize as usize),
                ext_functions: Vec::<Box<dyn Fn(&StackMachine)>>::new(),
                function_table: Vec::<Function>::new(),
                pid: 0,
            },
            code: Vec::new(),
        };
    }

    fn push(&mut self, line: (Op, Option<i32>)) {
        self.code.push(line);
    }

    pub fn print(&mut self) -> &mut Builder {
        self.push((Op::Print, None));
        return self;
    }

    pub fn r#const(&mut self, arg: i32) -> &mut Builder {
        self.push((Op::Const, Some(arg)));
        return self;
    }

    pub fn add(&mut self) -> &mut Builder {
        self.push((Op::Add, None));
        return self;
    }

    pub fn sub(&mut self) -> &mut Builder {
        self.push((Op::Sub, None));
        return self;
    }

    pub fn mul(&mut self) -> &mut Builder {
        self.push((Op::Mul, None));
        return self;
    }

    pub fn div(&mut self) -> &mut Builder {
        self.push((Op::Div, None));
        return self;
    }

    pub fn call(&mut self, arg: i32) -> &mut Builder {
        self.push((Op::Call, Some(arg)));
        return self;
    }

    pub fn call_ext(&mut self, arg: i32) -> &mut Builder {
        self.push((Op::CallExt, Some(arg)));
        return self;
    }

    pub fn fork(&mut self) -> &mut Builder {
        self.push((Op::Fork, None));
        return self;
    }

    pub fn r#if(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn if_eq(&mut self) -> &mut Builder {
        self.push((Op::IfEq, None));
        return self;
    }

    pub fn if_not(&mut self) -> &mut Builder {
        self.push((Op::IfNot, None));
        return self;
    }

    pub fn if_gt(&mut self) -> &mut Builder {
        self.push((Op::IfGT, None));
        return self;
    }

    pub fn if_lt(&mut self) -> &mut Builder {
        self.push((Op::IfLT, None));
        return self;
    }

    pub fn if_gte(&mut self) -> &mut Builder {
        self.push((Op::IfGTE, None));
        return self;
    }

    pub fn if_lte(&mut self) -> &mut Builder {
        self.push((Op::IfLTE, None));
        return self;
    }

    pub fn end_if(&mut self) -> &mut Builder {
        self.push((Op::EndIf, None));
        return self;
    }

    pub fn get_pid(&mut self) -> &mut Builder {
        self.push((Op::GetPid, None));
        return self;
    }

    pub fn execute(&mut self) -> &StackMachine {
        self.sm.execute(self.code.clone());
        return &mut self.sm;
    }
}

#[cfg(test)]
mod builder_test {

    use super::Builder;
    use super::Function;
    use super::Op;

    #[test]
    fn test_builder_new() {
        let builder = Builder::new(2u32.pow(16));

        assert_eq!(Vec::<i32>::new(), builder.sm.stack);
    }

    #[test]
    fn test_builder_const() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).execute();

        assert_eq!(Some(5), builder.sm.last());
    }

    #[test]
    fn test_builder_add() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(3).add().execute();

        assert_eq!(Some(8), builder.sm.last());
    }

    #[test]
    fn test_builder_sub() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(3).sub().execute();

        assert_eq!(Some(-2), builder.sm.last());
    }

    #[test]
    fn test_builder_mul() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(3).mul().execute();

        assert_eq!(Some(15), builder.sm.last());
    }

    #[test]
    fn test_builder_div() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(3).div().execute();

        assert_eq!(Some(0), builder.sm.last());
    }

    #[test]
    fn test_builder_fork() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.fork().execute();

        assert_eq!(Some(0), builder.sm.last());
    }

    #[test]
    fn test_builder_get_pid() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.fork().get_pid().execute();

        assert_eq!(Some(0), builder.sm.last());
    }

    #[test]
    fn test_builder_if_succeeds() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(1).r#if().r#const(5).end_if().execute();

        assert_eq!(Some(5), builder.sm.last());
    }

    #[test]
    fn test_builder_if_fails() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(0).r#if().r#const(3).end_if().execute();

        assert_eq!(Some(5), builder.sm.last());
    }

    #[test]
    fn test_builder_if_not_succeeds() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(0).r#if_not().r#const(3).end_if().execute();

        assert_eq!(Some(3), builder.sm.last());
    }

    #[test]
    fn test_builder_if_not_fails() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(3).r#const(1).r#if_not().r#const(5).end_if().execute();

        assert_eq!(Some(3), builder.sm.last());
    }

    #[test]
    fn test_builder_print() {
        let mut builder = Builder::new(2u32.pow(16));

        builder.r#const(5).r#const(0).print().execute();

        assert_eq!(Some(5), builder.sm.last());
    }

    #[test]
    fn test_builder_call() {
        let mut builder = Builder::new(2u32.pow(16));
        builder.sm.function_table = vec![Function::new(vec![(Op::Add, None)])];

        builder.r#const(5).r#const(3).call(0).execute();

        assert_eq!(Some(8), builder.sm.last());
    }
}
