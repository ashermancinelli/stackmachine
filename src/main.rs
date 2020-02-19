
mod stackmachine;

use crate::stackmachine::Function;
use crate::stackmachine::Operation as Op;

fn main() {
    let mut sm = stackmachine::StackMachine::new(2u32.pow(16));

    sm.ext_functions = vec![
        Box::new(| s: &stackmachine::StackMachine | {
            println!("Stack is {:?}", s.stack);
        }),
    ];

    sm.function_table = vec![
        Function::new(vec![
            (Op::Const,     Some(6u32)),
        ]),
    ];

    sm.execute(vec![
        (Op::Const,     Some(6u32)),
        (Op::Const,     Some(3u32)),
        (Op::CallExt,   Some(0)),
        (Op::Const,     Some(1u32)),
        (Op::CallExt,   Some(0)),
        (Op::Const,     Some(1u32)),
        (Op::CallExt,   Some(0)),
        (Op::Add,       None),
        (Op::CallExt,   Some(0)),
        (Op::Call,      Some(0)),
    ]);
}
