mod stackmachine;

use crate::stackmachine::Builder;
use crate::stackmachine::Function;
use crate::stackmachine::Op;
use crate::stackmachine::StackMachine;
use crate::stackmachine::reader::read;

fn main() {
    let mut sm = StackMachine::new(2u32.pow(16));

    sm.ext_functions = vec![Box::new(|s: &StackMachine| {
        println!("DEBUG sm.stack<{:?}>", s.stack);
    })];

    sm.function_table = vec![Function::new(vec![(Op::Add, None)])];

    sm.execute(vec![
        (Op::Const, Some(1)),
        (Op::If, None),
        (Op::Const, Some(7)),
        (Op::EndIf, None),
    ]);

    println!("Manual stackmachine with {:?}", sm.stack);

    let mut builder = Builder::new(2u32.pow(16));

    builder.r#const(5).r#const(2).mul().execute();

    println!("Builder stack with {:?}", builder.sm.stack);

    let filename = String::from("examples/adder.sm");
    let code = stackmachine::reader::read(&filename);
    sm = StackMachine::new(2u32.pow(16));
    if let Some(c) = code {
        sm.execute(c);
    }
    else {
        panic!("Could not read code from file {}", filename);
    }

    println!("Stack read from file with {:?}", sm.stack);
}
