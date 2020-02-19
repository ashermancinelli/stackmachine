
pub enum Operations {
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Noop,
}

#[allow(dead_code)]
pub struct StackMachine {
    pub stack: Vec<u32>,
    pub memory: Vec<u8>
}

impl StackMachine {
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        return self.stack.pop();
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn new(memsize: u32) -> StackMachine {
        return StackMachine {
            stack: Vec::new(),
            memory: Vec::with_capacity(memsize as usize),
        };
    }
    
    pub fn execute(&mut self, code: Vec<(Operations, Option<u32>)>) {
        for line in code {
            println!("Stack: {:?}", self.stack);
            match line.0 {
                Operations::Add => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.add(a, b);
                },
                Operations::Sub => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.sub(a, b);
                },
                Operations::Mul => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.mul(a, b);
                },
                Operations::Div => {
                    let a = self.pop().unwrap();
                    let b = self.pop().unwrap();
                    self.div(a, b);
                },
                Operations::Const => {
                    self.push(line.1.unwrap());
                },
                Operations::Print => {
                    println!("{}", self.pop().unwrap());
                },
                _ => panic!("Command not implemented.")
            };
        }
    }
}
