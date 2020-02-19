
mod stackmachine;

use crate::stackmachine::Function;
use crate::stackmachine::Operation as Op;

fn main() {
    let mut sm = stackmachine::StackMachine::new(2u32.pow(16));

    sm.ext_functions = vec![
        Box::new(| s: &stackmachine::StackMachine | {
            println!("DEBUG sm.stack<{:?}>", s.stack);
        }),
    ];

    sm.function_table = vec![
        Function::new(vec![
            (Op::Add,   None),
        ]),
    ];

    println!("const'ing 6 and 3, then calling a function from the function table");
    println!("to add, then calling external function to print.");
    sm.execute(vec![
        (Op::CallExt,   Some(0)),
        (Op::Const,     Some(6u32)),
        (Op::Const,     Some(3u32)),
        (Op::CallExt,   Some(0)),
        (Op::Call,      Some(0)),       // External adding func
        (Op::CallExt,   Some(0)),
    ]);
}
