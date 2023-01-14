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
    //gotoif
    GIF,
}
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
    for i in pr {
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let ind: &i64 = &loc.1;
        //println!("val=`{}`, loc.lin={}, loc.ind={}", val, lin, ind);
        res.append(&mut match strtoi64(val) {
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
                    "+f" => vec![
                        Ok(Op::PLUS),
                    ],
                    "putc" => vec![
                        Ok(Op::PRINT),
                    ],
                    "lbl" => {
                        state = State::LBL;
                        continue;
                    },
                    "impl" => {
                        state = State::FN;
                        continue;
                    },
                    ":if" => vec![
                        Ok(Op::GIF),
                    ],
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
                                println!("{}:{}:{}: label declaration to implement not found: {}", filename, lin, ind, val);
                                return None;
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
        });
    }
    //to avoid not founding labels
    labels.push(("", None));

    //linking
    println!("Linking... labels={:?}\nres={:?}", labels, res);
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
                let mut found: bool = false;
                //tring to find declaration
                for j in &labels {
                    //if found by name
                    if String::from(j.0).eq(&String::from(x)) {
                        found = true;
                        match j.1 {
                            //found defenition
                            Some(def) => {
                                println!("found label definition: {}", def);
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
    println!("simulating {}: {:?}", filename, pr);
    let mut stack: Vec<i64> = vec![];
    let main: i64 = match pr.pop() {
        Some(x) => match x {
            Op::Push(y) => {
                println!("sim: debug: main label was found: {}", y);
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
        let i: &Op = &pr[<i64 as TryInto<usize>>::try_into(ind).unwrap()];
        println!("sim:{}: {:?}", ind, i);
        match i {
            Op::Push(x) => {
                stack.push(*x);
            },
            Op::PRINT => {
                print!("print: {}", char::from_u32(stack.pop().unwrap().try_into().unwrap()).unwrap());
            },
            Op::PLUS => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(a + b)
            },
            Op::GIF => {
                let addr: i64 = stack.pop().unwrap() - 1;
                let cond: i64 = stack.pop().unwrap();
                println!("sim: debug: ifgotoifing to {} if {}", addr, cond);
                if cond != 0 {
                    println!("sim: debug: gotoifing is true stack={:?}", stack);
                    ind = addr.try_into().unwrap();
                }
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
                        match err {
                            Some(x) => {
                                if x == 0 {
                                    println!("[Simulation of `{}` succed]", i);
                                } else {
                                    println!("[Simulation of `{}` was finished with exit code {}]", i, x);
                                }
                            },
                            None => {
                                println!("[Simulation of `{}` failed", i);
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
