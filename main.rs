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
    println!("  sim, s                simulate (interpret) program");
    println!("  version, ver, v       print version information and exit");
    println!("  usage, u, help, h     print this message and exit");
    println!("NI = not implemented");
}
#[derive(Debug)]
enum Mode {
    NONE,
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
        "sim" | "s" => {
            if args.len() <= 2 {
                println!("No source file provided");
                usage();
                return Err({err+=1; err});
            }
            return Ok(Mode::SIM);
        },
        "version" | "ver" | "v" => {
            println!("F+, a stack-based interpreting programming language\n\
                     written on Rust to write compiling version of itself on itself");
            println!("version: 0.1.0");
            println!("download: https://github.com/TwoSpikes/fplus");
            println!("2022-2023 @ TwoSpikes");
            return Ok(Mode::NONE);
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

#[derive(Debug, Clone)]
enum Op {
    Push(i64),
    PRINT,
    PUTS,
    FLUSH,
    INP,
    PLUS,
    MUL,
    GIF,    //gotoif
    G,      //goto
    PUSHNTH,
    DROPNTH,
    NBROT,
    LT,
    EQ,
    NOT,
    OR,
    EXIT,
    PSTK,  //print stack
    PSTKE, //print stack & exit
    DBGMSG(Box<str>),
    DUMP,
    ARGC,
    ARGV,
    READ,  //read file to string
}
//////////////////////////////////////////////////
fn parse(pr: &Vec<Tok>, filename: &String) -> Option<Vec<(Op, Loc)>> {
    if false {
        println!("parsing: loc={:?} val={:?}", pr.iter().map(|x| vec![x.0.0, x.0.1]), pr.iter().map(|x| x.1.clone()));
    } else {
        println!("parsing...");
    }
    let mut res: Vec<(Result<Op, &str>, Loc)> = vec![];
    #[derive(Debug)]
    enum State {
        NONE,
        //label without definition
        LBL,
        //label with definition
        FN,
        DBGMSG,
    }
    let mut state: State = State::NONE;
    let mut labels: Vec<(&str, Option<i64>)> = Vec::new();
    let mut main: Option<usize> = None;
    //multi-line comment
    let mut mlc: u32 = 0;
    let mut callstk: Vec<Option<usize>> = Vec::new();
    let mut ind: isize = -1;
    while {ind+=1;ind} < pr.len().try_into().unwrap() {
        let i: &Tok = &pr[ind as usize];
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let index: &i64 = &loc.1;
        let parseerr = |msg: &str| {
            println!("{}:{}:{}: {}", filename, lin, index, msg);
            None
        };
        match val.as_str() {
            "/*" => {
                mlc += 1;
            },
            "*/" => {
                if mlc <= 0 {
                    return parseerr("comment underflow!");
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
                (Ok(Op::Push(match val.as_str().chars().nth(1) {
                    Some(x) => x,
                    None => ' ',
                } as i64)), *loc),
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
            let mut tmpres: Vec<(Result<Op, &str>, Loc)> = tmpStr.chars().take(postfix.unwrap()).collect::<String>().chars().rev().collect::<String>().chars().map(|x| (Ok(Op::Push(x as i64)), Loc(-1,-1))).collect();
            //println!("postfix is {} tmp is {:?}", postfix.unwrap(), tmp);
            match tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>().as_str() {
                "" => tmpres.push((Ok(Op::Push((val.len()-2).try_into().unwrap())), Loc(-1,-1))),
                "r" => {},
                "c" => tmpres.push((Ok(Op::Push(0)), Loc(-1,-1))),
                _ => {
                    println!("custom string postfixes are not implemented yet: {}", tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>());
                    return None;
                },
            }
            tmpres
        } else {match strtoi64(val) {
            Some(x) => {
                vec![
                    (Ok(Op::Push(x)), *loc),
                ]
            },
            None => {
                
                match state {
                    State::NONE =>
                vec![(match val.as_str() {
                    "" => continue,
                    "+" => Ok(Op::PLUS),
                    "*" => Ok(Op::MUL),
                    "putc" => Ok(Op::PRINT),
                    "puts" => Ok(Op::PUTS),
                    "flush" => Ok(Op::FLUSH),
                    "input" => Ok(Op::INP),
                    "lbl" => {
                        state = State::LBL;
                        continue;
                    },
                    "fn" => {
                        state = State::FN;
                        continue;
                    },
                    ":if" => Ok(Op::GIF),
                    ":" => {
                        res.push((Ok(Op::G), *loc));
                        if callstk.len() > 0 {
                            let callstktmp: i64 = callstk.pop().unwrap().unwrap().try_into().unwrap();
                            res.insert(callstktmp as usize, (Ok(Op::Push(callstktmp + (res.len() as i64 - callstktmp) + 1)), *loc));
                        }
                        continue;
                    },
                    "pushnth" => Ok(Op::PUSHNTH),
                    "dropnth" => Ok(Op::DROPNTH),
                    "nbrot" => Ok(Op::NBROT),
                    "<" => Ok(Op::LT),
                    "=" => Ok(Op::EQ),
                    "!" => Ok(Op::NOT),
                    "|" => Ok(Op::OR),
                    "exit" => Ok(Op::EXIT),
                    "??#" => Ok(Op::PSTK),
                    "???" => Ok(Op::PSTKE),
                    "dbgmsg" => {
                        state = State::DBGMSG;
                        continue;
                    },
                    "addr" => Ok(Op::Push(res.len().try_into().unwrap())),
                    "paddr" => {
                        println!("paddr: {}", res.len());
                        continue;
                    },
                    "paddre" => {
                        println!("paddre: {}", res.len());
                        ind = res.len().try_into().unwrap();
                        continue;
                    },
                    "dump" => Ok(Op::DUMP),
                    "call" => {
                        callstk.push(Some(res.len()+0));
                        continue;
                    },
                    "argc" => Ok(Op::ARGC),
                    "argv" => Ok(Op::ARGV),
                    "read" => Ok(Op::READ),
                    _ => Err(val.as_str()),
                }, *loc)],
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
                    },
                    State::DBGMSG => {
                        res.push((Ok(Op::DBGMSG(val.as_str().into())), *loc));
                        state = State::NONE;
                        continue;
                    },
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

    match link(&res, &labels, &main) {
        Some(x) => {
            println!("  [Linking succed]");
            Some(x)
        },
        None => {
            println!("[Linking failed]");
            None
        },
    }
}
fn link(res: &Vec<(Result<Op, &str>, Loc)>, labels: &Vec<(&str, Option<i64>)>, main: &Option<usize>) -> Option<Vec<(Op, Loc)>> {
    println!("  linking...");
    let mut linkres: Vec<(Op, Loc)> = Vec::new();
    let mut ind: i64 = -1;
    for i in res {
        ind += 1;
        let loc: Loc = i.1;
        match &i.0 {
            //simple operation
            Ok(x) => linkres.push((x.clone(), i.1)),
            //found label call
            Err(x) => {
                let mut ret: i64 = -1;
                //tring to find declaration
                for j in &*labels {
                    //if found by name
                    if String::from(j.0).eq(&String::from(*x)) {
                        match j.1 {
                            //found definition
                            Some(def) => {
                                linkres.push((Op::Push(def), loc));
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
    linkres.push((Op::Push(match main {
        Some(x) => *x as i64,
        None => 0,
    }), Loc(-2,-2)));
    return Some(linkres);
}

const SIM_DEBUG: bool = true;

fn sim(pr: &mut Vec<(Op, Loc)>,
       filename: &String,
       argv: Vec<String>) -> Option<i32> {
    println!("[simulation...]");
    let mut stack: Vec<i64> = vec![];
    let main: i64 = match pr.pop() {
        Some(x) => match x.0 {
            Op::Push(y) => {
                //println!("sim: debug: main is {}", y);
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
    while ind != pr.len().try_into().unwrap() {
        ind += 1;
        let i: &Op = &pr[{let tmp: usize = ind as usize; if tmp >= pr.len() {break;} else {tmp}}].0;
        //println!("{}: {:?}\n  {:?}", ind, i, stack);
        let loc: &Loc = &pr[ind as usize].1;
        let lin: i64 = loc.0;
        let index: i64 = loc.1;
        if SIM_DEBUG {
            println!("---- {},{}:{:?} ----\n{:?}", lin, index, i, stack);
        }
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
                if a >= stack.len().try_into().unwrap() {
                    println!("{}:{}: pushnth overflow: {} (stack length is {})", lin, index, a, stack.len());
                    return None;
                }
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
            Op::EQ => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push((b == a).try_into().unwrap());
            },
            Op::NOT => {
                let a: i64 = stack.pop().unwrap();
                stack.push((a == 0).try_into().unwrap());
            },
            Op::OR => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(((a != 0) || (b != 0)).try_into().unwrap());
            },
            Op::EXIT => {
                let a: i64 = stack.pop().unwrap();
                return Some(a.try_into().unwrap());
            },
            Op::PSTK => {
                if !SIM_DEBUG {
                    println!("{}:{}: pstk  {:?}", lin, index, stack);
                }
            },
            Op::PSTKE => {
                if !SIM_DEBUG {
                    println!("{}:{}: pstke {:?}", lin, index, stack);
                }
                return None;
            },
            Op::DUMP => {
                println!("dump: {}", stack.pop().unwrap());
            },
            Op::ARGC => {
                stack.push(argv.len().try_into().unwrap());
            },
            Op::ARGV => {
                let a: i64 = stack.pop().unwrap();
                for j in argv[a as usize].chars().rev() {
                    stack.push(j as i64);
                }
                stack.push(argv[a as usize].len().try_into().unwrap());
            },
            Op::READ => {
                let mut filename: String = "".to_owned();
                let filename_len: usize  = stack.pop().unwrap().try_into().unwrap();
                let mut ind: usize = 0;
                while {ind+=1;ind} < filename_len+1 {
                    let i: i64 = stack.pop().unwrap();
                    filename.push(i as u8 as char);
                }
                let file: String = match std::fs::read_to_string(filename) {
                    Ok(x) => x.chars().rev().collect(),
                    Err(x) => {
                        stack.push(-1);
                        continue;
                    },
                };
                stack.append(&mut file.chars().map(|x| x as i64).collect::<Vec<i64>>());
                stack.push(file.len().try_into().unwrap());
            },
            _ => {
                println!("Unknown op: {:?}", i);
                return None;
            },
        }
    }
    //println!("ind is {} len is {}", ind, pr.len());
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
                Mode::NONE => {
                    return;
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
