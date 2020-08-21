use std::env;
use std::io::{Error, ErrorKind};
use std::path::Path;
use stackmachine::stackmachine::{reader, StackMachine};

// Prefer panics at this level. If we encounter an error at this level, we
// want to self destruct. Lower than this, prefer to return results and
// options so problems can be handled.
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Please pass filename to stackmachine.")
    }

    for arg in args.iter().skip(1) {
        let path = Path::new(arg);
        if path.exists() {
            if let Some(p) = path.to_str() {
                let mut sm = StackMachine::new(2u32.pow(16));
                
                let code = reader::read(&String::from(p));
                if let Some(c) = code {
                    sm.execute(c);
                }
                else {
                    panic!("Could not parse code.");
                }
            }
        }
        else {
            panic!("Could not find file {}.", path.to_str().unwrap());
        }
    }
    Ok(())
}
