
mod stackmachine;

use crate::stackmachine::Function;
use crate::stackmachine::Op;

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

    sm.execute(vec![
        (Op::CallExt,   Some(0)),
        (Op::Const,     Some(1)),
        (Op::CallExt,   Some(0)),
        (Op::If,        None),
        (Op::CallExt,   Some(0)),
        (Op::Const,     Some(7)),
        (Op::CallExt,   Some(0)),
        (Op::EndIf,     None),
        (Op::CallExt,   Some(0)),
    ]);
}
