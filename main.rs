use std:: {
    io::Write,
    convert::TryInto,
    fmt
};


fn linkparselexget(filename: &String) -> Option<Vec<(Op, Loc)>> {
    match parse(&{
use crate::Retlex::EMPTY;
use crate::Retlex::N;
use crate::Retlex::E;
        match lex(&match get(&filename) {
        Some(x) => x,
        None => {
            return None;
        },
    }) {
            EMPTY => {
                println!("[empty file]");
                return None;
            },
            E => {
                println!("[lexing failed]");
                return None;
            },
            N(x) => x,
            _ => {
                println!("Unknown lexing return state");
                return None;
            },
    }}, &filename) {
        Some(x) => {
            println!("[Parsing succed]");
            match link(&x.0, &x.1, &x.2, &x.3) {
                Some(x) => {
                    println!("[linking succed]");
                    Some(x)
                },
                None => {
                    println!("[linking failed]");
                    return None;
                },
            }
        },
        None => {
            println!("[Parsing failed]");
            return None;
        },
    }
}

fn for_each_arg(args: &Vec<String>,
                argv: &Vec<String>,
                fargs: &mut Vec<String>,
                func: fn(error: simResult,
                         i: &String) -> ()) {
    let mut ind: isize = -1;
    while {ind+=1;ind}<argv.len().try_into().unwrap() {
        let i: String = argv[ind as usize].clone();
        fargs.insert(0, args[0].clone());
        let error: simResult = sim(&mut match linkparselexget(&i) {
            Some(x) => x,
            None => continue,
        }, &i, if ind==(argv.len()-1).try_into().unwrap() {
            fargs.clone()
        } else {
            vec![
                args[0].clone(),
            ]
        });
        println!();
        func(error, &i);
    }
}

fn strcat(a: &str, b: &str) -> String {
    let mut res: String = "".to_owned();
    for i in a.chars() {
        res.push(i);
    }
    for i in b.chars() {
        res.push(i);
    }
    return res;
}

fn repr(string: &str) -> String {
    let mut res: String = "\"".to_owned();
    for i in string.chars() {
        res += &match i {
            '\n' => vec!['\\', 'n'],
            '\t' => vec!['\\', 't'],
            '\\' => vec!['\\', '\\'],
            '\'' => vec!['\\', '\''],
            '\"' => vec!['\\', '\"'],
            _ => vec![i],
        }.iter().collect::<String>();
    }
    res.push('\"');
    return res;
}
fn urepr(string: &str) -> String {
    let mut res: String = "".to_owned();
    let mut ind: isize = -1;
    while {ind+=1;ind} < string.len()as isize {
        let i: char = match string.chars().nth(ind as usize) {
            Some(x) => x,
            None => break,
        };
        res += &match i {
            '\\' => {
                res += &vec![match match string.chars().nth((ind+1)as usize) {
                    Some(x) => x,
                    _ => panic!("Escape character not found"),
                }{
                'n' => '\n',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '\"' => '\"',
                _ => {
                    panic!("Unknown escaping character: {}", vec![i, string.chars().nth((ind+1)as usize).unwrap()].iter().collect::<String>());
                },
            }].iter().collect::<String>();
                ind += 1;
                continue;
            },
            _ => vec![i],
        }.iter().collect::<String>();
    }
    return res;
}

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
    println!("  usage, use, u, help, h, ?, info, information");
    println!("                        print this message and exit");
}
fn version() {
    println!("F+, a stack-based interpreting programming language\n\
                     written on Rust v.1.67.1");
    println!("version: 0.1.0");
    println!("download: https://github.com/TwoSpikes/fplus");
    println!("2022-2023 @ TwoSpikes");
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
            version();
            return Ok(Mode::NONE);
        },
        "usage" | "use" | "u" | "help" | "h" | "?" | "info" | "information" => {
            usage();
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
            return None;
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
        //'\n' then push '\n'
        if i == '\n' || i == ':' {
            loc.0 += 1;
            loc.1  = 1;
            res.push(Tok(ploc, tmp.to_owned()));
            res.push(Tok(loc, String::from(i)));
            tmp = "".to_owned();
            ploc = loc.clone();
            continue;
        }
        //' ' or '\t' then push tmp
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
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
//////////////////////////////////////////////////
fn parse(pr: &Vec<Tok>, filename: &String) -> Option<(String, Vec<(Result<Op, String>, Loc)>, Vec<(String, Option<i64>)>, Option<usize>)> {
    if false {
        println!("[parsing loc={:?} val={:?}]", pr.iter().map(|x| vec![x.0.0, x.0.1]), pr.iter().map(|x| x.1.clone()));
    } else {
        println!("[parsing...]");
    }
    let mut res: Vec<(Result<Op, String>, Loc)> = vec![];
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
    let mut labels: Vec<(String, Option<i64>)> = Vec::new();
    let mut main: Option<usize> = None;
    //multi-line comment
    let mut mlc: u32 = 0;
    let mut callstk: Vec<Option<usize>> = Vec::new();
    let mut ind: isize = -1;
    while {ind+=1;ind} < pr.len()as isize{
        let i: &Tok = &pr[ind as usize];
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let index: &i64 = &loc.1;
        let parseerr = |msg: &str| {
            println!("{}:{}:{}: Error: {}", filename, lin, index, msg);
            return None;
        };
        let parsewarn = |msg: &str| {
            println!("{}:{}:{}: Warning: {}", filename, lin, index, msg);
        };
        match val.as_str() {
            "/*" => {
                mlc += 1;
            },
            "*/" => {
                if mlc <= 0 {
                    return parseerr("Comment underflow!");
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

        res.append(&mut if val.as_str().chars().nth(0) == Some('\'') && matches!(state, State::NONE) {
            vec![
                (Ok(Op::Push(match val.as_str() {
                    "'" => ' ',
                    _ => {
                        let repred_string: String = urepr(&val[1..]);
                        if repred_string.len() > 1 {
                            parseerr("Char is more than one symbol");
                            return None;
                        }
                        repred_string.chars().nth(0).unwrap()
                    },
                } as i64)), *loc),
            ]
        } else if val.chars().nth(0) == Some('\"') && matches!(state, State::NONE) {
            let mut postfix: Option<usize> = None;
            let mut tmp: Vec<i64> = {
                let mut res: Vec<i64> = Vec::new();
                let mut jnd: isize = -1;
                let mut j: char = ' ';
                while {jnd+=1;jnd} < val[1..].len()as isize {
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
            let tmpStr: String = urepr(tmp.iter().map(|x| char::from(*x as u8)).collect::<Vec<char>>().iter().collect::<String>().as_str());
            let tmpstr: &str = tmpStr.as_str();
            let mut tmpres: Vec<(Result<Op, String>, Loc)> = tmpStr.chars().take(postfix.unwrap()).collect::<String>().chars().rev().collect::<String>().chars().map(|x| (Ok(Op::Push(x as i64)), Loc(-1,-1))).collect();
            match tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>().as_str() {
                "" => tmpres.push((Ok(Op::Push((tmpStr.len()).try_into().unwrap())), Loc(-1,-1))),
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
            None => { match state {
                    State::NONE =>
                vec![(match val.as_str() {
                    ""|"\n" => continue,
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
                    "if" => Ok(Op::GIF),
                    ":" => {
                        res.push((Ok(Op::G), *loc));
                        if callstk.len() > 0 {
                            let callstktmp: i64 = callstk.pop().unwrap().unwrap()as i64;
                            res.insert(callstktmp as usize, (Ok(Op::Push(callstktmp + (res.len()as i64 - callstktmp) + 1)), *loc));
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
                    "addr" => Ok(Op::Push(res.len()as i64)),
                    "paddr" => {
                        println!("paddr: {}", res.len());
                        continue;
                    },
                    "paddre" => {
                        println!("paddre: {}", res.len());
                        ind = res.len()as isize;
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
                    _ => Err(val.to_string()),
                }, *loc)],
                    State::LBL => {
                        if let "main" = &*val.as_str() {
                            main = Some(res.len()as usize);
                        }
                        labels.push((val.to_string(), None));
                        state = State::NONE;
                        continue;
                    },
                    State::FN => {
                        let pos: usize = match labels.iter().position(|x| String::from(x.0.clone()).eq(val)) {
                            Some(pos) => pos,
                            None => {
                                if let "main" = &*val.as_str() {
                                    main = Some(res.len()as usize);
                                }
                                labels.push((val.to_string(), Some(res.len()as i64)));
                                state = State::NONE;
                                continue;
                            }
                        };
                        labels[pos].1 = Some(res.len()as i64);
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
    let parseerr = |msg: &str| {
        println!("{}:EOF: Error: {}", filename, msg);
        return None::<(String, Vec<(Result<Op, String>, Loc)>, Vec<(String, Option<i64>)>, Option<usize>)>;
    };
    let parsewarn = |msg: &str| {
        println!("{}:EOF: Warning: {}", filename, msg);
    };
    if !matches!(state, State::NONE) {
        return parseerr("Parsing is ended but state is not none");
    }
    //to avoid not founding labels
    labels.push(("".to_string(), None));

    return Some((filename.to_string(), res, labels, main));
}
fn link(filename: &String, res: &Vec<(Result<Op, String>, Loc)>, labels: &Vec<(String, Option<i64>)>, main: &Option<usize>) -> Option<Vec<(Op, Loc)>> {
    println!("[linking...]");
    let mut linkres: Vec<(Op, Loc)> = Vec::new();
    let mut ind: i64 = -1;
    for i in res {
        ind += 1;
        let loc: Loc = i.1;
        let lin: i64 = loc.0;
        let index: i64 = loc.1;
        match &i.0 {
            //simple operation
            Ok(x) => linkres.push((x.clone(), i.1)),
            //found label call
            Err(x) => {
                let mut ret: i64 = -1;
                //tring to find declaration
                for j in &*labels {
                    //if found by name
                    if String::from(j.0.clone()).eq(&String::from(x.clone())) {
                        match j.1 {
                            //found definition
                            Some(def) => {
                                linkres.push((Op::Push(def), loc));
                            },
                            //not found definition
                            None => {
                                println!("{}:{}:{}: label is declared, but has no definition", filename, lin, index);
                                return None;
                            }
                        }
                    } else {
                        ret += 1;
                    }
                }
                if ret >= labels.len()as i64 - 1 {
                    println!("{}:{}:{}: label not found: {}", filename, lin, index, repr(x));
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

const SIM_DEBUG: bool = false;

#[derive(Debug)]
enum simResult {
    ok(i32),
    err,
    errs(String),
    stopped,
}
fn sim(pr: &mut Vec<(Op, Loc)>,
       filename: &String,
       argv: Vec<String>) -> simResult {
use simResult::*;
    println!("[simulation...]");
    let mut stack: Vec<i64> = vec![];
    let main: i64 = match pr.pop() {
        Some(x) => match x.0 {
            Op::Push(y) => {
                //println!("sim: debug: main is {}", y);
                y
            },
            _ => {
                return errs("main label not found".to_owned());
            }
        },
        None => return ok(0),
    };
    let mut ind: i64 = main - 1;
    while ind != pr.len()as i64 {
        ind += 1;
        let i: &Op = &pr[{let tmp: usize = ind as usize; if tmp >= pr.len() {break;} else {tmp}}].0;
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
                print!("{}", char::from_u32(stack.pop().unwrap()as u32).unwrap());
            },
            Op::PUTS => {
                if SIM_DEBUG {
                    println!("debug: puts: {:?}", stack);
                }
                let strlen: usize = stack.pop().unwrap()as usize;
                let mut i: usize = 0;
                let mut string: String = "".to_owned();
                while i < strlen {
                    let chr = char::from_u32(stack.pop().unwrap()as u32).unwrap();
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
                stack.push(input.len()as i64);
            },
            Op::PLUS => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for PLUS intrinsic not found".to_string());
                    },
                };
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for PLUS intrinsic not found".to_string());
                    },
                };
                stack.push(a + b)
            },
            Op::MUL => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for MUL intrinsic not found".to_string());
                    },
                };
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for MUL intrinsic not found".to_string());
                    },
                };
                stack.push(a * b)
            },
            Op::GIF => {
                let addr: i64 = match stack.pop() {
                    Some(x) => x-1,
                    None => {
                        return errs("Operand `addr` for GIF intrinsic not found".to_string());
                    },
                };
                let cond: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `cond` for GIF intrinsic not found".to_string());
                    },
                };
                if cond != 0 {
                    ind = addr.try_into().unwrap();
                }
            },
            Op::G => {
                let addr: i64 = match stack.pop() {
                    Some(x) => x-1,
                    None => {
                        return errs("Operand `addr` for G intrinsic not found".to_string());
                    },
                };
                ind = addr.try_into().unwrap();
            },
            Op::PUSHNTH => {
                let a: i64 = stack.pop().unwrap();
                if a >= stack.len().try_into().unwrap() {
                    return errs("pushnth overflow".to_owned());
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
                return ok(a.try_into().unwrap());
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
                return err;
            },
            Op::DUMP => {
                println!("dump: {}", match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand for DUMP intrinsic not found".to_owned());
                    },
                });
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
            Op::DBGMSG(x) => {
                println!("dbgmsg: {}", repr(x));
            },
            _ => {
                return errs(strcat("unknown op: ", &i.to_string()));
            },
        }
    }
    return ok(0);
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
                    for_each_arg(&args, &argv, &mut fargs,
                                 |error: simResult, i: &String| {
use simResult::*;
                        match error {
                            ok(x) => {
                                if x == 0 {
                                    println!("[Simulation of {} succed]", repr(&i));
                                } else {
                                    println!("[Simulation of {} was finished with exit code {}]", repr(&i), x);
                                }
                            },
                            err => {
                                println!("[Simulation of {} failed]", repr(&i));
                            },
                            errs(x) => {
                                println!("[Simulation of {} failed due to this error: {}]", repr(&i), repr(&x));
                            },
                            stopped => {

                            },
                            _ => {
                                println!("[Simulation of {}: Internal error: Unknown  state: {:?}]", repr(&i), err);
                            },
                        }
                    });
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

fn _test() {
}

fn _main() {
    let args: Vec<String> = std::env::args().collect();
    clah(&args);
}

fn main() {
    _main();
}
