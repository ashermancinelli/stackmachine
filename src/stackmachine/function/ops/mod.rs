
#[derive(Clone)]
pub enum Operation {
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Noop,
    Block,
    Loop,
    Return,
    Break,
    CallExt,
    Call,
}

