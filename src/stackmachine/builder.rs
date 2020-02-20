
use crate::stackmachine::StackMachine;
use crate::stackmachine::Op;
use crate::stackmachine::Function;

pub struct Builder {
    pub sm: StackMachine,
    pub code: Vec<(Op, Option<u32>)>,
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

    fn push(&mut self, line: (Op, Option<u32>)) {
        self.code.push(line);
    }

    pub fn Print(&mut self) -> &mut Builder {
        self.push((Op::Print, None));
        return self;
    }

    pub fn Const(&mut self, arg: u32) -> &mut Builder {
        self.push((Op::Const, Some(arg)));
        return self;
    }

    pub fn Add(&mut self) -> &mut Builder {
        self.push((Op::Add, None));
        return self;
    }

    pub fn Sub(&mut self) -> &mut Builder {
        self.push((Op::Sub, None));
        return self;
    }

    pub fn Mul(&mut self) -> &mut Builder {
        self.push((Op::Mul, None));
        return self;
    }

    pub fn Div(&mut self) -> &mut Builder {
        self.push((Op::Div, None));
        return self;
    }

    pub fn Call(&mut self, arg: u32) -> &mut Builder {
        self.push((Op::Call, Some(arg)));
        return self;
    }

    pub fn CallExt(&mut self, arg: u32) -> &mut Builder {
        self.push((Op::CallExt, Some(arg)));
        return self;
    }

    pub fn Fork(&mut self) -> &mut Builder {
        self.push((Op::Fork, None));
        return self;
    }

    pub fn If(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfEq(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfNot(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfGT(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfLT(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfGTE(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn IfLTE(&mut self) -> &mut Builder {
        self.push((Op::If, None));
        return self;
    }

    pub fn EndIf(&mut self) -> &mut Builder {
        self.push((Op::EndIf, None));
        return self;
    }

    pub fn GetPid(&mut self) -> &mut Builder {
        self.push((Op::GetPid, None));
        return self;
    }

    pub fn Execute(&mut self) -> &StackMachine {
        self.sm.execute(self.code.clone());
        return &mut self.sm;
    }
}

#[cfg(test)]
mod builder_test {

    use super::Builder;
    use super::StackMachine;

    #[test]
    fn test_builder_new() {
        let mut builder = Builder::new(2u32.pow(16));

        builder
            .Const(5)
            .Const(3)
            .Add()
            .Execute();

        assert_eq!(Some(8), builder.sm.last());
    }
}
