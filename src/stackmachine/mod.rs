use std::fmt;
use std::thread;

pub mod function;
pub mod reader;
pub mod builder;

pub use crate::stackmachine::function::Function;
pub use crate::stackmachine::function::Op;
pub use crate::stackmachine::builder::Builder;

pub struct StackMachine {
    pub stack: Vec<i32>,
    pub memory: Vec<u8>,
    pub ext_functions: Vec<Function>,
    pub function_table: Vec<Vec<(Op, Option<i32>)>>,
    pub pid: u16,
    pub child: bool,
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
        self.execute(self.function_table[fn_id as usize].clone());
    }

    pub fn call_func_ext(&mut self, fn_id: u8) {
        (self.ext_functions[fn_id as usize])(&mut self.stack);
    }

    pub fn new(memsize: u32) -> StackMachine {
        return StackMachine {
            stack: Vec::<i32>::new(),
            memory: Vec::with_capacity(memsize as usize),
            ext_functions: Vec::<Function>::new(),
            function_table: Vec::<Vec<(Op, Option<i32>)>>::new(),
            pid: 0,
            child: false,
        };
    }

    fn r#if(&mut self, index: &mut usize, code: &mut Vec<(Op, Option<i32>)>) {
        // Value that represents the conditional
        // default to false if `if` statement appears with empty stack
        let a = self.pop().unwrap_or(0);

        if a > 0 { // The condition was met

            // nesting level for conditional statements
            let mut nest = 1;
            let start = *index + 1;
            while nest > 0 {
                *index += 1;
                nest += match code[*index] {
                    (Op::Else, _) => {
                        if nest == 1 {
                            self.execute(code
                                         .iter()
                                         .cloned()
                                         .skip(start)
                                         .take(*index - start)
                                         .collect());
                            *index += code
                                .iter()
                                .cloned()
                                .skip(*index)
                                .take_while(|(x, _)| *x != Op::EndIf)
                                .count();
                            return;
                        }
                        0
                    }
                    (Op::EndIf, _) => -1,
                    (Op::If, _) => 1,
                    _ => 0,
                };
            }
            // Execute through the end of the if block
            self.execute(code
                         .iter()
                         .cloned()
                         .skip(start)
                         .take(*index-start)
                         .collect());

        } else { // The condition was not met

            if *index == code.len() {
                return;
            }

            let mut nest = 1;
            let mut else_idx = None; // starting index of else block
            while nest > 0 {
                match code[*index] {
                    (Op::EndIf, _) => {
                        nest -= 1;
                        if nest == 1 {
                            break;
                        }
                    }
                    (Op::Else, _) => {
                        if nest == 1 {
                            else_idx = Some(*index);
                        }
                    }
                    (Op::If, _) => {
                        nest += 1;
                    }
                    _ => (),
                };
                *index += 1;
            }

            if let Some(idx) = else_idx {
                self.execute(code
                             .iter()
                             .cloned()
                             .skip(idx+1)
                             .take(*index - idx - 1)
                             .collect());
            }
        }
    }

    // No return type because the stack machine will self destruct when syntax
    // errors are encountered.
    pub fn syntax_check(&self, code: &Vec<(Op, Option<i32>)>) {
        let mut ifs = 0;
        let mut endifs = 0; 
        let mut elses = 0;
        for (op, _) in code {
            match op {
                Op::If => ifs += 1,
                Op::EndIf => endifs += 1,
                Op::Else => elses += 1,
                _ => (),
            }
        }

        if ifs != endifs {
            panic!("Each `if` must have a matching `endif`. Got {} if statements and {} endif statements.",
                   ifs, endifs);
        }
        if elses > ifs {
            panic!("`Else` may appear max of one time per if block.");
        }
    }

    pub fn execute(&mut self, mut code: Vec<(Op, Option<i32>)>) {

        // println!("Executing routine: {:?}", code);

        self.syntax_check(&code);
        let mut children = vec![];
        let mut index = 0;
        loop {
            if index >= code.len() {
                break;
            }

            let (op, arg) = &code[index];

            // Match the opcodes with corresponding handlers or actions
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
                Op::EndIf | Op::EndFunction | Op::Return | Op::Else => {
                    return;
                }
                // Simluates a fork system call using threads.
                // Creates a new stack machine on the new thread, pushes the
                // rest of the code to currently being executed on this stack
                // machine, and sets the child's PID and 'child' member.
                Op::Fork => {
                    let child_code = code
                        .iter()
                        .cloned()
                        .skip(index+1) // omitting the +1 leades to infinite threads
                        .collect();
                    let stack = self.stack.clone();
                    let child_pid = self.pid * 2 + 1;
                    self.child = false;
                    children.push(thread::Builder::new()
                        .name(format!("Thread<{}>", child_pid).to_string())
                        .spawn(move || {
                            let mut sm = StackMachine::new(2u32.pow(16));
                            sm.pid = child_pid;
                            sm.stack = stack;
                            sm.child = true;
                            sm.execute(child_code);
                        })
                        .expect("Could not spawn thread"));
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

        // wait for children to finish
        for handle in children {
            handle.join().unwrap();
        }
    }
}

impl fmt::Display for StackMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StackMachine<{}, {:?}>", self.pid, self.stack)
    }
}
