
mod stackmachine;

type Op = stackmachine::Operations;

fn main() {
    let mut sm = stackmachine::StackMachine::new(2u32.pow(16));

    sm.execute(vec![
        (Op::Const, Some(1u32)),
        (Op::Const, Some(1u32)),
        (Op::Add,   None),
        (Op::Print, None),
    ]);
}
