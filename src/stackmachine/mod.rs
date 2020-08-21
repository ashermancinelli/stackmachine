use std::fmt;
use std::ptr;
use std::iter::FromIterator;
use std::thread;

pub mod builder;
pub mod function;
pub mod reader;

pub type Builder = builder::Builder;
pub type Function = function::Function;
pub type Op = function::op::Operation;

pub struct StackMachine {
    pub stack: Vec<i32>,
    pub memory: Vec<u8>,
    pub ext_functions: Vec<Box<dyn Fn(&StackMachine)>>,
    pub function_table: Vec<Function>,
    pub pid: u16,
    pub child: bool,
    pub child_pid: u16,
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
            child: false,
            child_pid: 1,
        };
    }

    fn r#if(&mut self, index: &mut usize, code: &mut Vec<(Op, Option<i32>)>) {
        // Value conditional is checked on
        let a = self.pop().unwrap_or(0);

        // The condition was met
        if a > 0 {
            // Execute through the end of the if block
            self.execute(code
                         .iter()
                         .cloned()
                         .skip(*index + 1)
                         .take_while(|(x, _)| *x != Op::EndIf && *x != Op::Else)
                         .collect());
            *index += code 
                .iter()
                .cloned()
                .skip(*index)
                .take_while(|(x, _)| *x != Op::EndIf && *x != Op::Else)
                .count();
        } else {
            // An else block exists
            // Execute through the end of the else block
            if let Some(_) = code.iter().cloned().find(|(x, _)| *x == Op::Else) {
                self.execute(code
                             .iter()
                             .cloned()
                             .skip(*index)
                             .skip_while(|(x, _)| *x != Op::Else)
                             .skip(1)
                             .take_while(|(x, _)| *x != Op::EndIf)
                             .collect());
                *index += code
                    .iter()
                    .cloned()
                    .skip(*index)
                    .take_while(|(x, _)| *x != Op::EndIf)
                    .count();
            }

            // No else block exists; just return
            else {
                return;
            }
        }
    }

    pub fn execute(&mut self, mut code: Vec<(Op, Option<i32>)>) {
        println!("Executing code {:?}", code);
        let mut index = 0;
        loop {
            if index == code.len() {
                break;
            }

            let (op, arg) = &code[index];
            // println!("DEBUG::op({:?})", op);

            // Most of these operations simply call corresponding handlers,
            // though some of the opcodes are noops (like endif for example).
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
                Op::r#Eq => {
                    let a = self.pop().unwrap();
                    if a == arg.unwrap() {
                        self.push(1);
                    }
                    else {
                        self.push(0);
                    }
                }
                Op::Call => {
                    self.call_func(arg.unwrap() as u8);
                }
                Op::If => {
                    self.r#if(&mut index, &mut code);
                }
                Op::Not => {
                    let a = self.pop().unwrap();
                    if a <= 0 {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Op::EndIf | Op::EndFunction | Op::Return => {
                    return;
                }
                // Simluates a fork system call using threads.
                // Creates a new stack machine on the new thread, pushes the
                // rest of the code to currently being executed on this stack
                // machine, and sets the child's PID and 'child' member.
                Op::Fork => {
                    let child_code = Vec::from_iter(code[index..].iter().cloned());
                    let stack = self.stack.clone();
                    let child_pid = self.child_pid.clone();
                    self.child = false;
                    thread::Builder::new()
                        .name(format!("Thread<{}>", self.child_pid).to_string())
                        .spawn(move || {
                            let mut sm = StackMachine::new(2u32.pow(16));
                            sm.pid = child_pid;
                            sm.stack = stack;
                            sm.child = true;
                            sm.execute(child_code);
                        })
                        .expect("Could not spawn thread");
                    self.child_pid += 1;
                }
                Op::GetPid => {
                    self.push(self.pid.into());
                }
                Op::Child => {
                    if self.child {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Op::Pop => {
                    self.pop().unwrap();
                }
                Op::Push => self.push(arg.unwrap()),

                // Call external function described in-code
                Op::CallExt => {
                    self.call_func_ext(arg.unwrap() as u8);
                }
                Op::Print => {
                    println!("{}", self.last().unwrap());
                }
                Op::Debug => {
                    println!("DEBUG::{}", self);
                }
                _ => panic!("Command {:?} not implemented.", op),
            };
            index += 1;
        }
    }
}

impl fmt::Display for StackMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StackMachine<{}, {:?}>", self.pid, self.stack)
    }
}
