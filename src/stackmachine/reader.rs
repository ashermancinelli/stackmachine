use std::borrow::Cow;
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

fn resolve_path(short_path: &String) -> Result<String, ()> {
    let pkgs = short_path.split(".").collect::<Vec<&str>>();
    if pkgs.len() == 1 {
        let p = Path::new(pkgs[0]).join(".sm");
        if p.exists() {
            return match p.to_string_lossy() {
                Cow::Borrowed(resolved) => Ok(resolved.to_string()),
                Cow::Owned(resolved) => {
                    println!("Filepath {} was not valid UTF-8.", resolved);
                    Err(())
                }
            };
        } else {
            println!("No local file with name {}.sm was found.", pkgs[0]);
            Err(())
        }
    } else {
        println!("No standard library yet. Please only include local files.");
        Err(())
    }
}

fn parse_opcode(line: &std::string::String, code: &mut Vec<(Op, Option<i32>)>) {
    if line.len() == 0 {
        return;
    }
    let strargs = String::from(line.as_str().trim());

    let args = strargs
        .split(" ")
        .filter(|v| v.len() != 0)
        .collect::<Vec<&str>>();

    if args.len() == 0 || args[0] == "#" || args[0].chars().nth(0) == Some('#') {
        return;
    }

    let res = match args[0].to_ascii_lowercase().as_str() {
        "const" => Some(Op::Const),
        "add" => Some(Op::Add),
        "sub" => Some(Op::Sub),
        "mul" => Some(Op::Mul),
        "div" => Some(Op::Div),
        "pop" => Some(Op::Pop),
        "push" => Some(Op::Push),
        "pushstr" => Some(Op::PushStr),
        "if" => Some(Op::If),
        "else" => Some(Op::Else),
        "endif" => Some(Op::EndIf),
        "gt" => Some(Op::GT),
        "lt" => Some(Op::LT),
        "gte" => Some(Op::GTE),
        "lte" => Some(Op::LTE),
        "eq" => Some(Op::r#Eq),
        "call" => Some(Op::Call),
        "function" => Some(Op::Function),
        "endfunction" => Some(Op::EndFunction),
        "return" => Some(Op::Return),
        "fork" => Some(Op::Fork),
        "child" => Some(Op::Child),
        "getpid" => Some(Op::GetPid),
        "dbg" => Some(Op::Debug),
        "print" => Some(Op::Print),
        "printstr" => Some(Op::PrintStr),
        "include" => Some(Op::Include),
        "true" => {
            code.push((Op::Const, Some(1)));
            return;
        }
        "false" => {
            code.push((Op::Const, Some(0)));
            return;
        }
        _ => None,
    };

    if let Some(op) = res {
        if args.len() > 1 {
            // handle i32 arg
            if let Ok(val) = args[1].parse::<i32>() {
                code.push((op, Some(val)));
                return;
            }

            // Handle special non-i32 args
            match op {
                Op::Include => {
                    if let Ok(filename) = resolve_path(&args[1].to_string()) {
                        let input = read(&filename);
                        if let Some(c) = input {
                            code.extend(c)
                        } else {
                            panic!("Could not parse included file {}.", args[1]);
                        }
                    }
                }
                /*
                 * Multiple strings passed to pushstr are joined by a single
                 * space, no matter what they were originally seperated by
                 * when pushed.
                 */
                Op::PushStr => {
                    for (i, string) in args[1..].to_vec().iter().rev().enumerate() {
                        if i > 0 {
                            code.push((Op::Const, Some(' ' as i32)));
                        }
                        for c in string.chars().rev() {
                            code.push((Op::Const, Some(c as i32)));
                        }
                    }
                }
                _ => panic!("Could not parse argument to i32 and op was not a string opcode."),
            }
            return;
        }
        code.push((op, None));
    } else {
        panic!("Could not parse opcode from {:?}.", args);
    }
}

// Reads opcodes from a file
pub fn read(filename: &String) -> Option<Vec<(Op, Option<i32>)>> {
    let mut code: Vec<(Op, Option<i32>)> = Vec::new();

    #[cfg(debug_assertions)]
    println!("-- {}", filename);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                parse_opcode(&l, &mut code);
            }
        }
        #[cfg(debug_assertions)]
        for (op, v) in code.iter().cloned() {
            match v {
                Some(v) => println!("{:?} {}", op, v),
                None => println!("{:?}", op),
            }
        }
        return Some(code);
    }
    None
}
