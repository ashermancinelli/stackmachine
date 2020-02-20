use std::iter::FromIterator;
use std::thread;

pub mod builder;
pub mod function;

pub type Builder = builder::Builder;
pub type Function = function::Function;
pub type Op = function::op::Operation;

pub struct StackMachine {
    pub stack: Vec<i32>,
    pub memory: Vec<u8>,
    pub ext_functions: Vec<Box<dyn Fn(&StackMachine)>>,
    pub function_table: Vec<Function>,
    pub pid: u8,
}

impl StackMachine {
    pub fn last(&self) -> Option<i32> {
        if self.stack.len() == 0 {
            return None;
        } else {
            return Some(self.stack[self.stack.len() - 1]);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        return self.stack.pop();
    }

    pub fn push(&mut self, item: i32) {
        return self.stack.push(item);
    }

    pub fn add(&mut self, a: i32, b: i32) {
        self.push(a + b);
    }

    pub fn sub(&mut self, a: i32, b: i32) {
        self.push(a - b);
    }

    pub fn mul(&mut self, a: i32, b: i32) {
        self.push(a * b);
    }

    pub fn div(&mut self, a: i32, b: i32) {
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
            pid: 0,
        };
    }

    fn r#if(&mut self, index: &mut usize, code: &mut Vec<(Op, Option<i32>)>) {
        let a = self.pop().unwrap();
        if a > 0 {
            *index += 1;
            let inner = Vec::from_iter(code[*index..].iter().cloned());
            self.execute(inner);
        } else {
            let start_if_level = 0;
            let mut if_level = 0;
            loop {
                *index += 1;
                let (op, _) = &code[*index];

                match op {
                    Op::If => {
                        if_level += 1;
                    }
                    Op::EndIf => {
                        if_level -= 1;
                    }
                    Op::Else => {
                        *index += 1;
                        let inner = Vec::from_iter(code[*index..].iter().cloned());
                        self.execute(inner);
                    }
                    _ => {
                        if *index == code.len() {
                            panic!("Mismatch in number of if's and endif's!");
                        }
                    }
                };
                if start_if_level == if_level {
                    return;
                }
            }
        }
    }

    pub fn execute(&mut self, mut code: Vec<(Op, Option<i32>)>) {
        let mut index = 0;
        let mut child_pid = 1;
        loop {
            if index == code.len() {
                break;
            }

            let (op, arg) = &code[index];
            // println!("DEBUG::op({:?})", op);
            match op {
                Op::Const => {
                    self.push(arg.unwrap());
                }
                Op::Add => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.add(a, b);
                }
                Op::Sub => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.sub(a, b);
                }
                Op::Mul => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.mul(a, b);
                }
                Op::Div => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.div(a, b);
                }
                Op::Call => {
                    self.call_func(arg.unwrap() as u8);
                }
                Op::If => {
                    self.r#if(&mut index, &mut code);
                }
                Op::IfNot => {
                    let a = self.pop().unwrap();
                    if a <= 0 {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                    self.r#if(&mut index, &mut code);
                }
                Op::EndIf | Op::EndFunction => {
                    return;
                }
                Op::Fork => {
                    let mut _code = Vec::from_iter(code[index..].iter().cloned());
                    thread::Builder::new()
                        .name(format!("Thread<{}>", child_pid).to_string())
                        .spawn(move || {
                            let mut sm = StackMachine::new(2u32.pow(16));
                            sm.pid = child_pid;
                            sm.push(child_pid.into());
                            sm.execute(_code);
                        })
                        .expect("Could not spawn thread");
                    self.push(self.pid.into());
                    child_pid += 1;
                }
                Op::GetPid => {
                    self.push(self.pid.into());
                }
                Op::Pop => {
                    self.pop().unwrap();
                }
                Op::Push => self.push(arg.unwrap()),
                Op::CallExt => {
                    self.call_func_ext(arg.unwrap() as u8);
                }
                Op::Print => {
                    println!("{}", self.pop().unwrap());
                }
                _ => panic!("Command {:?} not implemented.", op),
            };
            index += 1;
        }
    }
}
