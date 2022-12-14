use std::process::exit; 
use std::fs; 
use std::io; 
use std::io::Read;
use std::str::Chars; 
use std::convert::TryInto;

fn usage() {
    println!("usage: subcommand option... source...");
    println!("subcommand:");
    println!("  sim  simulate (interpret) program");
}
#[derive(Debug)]
enum Mode {
    SIM,
}
fn cla(args: &Vec<String>) -> Result<Mode, i32> {
    let mut err: i32 = 0;
    if args.len() <= 1 {
        println!("No subcommand provided");
        usage();
        return Err({err += 1; err});
    }
    match args[1].as_str() {
        "sim" => {
            println!("Simulation is about to start");
            if args.len() <= 2 {
                println!("No source file provided");
                usage();
                return Err({err+=1; err});
            }
            return Ok(Mode::SIM);
        },
        _ => {
            println!("Unknown subcommand: `{}`", args[1]);
            usage();
            return Err({err+=1; err});
        },
    }
}

fn get(name: &String) -> Option<String> {
    match std::fs::read_to_string(name) {
        Ok(x) => Some(x),
        Err(_) => {
            println!("Cannot read file `{}`", name);
            None
        },
    }
}

#[derive(Clone)]
struct Loc (i64, i64);
struct Tok (Loc, String);

fn lex(file: &String) -> Vec<Tok> {
    let mut res: Vec<Tok> = vec![];
    let mut tmp: String = "".to_owned();
    let mut ploc: Loc = Loc(1, 1);
    let mut loc:  Loc = Loc(1, 1);
    for i in file.chars() {
        loc.1 += 1;
        if i == '\n' {
            loc.0 += 1;
            loc.1  = 1;
            res.push(Tok(ploc, tmp.to_owned()));
            tmp = "".to_owned();
            ploc = loc.clone();
            continue;
        }
        if i == ' ' || i == '\t' {
            if tmp.len() > 0 {
                res.push(Tok(ploc, tmp.to_owned()));
                ploc = loc.clone();
                tmp = "".to_owned();
            }
            continue;
        }
        tmp.push(i);
    }
    if tmp.len() > 0 {
        res.push(Tok(ploc, tmp.to_owned()));
    }
    return res;
}

fn strtoi64(x: &String) -> Option<i64> {
    let mut res: i64 = 0;
    let mut reversed: bool = false;
    let mut wasdigit: bool = false;
    for i in x.chars() {
        res = res*10 + match i {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '-' => 0,
            '+' => 0,
            _ => return None,
        };
        if i == '-' || i == '+' {
            if wasdigit {
                return None;
            }
            if i == '-' {
                reversed = !reversed;
            }
        } else {
            wasdigit = true;
        }
    }
    if !wasdigit {
        return None;
    }
    return Some(if reversed {
        -1
    } else {
        1
    } * res);
}

//////////////////////////////////////////////////
#[derive(Debug)] enum Op {
    Push(i64),
    PRINT,
    PLUS,
}
fn parse(pr: &Vec<Tok>) -> Option<Vec<Op>> {
    let mut res: Vec<Op> = vec![];
    for i in pr {
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let ind: &i64 = &loc.1;
        println!("val=`{}`, loc.lin={}, loc.ind={}", val, lin, ind);
        res.append(&mut match strtoi64(val) {
            Some(x) => vec![
                Op::Push(x),
            ],
            None => {
                match val.as_str() {
                    "" => {
                        continue;
                    },
                    "+f" => vec![
                        Op::PLUS,
                    ],
                    "putc" => vec![
                        Op::PRINT,
                    ],
                    _ => {
                        println!("Unknown token: `{}`", val);
                        return None;
                    },
                }
            }
        });
    }
    return Some(res);
}

fn sim(pr: &Vec<Op>) -> i32 {
    let mut stack: Vec<i64> = vec![];
    for i in pr {
        println!("sim: {:?}", i);
        match i {
            Op::Push(x) => stack.push(*x),
            Op::PRINT => print!("print: {}", stack.pop().unwrap()),
            Op::PLUS => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(a + b)
            },
            _ => {
                println!("Unknown op: {:?}", i);
                return -1;
            },
        }
    }
    return 0;
}

fn clah(args: &Vec<String>) {
    match cla(args) {
        Ok(mode) => {
            println!("[command line arguments reading succed]");
            match mode {
                Mode::SIM => {
                    for i in &args[2..] {
                        let err: i32 = sim(&match parse(&lex(&match get(i) {
                            Some(x) => x,
                            None => continue,
                        })) {
                            Some(x) => {
                                println!("[Parsing succed]");
                                x
                            },
                            None => {
                                println!("[Parsing failed]");
                                continue;
                            },
                        });
                        if err == 0 {
                            println!("[Simulation of `{}` succed]", i);
                        } else {
                            println!("[Simulation of `{}` failed with exit code {}", i, err);
                        }
                    }
                },
                _ => {
                    println!("Unknown mode: {:?}", mode);
                },
            }
        },
        Err(x) => {
            print!("[command line arguments reading failed due to {} previous error", x);
            if x >= 2 {
                print!("s");
            }
            println!("]");
        }
    }
}

fn _main() {
    let args: Vec<String> = std::env::args().collect();
    clah(&args);
}

fn main() {
    _main();
}
