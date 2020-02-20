
use std::thread;
use std::iter::FromIterator;

mod function;

pub type Function = function::Function;
pub type Operation = function::op::Operation;

pub struct StackMachine {
    pub stack: Vec<u32>,
    pub memory: Vec<u8>,
    pub ext_functions: Vec<Box<dyn Fn(&StackMachine)>>,
    pub function_table: Vec<Function>,
}

impl StackMachine {
    pub fn pop(&mut self) -> Option<u32> {
        return self.stack.pop();
    }

    pub fn push(&mut self, item: u32) {
        return self.stack.push(item);
    }

    pub fn add(&mut self, a: u32, b: u32) {
        self.push(a + b);
    }

    pub fn sub(&mut self, a: u32, b: u32) {
        self.push(a - b);
    }

    pub fn mul(&mut self, a: u32, b: u32) {
        self.push(a * b);
    }

    pub fn div(&mut self, a: u32, b: u32) {
        self.push(a / b);
    }

    pub fn call_func(&mut self, fn_id: u8) {
        self.execute(self.function_table[fn_id as usize].code.clone());
    }

    pub fn call_func_ext(&mut self, fn_id: u8) {
        (self.ext_functions[fn_id as usize])(&self);
    }

    pub fn new(memsize: u32) -> StackMachine {
        return StackMachine {
            stack: Vec::new(),
            memory: Vec::with_capacity(memsize as usize),
            ext_functions: Vec::<Box<dyn Fn(&StackMachine)>>::new(),
            function_table: Vec::<Function>::new(),
        };
    }

    pub fn execute(&mut self, mut code: Vec<(Operation, Option<u32>)>) {

        let mut index = 0;
        let mut child_pid = 1;
        loop {
            if index == code.len() { break; }

            let (op, arg) = &code[index];
            match op {
                Operation::Add => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.add(a, b);
                },
                Operation::Sub => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.sub(a, b);
                },
                Operation::Mul => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.mul(a, b);
                },
                Operation::Div => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.div(a, b);
                },
                Operation::Const => {
                    self.push(arg.unwrap());
                },
                Operation::Call => {
                    self.call_func(arg.unwrap() as u8);
                },
                Operation::Fork => {
                    self.push(0);
                    let mut _code = Vec::from_iter(code[index..].iter().cloned());
                    thread::spawn(move || {
                        let mut sm = StackMachine::new(2u32.pow(16));
                        sm.push(child_pid);
                        sm.execute(_code);
                    });
                    child_pid += 1;
                },
                Operation::Pop => { self.pop().unwrap(); },
                Operation::Push => self.push(arg.unwrap()),
                Operation::CallExt => {
                    self.call_func_ext(arg.unwrap() as u8);
                },
                Operation::Print => {
                    println!("{}", self.pop().unwrap());
                },
                _ => panic!("Command not implemented.")
            };

            index += 1;
        }
    }
}
