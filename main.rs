use std::io::Write;
use std::process::exit; 
use std::fs; 
use std::io; 
use std::io::Read;
use std::str::Chars; 
use std::convert::TryInto;

fn from(u: &String) -> Vec<i64> {
    let len: usize = u.len();
    let mut res: Vec<i64> = Vec::with_capacity(len);
    for x in u.chars() {
        res.push(x as i64);
    }
    res
}

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

#[derive(Debug)] enum Op {
    Push(i64),
    PRINT,
    PUTS,
    FLUSH,
    INP,
    PLUS,
    MUL,
    //gotoif
    GIF,
    //goto
    G,
    PUSHNTH,
    DROPNTH,
    NBROT,
    LT,
    NOT,
    EXIT,
    //print stack
    PSTK,
    //print stack & exit
    PSTKE,
}
//////////////////////////////////////////////////
fn parse(pr: &Vec<Tok>, filename: &String) -> Option<Vec<Op>> {
    let mut res: Vec<Result<Op, &str>> = vec![];
    #[derive(Debug)]
    enum State {
        NONE,
        //label without definition
        LBL,
        //label with definition
        FN,
        OPER,
    }
    let mut state: State = State::NONE;
    let mut labels: Vec<(&str, Option<i64>)> = Vec::new();
    let mut main: Option<usize> = None;
    //multi-line comment
    let mut mlc: u32 = 0;
    let mut callstk: Vec<Option<usize>> = Vec::new();
    for i in pr {
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let ind: &i64 = &loc.1;
        match val.as_str() {
            "/*" => {
                mlc += 1;
            },
            "*/" => {
                if mlc <= 0 {
                    println!("comment underflow!");
                    return None;
                }
                mlc -= 1;
                continue;
            },
            _ => {
                
            },
        }
        if mlc > 0 {
            continue;
        }

        res.append(&mut if val.as_str().chars().nth(0) == Some('\'') {
            vec![
                Ok(Op::Push(match val.as_str().chars().nth(1) {
                    Some(x) => x,
                    None => ' ',
                } as i64)),
            ]
        } else {match strtoi64(val) {
            Some(x) => {
                vec![
                    Ok(Op::Push(x)),
                ]
            },
            None => {
                
                match state {
                    State::NONE =>
                match val.as_str() {
                    "" => {
                        continue;
                    },
                    "+" => vec![
                        Ok(Op::PLUS),
                    ],
                    "*" => vec![
                        Ok(Op::MUL),
                    ],
                    "putc" => vec![
                        Ok(Op::PRINT),
                    ],
                    "puts" => vec![
                        Ok(Op::PUTS),
                    ],
                    "flush" => vec![
                        Ok(Op::FLUSH),
                    ],
                    "inp" => vec![
                        Ok(Op::INP),
                    ],
                    "lbl" => {
                        state = State::LBL;
                        continue;
                    },
                    "fn" => {
                        state = State::FN;
                        continue;
                    },
                    ":if" => vec![
                        Ok(Op::GIF),
                    ],
                    ":" => {
                        res.push(Ok(Op::G));
                        if callstk.len() > 0 {
                        println!("fucking callstk: {:?}", callstk);
                            let callstktmp: i64 = callstk.pop().unwrap().unwrap().try_into().unwrap();
                            res.insert(callstktmp as usize, Ok(Op::Push(callstktmp + (res.len() as i64 - callstktmp) + 1)));
                        }
                        continue;
                    },
                    "pushnth" => vec![
                        Ok(Op::PUSHNTH),
                    ],
                    "dropnth" => vec![
                        Ok(Op::DROPNTH),
                    ],
                    "nbrot" => vec![
                        Ok(Op::NBROT),
                    ],
                    "<" => vec![
                        Ok(Op::LT),
                    ],
                    "!" => vec![
                        Ok(Op::NOT),
                    ],
                    "exit" => vec![
                        Ok(Op::EXIT),
                    ],
                    "???" => vec![
                        Ok(Op::PSTKE),
                    ],
                    "??#" => vec![
                        Ok(Op::PSTK),
                    ],
                    "call" => {
                        callstk.push(Some(res.len()+0));
                        continue;
                    },
                    _ => vec![
                        Err(val.as_str()),
                    ],
                },
                    State::LBL => {
                        if let "main" = &*val.as_str() {
                            main = Some(res.len().try_into().unwrap());
                        }
                        labels.push((val.as_str(), None));
                        state = State::NONE;
                        continue;
                    },
                    State::FN => {
                        let pos: usize = match labels.iter().position(|x| String::from(x.0).eq(val)) {
                            Some(pos) => pos,
                            None => {
                        if let "main" = &*val.as_str() {
                            main = Some(res.len().try_into().unwrap());
                        }
                                labels.push((val.as_str(), Some(res.len().try_into().unwrap())));
                                state = State::NONE;
                                continue;
                            }
                        };
                        labels[pos].1 = Some(res.len().try_into().unwrap());
                        state = State::NONE;
                        continue;
                    }
                    _ => {
                        println!("Unknown state of parser (debug): `{:?}`", state);
                        return None;
                    },
                }
            }
        }});
    }
    //to avoid not founding labels
    labels.push(("", None));

    //linking
    println!("linking: labels={:?}\nres={:?}", labels, res);
    let mut linkres: Vec<Op> = Vec::new();
    let mut ind: i64 = -1;
    for i in res {
        ind += 1;
        match i {
            //simple operation
            Ok(x) => linkres.push(x),
            //found label call
            Err(x) => {
                let mut ret: i64 = -1;
                //tring to find declaration
                for j in &labels {
                    //if found by name
                    if String::from(j.0).eq(&String::from(x)) {
                        match j.1 {
                            //found defenition
                            Some(def) => {
                                linkres.push(Op::Push(def));
                            },
                            //not found definition
                            None => {
                                println!("label is declared, but has no definition");
                                return None;
                            }
                        }
                    } else {
                        ret += 1;
                    }
                }
                if ret >= <usize as TryInto<i64>>::try_into(labels.len()).unwrap() - 1 {
                    println!("label not found: {x}");
                    return None;
                }
            },
        };
    }
    linkres.push(Op::Push(match main {
        Some(x) => x.try_into().unwrap(),
        None => 0,
    }));
    return Some(linkres);
}

fn sim(pr: &mut Vec<Op>, filename: &String) -> Option<i32> {
   let mut stack: Vec<i64> = vec![];
    let main: i64 = match pr.pop() {
        Some(x) => match x {
            Op::Push(y) => {
                y
            },
            _ => {
                println!("sim: debug: main label not found");
                return None;
            }
        },
        None => return Some(0),
    };
    let mut ind: i64 = main - 1;
    while ind < pr.len().try_into().unwrap() {
        ind += 1;
        let i: &Op = &pr[{let tmp: usize = <i64 as TryInto<usize>>::try_into(ind).unwrap(); if tmp >= pr.len() {break;} else {tmp}}];
        //println!("{}: sim: {:?}", ind, i);
        match i {
            Op::Push(x) => {
                stack.push(*x);
            },
            Op::PRINT => {
                println!("print: debug: {:?}", stack);
                println!("print: {}", char::from_u32(stack.pop().unwrap().try_into().unwrap()).unwrap());
            },
            Op::PUTS => {
                //println!("puts: debug: {:?}", stack);
                let strlen: usize = stack.pop().unwrap().try_into().unwrap();
                let mut i: usize = 0;
                let mut string: String = "".to_owned();
                while i < strlen {
                    let chr = char::from_u32(stack.pop().unwrap().try_into().unwrap()).unwrap();
                    string.push(chr);
                    i += 1;
                }
                print!("{}", string);
            },
            Op::FLUSH => {
                std::io::stdout().flush();
            },
            Op::INP => {
                let mut input: String = String::new();
                let stdin = std::io::stdin();
                stdin.read_line(&mut input);
                stack.append(&mut from(&input));
                stack.push(input.len().try_into().unwrap());
            },
            Op::PLUS => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(a + b)
            },
            Op::MUL => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(a * b)
            },
            Op::GIF => {
                let addr: i64 = stack.pop().unwrap() - 1;
                let cond: i64 = stack.pop().unwrap();
                if cond != 0 {
                    ind = addr.try_into().unwrap();
                }
            },
            Op::G => {
                let addr: i64 = stack.pop().unwrap() - 1;
                ind = addr.try_into().unwrap();
            },
            Op::PUSHNTH => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack[stack.len()-1-a as usize];
                stack.push(b);
            },
            Op::DROPNTH => {
                let a: i64 = stack.pop().unwrap();
                stack.remove(stack.len()-1-a as usize);
            },
            Op::NBROT => {
                let l: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.insert(stack.len()-0-l as usize, a);
            },
            Op::LT => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push((b < a).try_into().unwrap());
            },
            Op::NOT => {
                let a: i64 = stack.pop().unwrap();
                stack.push((a == 0) as i64);
            },
            Op::EXIT => {
                let a: i64 = stack.pop().unwrap();
                return Some(a.try_into().unwrap());
            },
            Op::PSTK => {
                println!("pstk {:?}", stack);
            },
            Op::PSTKE => {
                println!("pstke {:?}", stack);
                return None;
            },
            _ => {
                println!("Unknown op: {:?}", i);
                return None;
            },
        }
    }
    return Some(0);
}

fn clah(args: &Vec<String>) {
    match cla(args) {
        Ok(mode) => {
            println!("[command line arguments reading succed]");
            match mode {
                Mode::SIM => {
                    for i in &args[2..] {
                        let err: Option<i32> = sim(&mut match parse(&lex(&match get(i) {
                            Some(x) => x,
                            None => continue,
                        }), i) {
                            Some(x) => {
                                println!("[Parsing succed]");
                                x
                            },
                            None => {
                                println!("[Parsing failed]");
                                continue;
                            },
                        }, i);
                        println!("");
                        match err {
                            Some(x) => {
                                if x == 0 {
                                    println!("[Simulation of `{}` succed]", i);
                                } else {
                                    println!("[Simulation of `{}` was finished with exit code {}]", i, x);
                                }
                            },
                            None => {
                                println!("[Simulation of `{}` failed]", i);
                            }
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
