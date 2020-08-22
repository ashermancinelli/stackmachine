use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::stackmachine::Op;

pub struct Reader {
    pub filename: String,
    pub lines: u8,
    pub code: Vec<(Op, Option<i32>)>,
}

// Taken from rust documentation
// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_opcode(line: &std::string::String) -> Option<(Op, Option<i32>)> {
    if line.len() == 0 {
        return None;
    }
    let strargs = String::from(line.to_ascii_lowercase().as_str().trim());

    let args = strargs
        .split(" ")
        .filter(|v| v.len() != 0)
        .collect::<Vec<&str>>();

    if args.len() == 0 || args[0] == "#" || args[0].chars().nth(0) == Some('#') {
        return None;
    }

    if args.len() > 2 {
        panic!("Got more than two arguments.");
    }

    let res = match args[0] {
        "const" => Some(Op::Const),
        "add" => Some(Op::Add),
        "sub" => Some(Op::Sub),
        "mul" => Some(Op::Mul),
        "div" => Some(Op::Div),
        "pop" => Some(Op::Pop),
        "push" => Some(Op::Push),
        "if" => Some(Op::If),
        "else" => Some(Op::Else),
        "endif" => Some(Op::EndIf),
        "gt" => Some(Op::GT),
        "lt" => Some(Op::LT),
        "gte" => Some(Op::GTE),
        "lte" => Some(Op::LTE),
        "eq" => Some(Op::r#Eq),
        "function" => Some(Op::Function),
        "endfunction" => Some(Op::EndFunction),
        "return" => Some(Op::Return),
        "fork" => Some(Op::Fork),
        "child" => Some(Op::Child),
        "getpid" => Some(Op::GetPid),
        "dbg" => Some(Op::Debug),
        "print" => Some(Op::Print),
        _ => None,
    };

    if let Some(op) = res {
        if args.len() > 1 {
            let val = args[1].parse::<i32>().unwrap();
            return Some((op, Some(val)));
        }
        return Some((op, None));
    } else {
        panic!("Could not parse opcode.");
    }
}

// Reads opcodes from a file
pub fn read(filename: &String) -> Option<Vec<(Op, Option<i32>)>> {
    let mut lineno = 0;
    let mut code: Vec<(Op, Option<i32>)> = Vec::new();

    println!("-- {}", filename);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if let Some(tup) = parse_opcode(&l) {
                    println!("{} {:?}", lineno, tup);
                    code.push(tup);
                }
                lineno += 1;
            }
        }
        return Some(code);
    }
    None
}
