use std::fmt;
use std::io::{self, Write};
use std::char;
use std::thread;
use std::collections::HashMap;

pub mod builder;
pub mod function;
pub mod reader;

pub use crate::stackmachine::builder::Builder;
pub use crate::stackmachine::function::Function;
pub use crate::stackmachine::function::Op;

pub struct StackMachine<'a> {
    pub stack: Vec<i32>,
    pub memory: Vec<u8>,
    pub ext_functions: Vec<Function>,
    pub ext_functions_: HashMap<&'a str, Function>,
    pub function_table: Vec<Vec<(Op, Option<i32>)>>,
    pub pid: u16,
    pub child_pid: u16,
    pub child: bool,
}

impl<'a> StackMachine<'a> {
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

    pub fn new(memsize: u32) -> StackMachine<'a> {
        StackMachine {
            stack: Vec::<i32>::new(),
            memory: Vec::with_capacity(memsize as usize),
            ext_functions: Vec::<Function>::new(),
            ext_functions_: HashMap::<&'a str, Function>::new(),
            function_table: Vec::<Vec<(Op, Option<i32>)>>::new(),
            pid: 0,
            child_pid: 0,
            child: false,
        }
    }

    fn r#if(&mut self, index: &mut usize, code: &mut Vec<(Op, Option<i32>)>) {
        // _a_ is the value that represents the conditional
        if let Some(a) = self.pop() {
            if a > 0 { // The condition was met

                /*
                 * Search for either an EndIf statement that matches the same
                 * nesting level or an else block that matches the same nesting
                 * level. 
                 *
                 * If an Else is found, the index is incremented to the end of the
                 * `EndIf` statement, not the `Else` statement.
                 */
                let mut nest = 1;
                let start = *index + 1;
                while nest > 0 {
                    *index += 1;
                    nest += match code[*index] {
                        (Op::Else, _) => {
                            if nest == 1 {
                                self.execute(
                                    code.iter()
                                        .cloned()
                                        .skip(start)
                                        .take(*index - start)
                                        .collect(),
                                );
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
                self.execute(
                    code.iter()
                        .cloned()
                        .skip(start)
                        .take(*index - start)
                        .collect(),
                );
            } else { // The condition was not met

                if *index == code.len() {
                    return;
                }

                /*
                 * Find a matching else block that corresponds with the current
                 * level of `if` block. If not found, the condition after this for
                 * loop will fall through and execution will pick up at the end of
                 * the `if` block.
                 */
                let mut nest = 1;
                let mut else_idx = None; // starting index of else block
                while nest > 0 {
                    *index += 1;
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
                }

                if let Some(idx) = else_idx {
                    self.execute(
                        code.iter()
                            .cloned()
                            .skip(idx + 1)
                            .take(*index - idx - 1)
                            .collect(),
                    );
                }
            }
        }
        else {
            panic!("`if` statement called without anything on the stack!");
        }
    }

    // No return type because the stack machine will self destruct when syntax
    // errors are encountered.
    pub fn syntax_check(&self, code: &Vec<(Op, Option<i32>)>) {
        let mut ifs = 0;
        let mut endifs = 0;
        let mut elses = 0;
        let mut fns = 0;
        let mut endfns = 0;
        for (op, _) in code {
            match op {
                Op::If => ifs += 1,
                Op::EndIf => endifs += 1,
                Op::Else => elses += 1,
                Op::Function => fns += 1,
                Op::EndFunction => endfns += 1,
                _ => (),
            }
        }

        if ifs != endifs {
            panic!("Each `if` must have a matching `endif`. Got {} if statements and {} endif statements.",
                   ifs, endifs);
        }
        if fns != endfns {
            panic!("Each `function` must have a matching `endfunction`. Got {} function statements and {} endfunction statements.",
                   fns, endfns);
        }
        if elses > ifs {
            panic!("`Else` may appear max of one time per if block.");
        }
    }

    pub fn execute(&mut self, mut code: Vec<(Op, Option<i32>)>) {
        #[cfg(debug_assertions)]
        println!("Executing routine: {:?}", code);

        self.syntax_check(&code);
        let mut children = Vec::<thread::JoinHandle<_>>::new();
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
                    let b = self.pop().unwrap();
                    if a == b {
                        self.push(1);
                    } else {
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
                Op::EndFunction | Op::Return => {
                    return;
                }
                Op::Else | Op::EndIf => panic!("Each `else` or `endif` must have a matching `if` statement!"),
                // Simluates a fork system call using threads.
                // Creates a new stack machine on the new thread, pushes the
                // rest of the code to currently being executed on this stack
                // machine, and sets the child's PID and 'child' member.
                //
                // TODO: favor a rudimentary scheduler instead of using threads
                Op::Fork => {
                    let child_code = code
                        .iter()
                        .cloned()
                        .skip(index + 1) // omitting the +1 leades to infinite threads
                        .collect();
                    let stack = self.stack.clone();
                    self.child_pid *= 2;
                    let child_pid = self.child_pid + 1;
                    self.child = false;
                    children.push(
                        thread::Builder::new()
                            .name(format!("Thread<{}>", child_pid).to_string())
                            .spawn(move || {
                                let mut sm = StackMachine::new(2u32.pow(16));
                                sm.pid = child_pid;
                                sm.stack = stack;
                                sm.child = true;
                                sm.execute(child_code);
                            })
                            .expect("Could not spawn thread"),
                    );
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
                Op::PrintStr => {
                    while let Some(v) = self.pop() {
                        if v == 0 {
                            break;
                        }
                        else {
                            print!("{}", (v as u8) as char);
                        }
                    }
                    println!("");
                    io::stdout().flush().unwrap();
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

impl<'a> fmt::Display for StackMachine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StackMachine<{}, {:?}>", self.pid, self.stack)
    }
}
