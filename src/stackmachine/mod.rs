
use std::thread;

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

        let index = 0;

        loop {
            let (op, arg) = code[index];
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
                Operation::Equal => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.push(a == b ? 1 : 0);
                },
                Operation::NotEqual => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.push(a != b ? 1 : 0);
                },
                Operation::Function => {
                },
                Operation::EndFunction => {
                },
                Operation::If => {
                },
                Operation::EndIf => {
                },
                Operation::Const => {
                    self.push(arg.unwrap());
                },
                Operation::Call => {
                    self.call_func(arg.unwrap() as u8);
                },
                Operation::CallExt => {
                    self.call_func_ext(arg.unwrap() as u8);
                },
                Operation::Fork => {
                    let mut child = self.clone();
                    child.push(1);
                    self.push(0);
                    let mut _code = code.clone()[index..];
                    
                    thread::spawn(move || {
                        child.execute(_code);
                    });
                },
                Operation::Print => {
                    println!("{}", self.pop().unwrap());
                },
                _ => panic!("Command not implemented.")
            };
        }
    }
}
