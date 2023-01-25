use std::io::Write;
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

#[derive(Debug, Clone, Copy)]
struct Loc (i64, i64);
#[derive(Debug)]
struct Tok (Loc, String);

#[derive(Debug)]
enum Retlex {
    //normal
    N(Vec<Tok>),
    //error
    E,
    //empty file
    EMPTY,
}
fn lex(file: &String) -> Retlex {
    use crate::Retlex::EMPTY;
    use crate::Retlex::N;
    use crate::Retlex::E;
    if file.len() == 0 {
        return EMPTY;
    }
    let mut res: Vec<Tok> = vec![];
    let mut tmp: String = "".to_owned();
    let mut ploc: Loc = Loc(1, 1);
    let mut loc:  Loc = Loc(1, 1);
    #[derive(Debug)]
    enum Quotes {
        NO,
        IN,
        POSTF,
    }
    let mut quotes: Quotes = Quotes::NO;
    for i in file.chars() {
        loc.1 += 1;
        //if '"'  then remember it
        if i == '"' {
            tmp.push(i);
            quotes = match quotes {
                Quotes::NO => Quotes::IN,
                Quotes::IN => Quotes::POSTF,
                Quotes::POSTF => {
                    res.push(Tok(ploc, tmp.to_owned()));
                    tmp = "".to_owned();
                    ploc = loc.clone();
                    Quotes::NO
                },
                _ => {
                    println!("lex: unknown quotes: {:?}", quotes);
                    return E;
                },
            };
            continue;
        }
        match quotes {
            Quotes::NO => {
                
            },
            Quotes::IN => {
                tmp.push(i);
                continue;
            },
            Quotes::POSTF => {
                if i == '\n' || i == ' ' {
                    quotes = Quotes::NO;
                    res.push(Tok(ploc, tmp.to_owned()));
                    tmp = "".to_owned();
                    ploc = loc.clone();
                } else {
                    tmp.push(i);
                }
                continue;
            },
            _ => {
                println!("lex: unknown quotes: {:?}", quotes);
                return E;
            },
        }
        //if '\n' then just push it
        if i == '\n' {
            loc.0 += 1;
            loc.1  = 1;
            res.push(Tok(ploc, tmp.to_owned()));
            tmp = "".to_owned();
            ploc = loc.clone();
            continue;
        }
        //if ' ' or '\t' then push tmp
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
    return N(res);
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
    DUMP,
    ARGC,
    ARGV,
}
//////////////////////////////////////////////////
fn parse(pr: &Vec<Tok>, filename: &String) -> Option<Vec<Op>> {
    println!("parse loc={:?} val={:?}", pr.iter().map(|x| vec![x.0.0, x.0.1]), pr.iter().map(|x| x.1.clone()));
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
        } else if val.chars().nth(0) == Some('\"') {
            let mut postfix: Option<usize> = None;
            let mut tmp: Vec<i64> = {
                let mut res: Vec<i64> = Vec::new();
                let mut jnd: isize = -1;
                let mut j: char = ' ';
                while {jnd+=1;jnd} < val[1..].len().try_into().unwrap() {
                    j = match val[1..].chars().nth(jnd as usize) {
                        Some(x) => x,
                        None => {
                            break;
                        },
                    };
                    if j == '"' {
                        postfix = Some(jnd as usize);
                        continue;
                    }
                    res.push(j as i64);
                }
                res
            };
            let tmpStr: String = tmp.iter().map(|x| char::from(*x as u8)).collect::<Vec<char>>().iter().collect::<String>();
            let tmpstr: &str = tmpStr.as_str();
            let mut tmpres: Vec<Result<Op, &str>> = tmpStr.chars().take(postfix.unwrap()).collect::<String>().chars().rev().collect::<String>().chars().map(|x| Ok(Op::Push(x as i64))).collect();
            println!("postfix is {} tmp is {:?}", postfix.unwrap(), tmp);
            match tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>().as_str() {
                "" => tmpres.push(Ok(Op::Push((val.len()-2).try_into().unwrap()))),
                "r" => {},
                "c" => tmpres.push(Ok(Op::Push(0))),
                _ => {
                    println!("custom string postfixes are not implemented yet: {}", tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>());
                    return None;
                },
            }
            tmpres
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
                    "input" => vec![
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
                    "dump" => vec![
                        Ok(Op::DUMP),
                    ],
                    "call" => {
                        callstk.push(Some(res.len()+0));
                        continue;
                    },
                    "argc" => vec![
                        Ok(Op::ARGC),
                    ],
                    "argv" => vec![
                        Ok(Op::ARGV),
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
    println!("linking...");
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

fn sim(pr: &mut Vec<Op>,
       filename: &String,
       argv: Vec<String>) -> Option<i32> {
    //println!("sim: argv: {:?}", argv);
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
                print!("{}", char::from_u32(stack.pop().unwrap().try_into().unwrap()).unwrap());
            },
            Op::PUTS => {
                //println!("debug: puts: {:?}", stack);
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
                stack.append(&mut from(&input).iter().rev().map(|x| *x).collect::<Vec<i64>>());
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
                //println!("debug: g: addr={} stk={:?}", addr, stack);
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
                stack.push((a == 0).try_into().unwrap());
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
            Op::DUMP => {
                println!("dump: {}", stack.pop().unwrap());
            },
            Op::ARGC => {
                stack.push(argv.len().try_into().unwrap());
            },
            Op::ARGV => {
                println!("argv {:?}", stack);
                let a: i64 = stack.pop().unwrap();
                for j in argv[a as usize].chars() {
                    stack.push(j as i64);
                }
                stack.push(argv[a as usize].len().try_into().unwrap());
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
                let mut argv: Vec<String> = Vec::new();
                //fucking arguments
                let mut fargs: Vec<String> = Vec::new();
                {
                    let mut i: String = "".to_owned();
                    let mut ind: isize = 1;
                    let mut isargs: bool = false;
                    while {ind+=1;ind} < args.len().try_into().unwrap() {
                        i = args[ind as usize].clone();
                        if i == "--" {
                            isargs = true;
                            continue;
                        }
                        if isargs {
                            fargs.push(i);
                            continue;
                        }
                        argv.push(i);
                    }
                }
                {
                    let mut ind: isize = -1;
                    let mut i: String = "".to_owned();
                    while {ind+=1;ind}<argv.len().try_into().unwrap() {
                        i = argv[ind as usize].clone();
                        fargs.insert(0, args[0].clone());
                        let err: Option<i32> = sim(&mut match parse(&{
    use crate::Retlex::EMPTY;
    use crate::Retlex::N;
    use crate::Retlex::E;
                            match lex(&match get(&i) {
                            Some(x) => x,
                            None => continue,
                        }) {
                                EMPTY => {
                                    println!("[empty file]");
                                    continue;
                                },
                                E => {
                                    println!("[lexing failed]");
                                    continue;
                                },
                                N(x) => x,
                                _ => {
                                    println!("Unknown lexing return state");
                                    continue;
                                },
                        }}, &i) {
                            Some(x) => {
                                println!("[Parsing succed]");
                                x
                            },
                            None => {
                                println!("[Parsing failed]");
                                continue;
                            },
                        }, &i, if ind==(argv.len()-1).try_into().unwrap() {
                            fargs.clone()
                        } else {
                            vec![
                                args[0].clone(),
                            ]
                        });
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
