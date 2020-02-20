
use std::thread;
use std::iter::FromIterator;

mod function;

pub type Function = function::Function;
pub type Op = function::op::Operation;

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

    pub fn execute(&mut self, mut code: Vec<(Op, Option<u32>)>) {

        let mut index = 0;
        let mut child_pid = 1;
        loop {
            if index == code.len() { break; }

            let (op, arg) = &code[index];
            println!("DEBUG::op({:?})", op);
            match op {
                Op::Add => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.add(a, b);
                },
                Op::Sub => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.sub(a, b);
                },
                Op::Mul => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.mul(a, b);
                },
                Op::Div => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.div(a, b);
                },
                Op::Const => {
                    self.push(arg.unwrap());
                },
                Op::Call => {
                    self.call_func(arg.unwrap() as u8);
                },
                Op::If => {
                    let a = self.pop().unwrap();
                    if a > 0 {
                        index += 1;
                        let inner = Vec::from_iter(code[index..].iter().cloned());
                        self.execute(inner);
                    } else {
                        let mut start_if_level = 0;
                        let mut if_level = 0;
                        loop {
                            index += 1;
                            let (op, _) = &code[index];

                            match op {
                                Op::If => { if_level += 1; },
                                Op::EndIf => { if_level -= 1; },
                                _ => {
                                    if index == code.len() {
                                        panic!("Mismatch in number of if's and endif's!");
                                    }
                                },
                            }
                            if start_if_level == if_level {
                                return;
                            }
                        }
                    }
                },
                Op::EndIf | Op::EndFunction => {
                    return;
                },
                Op::Fork => {
                    self.push(0);
                    let mut _code = Vec::from_iter(code[index..].iter().cloned());
                    thread::spawn(move || {
                        let mut sm = StackMachine::new(2u32.pow(16));
                        sm.push(child_pid);
                        sm.execute(_code);
                    });
                    child_pid += 1;
                },
                Op::Pop => { self.pop().unwrap(); },
                Op::Push => self.push(arg.unwrap()),
                Op::CallExt => {
                    self.call_func_ext(arg.unwrap() as u8);
                },
                Op::Print => {
                    println!("{}", self.pop().unwrap());
                },
                _ => panic!("Command not implemented.")
            };

            index += 1;
        }
    }
}
