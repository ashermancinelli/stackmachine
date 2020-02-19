mod ops;

pub type Operation = ops::Operation;

#[derive(Clone)]
pub struct Function {
    pub code: Vec<(Operation, Option<u32>)>,
}

impl Function {
    pub fn new(code: Vec<(Operation, Option<u32>)>) -> Function {
        return Function {
            code: code,
        };
    }
}
