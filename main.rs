use std:: {
    io::Write,
    convert::TryInto,
    fmt
};


// -- simulating --
//show every token on runtime and stack state
const SIM_DEBUG: bool = false;
//show stack state in puts command
const SIM_DEBUG_PUTS: bool = false;

// -- linking --
//Show resulting program
const LINK_DEBUG: bool = false;
//Stop on linking, do not run
//(e.g. when infinite loop)
const ONLY_LINK: bool = false;
//print message "[linking succed]"
const LINK_DEBUG_SUCCED: bool = true;

// -- parsing --
//show every token and some variables for parsing
const PARSE_DEBUG: bool = false;
//show callstack
const PARSE_DEBUG_CALL: bool = false;
//print message "[Parsing succed]"
const PARSE_DEBUG_SUCCED: bool = true;
//callmode without # operator
const CALLMODE_DEFAULT: Callmode = Callmode::WITH_ADDRESS_LEFT;
//callmode with # operator
const CALLMODE_ON_OPERATOR: Callmode = Callmode::WITHOUT_ADDRESS;

#[derive(Debug)]
enum Callmode {
    WITHOUT_ADDRESS,    //like goto operator in C
                        //or jmp operator in asm
    WITH_ADDRESS_LEFT,  //save address in the top of the stack
                        //to jump there in the end of function
    WITH_ADDRESS_RIGHT, //save address before arguments
}

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
                eprintln!("[empty file]");
                return None;
            },
            E => {
                eprintln!("[lexing failed]");
                return None;
            },
            N(x) => x,
            _ => {
                eprintln!("Unknown lexing return state");
                return None;
            },
    }}, &filename) {
        Some(x) => {
            if PARSE_DEBUG_SUCCED {
                eprintln!("[Parsing succed]");
            }
            match link(&x.0, &x.1, &x.2, &x.3) {
                Some(x) => {
                    if LINK_DEBUG_SUCCED {
                        eprintln!("[linking succed]");
                    }
                    Some(x)
                },
                None => {
                    eprintln!("[linking failed]");
                    return None;
                },
            }
        },
        None => {
            eprintln!("[Parsing failed]");
            return None;
        },
    }
}

fn for_each_arg(args: &Vec<String>,
                func: fn(i: &String,
                         ind: isize,
                         argv: &Vec<String>,
                         fargs: &Vec<String>,
                         args: &Vec<String>,
                         output_to_file: Option<String>) -> ()) {
    let mut argv: Vec<String> = Vec::new();
    //fucking arguments
    let mut fargs: Vec<String> = Vec::new();
    let mut output_to_file: Option<Option<String>> = None;
    {
        let mut i: String = "".to_owned();
        let mut ind: isize = 1;
        let mut isargs: bool = false;
        while {ind+=1;ind} < args.len().try_into().unwrap() {
            i = args[ind as usize].clone();

            match output_to_file.clone() {
                Some(x) => {
                    match x {
                        Some(_) => {},
                        None => {
                            output_to_file = Some(Some(i.clone()));
                            continue;
                        },
                    }
                },
                None => {},
            }

            if i == "--" {
                isargs = true;
                continue;
            }
            if isargs {
                fargs.push(i);
                continue;
            }
            if (i == "-o") | (i == "--output") {
                output_to_file = Some(None);
                continue;
            }
            argv.push(i);
        }
    }
    let mut ind: isize = -1;
    while {ind+=1;ind}<argv.len().try_into().unwrap() {
        let i: String = argv[ind as usize].clone();
        fargs.insert(0, args[0].clone());
        func(&i, ind, &argv, &fargs, &args, match output_to_file {
            Some(ref x) => {
                match x {
                    Some(y) => Some(y.to_string()),
                    None => {
                        eprintln!("No argument for \"-o\" option was provided");
                        usage();
                        break;
                    },
                }
            },
            None => None,
        });
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
                    panic!("Unknown escaping character: \'{}\'", vec![i, string.chars().nth((ind+1)as usize).unwrap()].iter().collect::<String>());
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
    println!("Usage:
$ ./target/release/fplus SUBCOMMAND [OPTION]... [SOURCE]... -- [ARG]...

SUBCOMMAND:
sim s                 Simulate program
version ver v         Print version information and exit
usage use u help h ? info information
		      Print help information and exit
dump d                Dump the tokens of the program.
error e               Print error code and information about them");
}
fn version() {
    println!("F+, a stack-based interpreting programming language
written on Rust v.1.68.0
version: 0.1.0-4
download: https://github.com/TwoSpikes/fplus
2022-2023 @ TwoSpikes");
}
fn errorcodes() {
    println!("errorcodes:
E0                    Cannot open file");
}

#[derive(Debug)]
enum Mode {
    NONE,
    SIM,
    DUMP,
    ERRCODES,
}
fn cla(args: &Vec<String>) -> Result<Mode, i32> {
    let mut err: i32 = 0;
    if args.len() <= 1 {
        eprintln!("No subcommand provided");
        usage();
        return Err({err += 1; err});
    }
    match args[1].as_str() {
        "sim"|"s" => {
            if args.len() <= 2 {
                eprintln!("No source file provided");
                usage();
                return Err({err+=1; err});
            }
            return Ok(Mode::SIM);
        },
        "version"|"ver"|"v" => {
            version();
            return Ok(Mode::NONE);
        },
        "usage"|"use"|"u"|"help"|"h"|"?"|"info"|"information" => {
            usage();
            return Ok(Mode::NONE);
        },
        "dump"|"d" => {
            return Ok(Mode::DUMP);
        },
        "error"|"e" => {
            errorcodes();
            return Ok(Mode::NONE);
        },
        _ => {
            eprintln!("Unknown subcommand: \"{}\"", args[1]);
            usage();
            return Err({err+=1; err});
        },
    }
}

fn get(name: &String) -> Option<String> {
    match std::fs::read_to_string(name) {
        Ok(x) => Some(x),
        Err(_) => {
            eprintln!("Cannot read file \"{}\"", name);
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
                    eprintln!("lex: unknown quotes: {:?}", quotes);
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
                eprintln!("lex: unknown quotes: {:?}", quotes);
                return E;
            },
        }
        //push special symbols as special symbols
        if i == '\n' || i == ':' || i == '(' || i == ')' || i == '#' {
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
    Push(i64), //push number to the stack
    PRINT,  //print char (same as number)
    PUTS,   //print string
    FLUSH,  //print stdout buffer and clear it
    INP,    //read line from stdin
    PLUS,   // +
    MUL,    // *
    GIF,    //gotoif
    G,      //goto
    PUSHNTH,//copy nth element to the top
    DROPNTH,//remove nth element
    NBROT,  //move top of the stack to n elements to left
    LT,     // <
    EQ,     // ==
    NOT,    // !
    OR,     // ||
    EXIT,   //exit the program
    PSTK,   //print stack
    PSTKE,  //print stack & exit
    DBGMSG(Box<str>), //print debug message
    DUMP,   //print stack top
    ARGC,   //command line arguments: get length
    ARGV,   //command line arguments: get element by index
    READ,   //read file to string
    EMPTY,  //does nothing
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
//////////////////////////////////////////////////////////////////////
fn parse(pr: &Vec<Tok>, filename: &String) -> Option<(String, Vec<(Result<Op, String>, Loc)>, Vec<(String, Option<i64>)>, Option<usize>)> {
    if false {
        eprintln!("[parsing loc={:?} val={:?}]", pr.iter().map(|x| vec![x.0.0, x.0.1]), pr.iter().map(|x| x.1.clone()));
    } else {
        eprintln!("[parsing...]");
    }
    let mut res: Vec<(Result<Op, String>, Loc)> = vec![];
    #[derive(Debug)]
    enum State {
        NONE,   //no special commands
        LBL,    //label without definition (maybe useless)
        FN,     //label with definition
        DBGMSG, //print debug message
        FNADDR, //function address
    }
    let mut state: State = State::NONE;
    let mut labels: Vec<(String, Option<i64>)> = Vec::new();
    let mut main: Option<usize> = None;
    //multi-line comment
    let mut mlc: u32 = 0;
    let mut callstk: Vec<usize> = Vec::new();
    let mut stksim: Vec<usize> = Vec::new();
    let mut callmode: Callmode = CALLMODE_DEFAULT;
    let mut ind: isize = -1;
    while {ind+=1;ind} < pr.len()as isize{
        let i: &Tok = &pr[ind as usize];
        let val: &String = &i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let index: &i64 = &loc.1;
        let parseerr = |msg: &str| {
            eprintln!("{}:{}:{}: Error: {}", filename, lin, index, msg);
            return None;
        };
        let parsewarn = |msg: &str| {
            eprintln!("{}:{}:{}: Warning: {}", filename, lin, index, msg);
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
        if PARSE_DEBUG {
            eprintln!("parse: callstk={:?} val={} callmode={:?}",
                      callstk, repr(val.as_str()), callmode);
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
                    eprintln!("custom string postfixes are not implemented yet: {}", tmpStr.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>());
                    return None;
                },
            }
            tmpres
        } else {match strtoi64(val) {
            Some(x) => {
                res.append(&mut vec![
                    (Ok(Op::Push(x)), *loc),
                ]);
                stksim.push(res.len());
                continue;
            },
            None => match state {
                    State::NONE => {
use crate::Op::*;
                let matchresult: Op = match val.as_str() {
                    ""|"\n" => continue,
                    "+" => PLUS,
                    "*" => MUL,
                    "putc" => PRINT,
                    "puts" => PUTS,
                    "flush" => FLUSH,
                    "input" => INP,
                    "lbl" => {
                        state = State::LBL;
                        continue;
                    },
                    "fn" => {
                        state = State::FN;
                        continue;
                    },
                    "if" => GIF,
                    ":" => {
                        state = State::FNADDR;
                        continue;
                    },
                    "pushnth" => PUSHNTH,
                    "dropnth" => DROPNTH,
                    "nbrot" => NBROT,
                    "<" => LT,
                    "=" => EQ,
                    "!" => NOT,
                    "|" => OR,
                    "exit" => EXIT,
                    "??#" => PSTK,
                    "???" => PSTKE,
                    "dbgmsg" => {
                        state = State::DBGMSG;
                        continue;
                    },
                    "addr" => Push(res.len()as i64),
                    "paddr" => {
                        println!("paddr: {}", res.len());
                        continue;
                    },
                    "paddre" => {
                        println!("paddre: {}", res.len());
                        ind = res.len()as isize;
                        continue;
                    },
                    "dump" => DUMP,
                    "(" => {
                        callstk.push(res.len());
                        continue;
                    },
                    ")" => {
use crate::Callmode::*;
                        let insertion_index: usize = match callstk.pop() {
                            Some(x) => x,
                            None => return parseerr("call underflow!"),
                        };
                        if PARSE_DEBUG_CALL {
                            eprintln!("insertion_index={} res={:?}", insertion_index, res);
                        }
                        //remove address
                        let element = res.remove(insertion_index-1);
                        //push it to the top
                        res.push(element);

                        match callmode {
                            WITHOUT_ADDRESS => {
                                
                            },
                            WITH_ADDRESS_LEFT => {
                                res.insert(insertion_index, (Ok(Push(res.len()as i64)), *loc));
                                callmode = WITHOUT_ADDRESS;
                            },
                            WITH_ADDRESS_RIGHT => {
                                res.push((Ok(Push(res.len()as i64)), *loc));
                                callmode = WITHOUT_ADDRESS;
                            },
                        }

                        //push condition (true)
                        res.push((Ok(Push(1)), *loc));
                        res.push((Ok(G), *loc));
                        continue;
                    },
                    "#" => {
                        callmode = CALLMODE_ON_OPERATOR;
                        continue;
                    },
                    "argc" => ARGC,
                    "argv" => ARGV,
                    "read" => READ,
                    "empty_op" => EMPTY,
                    _ => {
                        res.append(&mut vec![
                            (Err(val.to_string()), *loc),
                            (Ok(Op::G), *loc),
                        ]);
                        continue;
                    },
                };
                res.append(&mut vec![
                    (Ok(matchresult), *loc),
                ]);
                continue;
                    },
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
                    State::FNADDR => {
                        res.append(&mut vec![
                            (Err(val.to_string()), *loc),
                        ]);
                        state = State::NONE;
                        continue;
                    },
                    State::DBGMSG => {
                        res.push((Ok(Op::DBGMSG(val.as_str().into())), *loc));
                        state = State::NONE;
                        continue;
                    },
                    _ => {
                        eprintln!("Unknown state of parser (debug): \"{:?}\"", state);
                        return None;
                    },
                }
            }
        }
    );}
    let parseerr = |msg: &str| {
        eprintln!("{}:EOF: Error: {}", filename, msg);
        return None::<(String, Vec<(Result<Op, String>, Loc)>, Vec<(String, Option<i64>)>, Option<usize>)>;
    };
    let parsewarn = |msg: &str| {
        eprintln!("{}:EOF: Warning: {}", filename, msg);
    };
    if !matches!(state, State::NONE) {
        return parseerr("Parsing is ended but state is not none");
    }
    if callstk.len() > 0 {
        return parseerr("Callstk is not empty");
    }
    //to avoid not founding labels
    labels.push(("".to_string(), None));

    return Some((filename.to_string(), res, labels, main));
}
fn link(filename: &String, res: &Vec<(Result<Op, String>, Loc)>, labels: &Vec<(String, Option<i64>)>, main: &Option<usize>) -> Option<Vec<(Op, Loc)>> {
    eprintln!("[linking...]");
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
                                eprintln!("{}:{}:{}: label is declared, but has no definition", filename, lin, index);
                                return None;
                            }
                        }
                    } else {
                        ret += 1;
                    }
                }
                if ret >= labels.len()as i64 - 1 {
                    eprintln!("{}:{}:{}: label not found: {}", filename, lin, index, repr(x));
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

#[derive(Debug)]
enum simResult {
    ok(i32),
    err,
    errs(String),
    stopped,
}
fn sim(pr: &mut Vec<(Op, Loc)>,
       filename: &String,
       argv: Vec<String>,
       output_to_file: Option<String>) -> simResult {
use simResult::*;
use std::fs::{File, OpenOptions};
use crate::Op::*;
    if !LINK_DEBUG {
        eprintln!("[simulation...]");
    } else {
        eprintln!("[simulation:");
        let mut ind: usize = 0;
        for i in &mut *pr {
            ind += 1;
            eprintln!("  {}  {}:{}:{:?}",
                      ind, i.1.0, i.1.1, i.0);
        }
        eprintln!("]");
    }
    if ONLY_LINK {
        return stopped;
    }
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
    let mut f: Option<File> = match output_to_file {
        Some(ref x) => {
            //clear file
            {
                let mut clear_f = match OpenOptions::new().write(true).append(false).create(true).open(x) {
                    Ok(y) => y,
                    Err(e) => {
                        eprintln!("cannot open file \"{}\" to write in: {}", repr(x), e);
                        return errs("E0".to_string());
                    },
                };
                _ = File::create(x);
                _ = clear_f.write(b"");
            }
            //open file to append mode
            Some(match OpenOptions::new().append(true).write(true).open(x) {
                Ok(y) => y,
                Err(e) => {
                    eprintln!("cannot open file \"{}\" to append in: {}", repr(x), e);
                    return errs("E0".to_string());
                },
            })
        },
        None => {
            None
        },
    };
    let mut ind: i64 = main - 1;
    while ind != pr.len()as i64 {
        ind += 1;
        let i: &Op = &pr[{let tmp: usize = ind as usize; if tmp >= pr.len() {break;} else {tmp}}].0;
        let loc: &Loc = &pr[ind as usize].1;
        let lin: i64 = loc.0;
        let index: i64 = loc.1;
        if SIM_DEBUG {
            eprintln!("---- {},{}:{:?} ----\n{:?}", lin, index, i, stack);
        }
        match i {
            Push(x) => {
                stack.push(*x);
            },
            PRINT => {
                match output_to_file {
                    Some(_) => {
                        _ = f.as_ref().unwrap().write(&[stack.pop().unwrap()as u8]);
                    },
                    None => {
                        print!("{}", char::from_u32(stack.pop().unwrap()as u32).unwrap());
                    },
                }
            },
            PUTS => {
                if SIM_DEBUG_PUTS && !SIM_DEBUG{
                    eprintln!("debug: puts: {:?}", stack);
                }
                let strlen: usize = stack.pop().unwrap()as usize;
                let mut i: usize = 0;
                let mut string: String = "".to_owned();
                while i < strlen {
                    let chr = char::from_u32(stack.pop().unwrap()as u32).unwrap();
                    string.push(chr);
                    i += 1;
                }
                match output_to_file {
                    Some(_) => {
                        f.as_ref().unwrap().write(string.as_bytes());
                    },
                    None => {
                        print!("{}", string);
                    },
                }
            },
            FLUSH => {
                _ = std::io::stdout().flush();
            },
            INP => {
                let mut input: String = String::new();
                let stdin = std::io::stdin();
                _ = stdin.read_line(&mut input);
                stack.append(&mut from(&input).iter().rev().map(|x| *x).collect::<Vec<i64>>());
                stack.push(input.len()as i64);
            },
            PLUS => {
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
            MUL => {
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
            GIF => {
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
            G => {
                let addr: i64 = match stack.pop() {
                    Some(x) => x-1,
                    None => {
                        return errs("Operand `addr` for G intrinsic not found".to_string());
                    },
                };
                ind = addr.try_into().unwrap();
            },
            PUSHNTH => {
                let a: i64 = stack.pop().unwrap();
                if a >= stack.len()as i64 {
                    return errs("pushnth overflow".to_owned());
                }
                let b: i64 = stack[stack.len()-1-a as usize];
                stack.push(b);
            },
            DROPNTH => {
                let a: i64 = stack.pop().unwrap();
                stack.remove(stack.len()-1-a as usize);
            },
            NBROT => {
                let l: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.insert(stack.len()-0-l as usize, a);
            },
            LT => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push((b < a).try_into().unwrap());
            },
            EQ => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push((b == a).try_into().unwrap());
            },
            NOT => {
                let a: i64 = stack.pop().unwrap();
                stack.push((a == 0).try_into().unwrap());
            },
            OR => {
                let a: i64 = stack.pop().unwrap();
                let b: i64 = stack.pop().unwrap();
                stack.push(((a != 0) || (b != 0)).try_into().unwrap());
            },
            EXIT => {
                println!();
                let a: i64 = stack.pop().unwrap();
                return ok(a.try_into().unwrap());
            },
            PSTK => {
                if !SIM_DEBUG {
                    println!("{}:{}: pstk  {:?}", lin, index, stack);
                }
            },
            PSTKE => {
                if !SIM_DEBUG {
                    println!("{}:{}: pstke {:?}", lin, index, stack);
                }
                return err;
            },
            DUMP => {
                println!("dump: {}", match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand for DUMP intrinsic not found".to_owned());
                    },
                });
            },
            ARGC => {
                stack.push(argv.len().try_into().unwrap());
            },
            ARGV => {
                let a: i64 = stack.pop().unwrap();
                if a >= argv.len()as i64 {
                    return errs("Argv overflow".to_owned());
                }
                if a < 0 {
                    return errs("Argv underflow".to_owned());
                }
                for j in argv[a as usize].chars().rev() {
                    stack.push(j as i64);
                }
                stack.push(argv[a as usize].len().try_into().unwrap());
            },
            READ => {
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
            DBGMSG(x) => {
                println!("dbgmsg: {}", repr(x));
            },
            EMPTY => {

            },
            _ => {
                return errs(strcat("unknown op: ", &i.to_string()));
            },
        }
    }
    println!();
    return ok(0);
}

fn clah(args: &Vec<String>) {
    match cla(args) {
        Ok(mode) => {
            eprintln!("[command line arguments reading succed]");
            match mode {
                Mode::SIM => {
                    for_each_arg(&args, |i: &String,
                                        ind: isize,
                                        argv: &Vec<String>,
                                        fargs: &Vec<String>,
                                        args: &Vec<String>,
                                        output_to_file: Option<String>| {
use simResult::*;
                        let error: simResult = sim(&mut match linkparselexget(&i) {
                            Some(x) => x,
                            None => return,
                        }, &i, if ind==(argv.len()-1).try_into().unwrap() {
                            fargs.clone()
                        } else {
                            vec![
                                args[0].clone(),
                            ]
                        }, output_to_file);
                        match error {
                            ok(x) => {
                                if x == 0 {
                                    eprintln!("[Simulation of {} succed]", repr(&i));
                                } else {
                                    eprintln!("[Simulation of {} was finished with exit code {}]", repr(&i), x);
                                }
                            },
                            err => {
                                eprintln!("[Simulation of {} failed]", repr(&i));
                            },
                            errs(x) => {
                                eprintln!("[Simulation of {} failed due to this error: {}]", repr(&i), repr(&x));
                            },
                            stopped => {
                                eprintln!("[Simulation of {} stopped]", repr(&i));
                            },
                            _ => {
                                eprintln!("[Simulation of {}: Internal error: Unknown  state: {:?}]", repr(&i), err);
                            },
                        }
                    });
                },
                Mode::DUMP => {
                    for_each_arg(&args, |i: &String,
                                        ind: isize,
                                        argv: &Vec<String>,
                                        fargs: &Vec<String>,
                                        args: &Vec<String>,
                                        output_to_file: Option<String>| {
                        let tokens: Vec<(Op, Loc)> = match linkparselexget(&i) {
                            Some(x) => x,
                            None => return,
                        };
                        match output_to_file {
                            Some(ref x) => {
use std::fs::{File, OpenOptions};
                                match File::create(x) {
                                    Ok(_)|Err(_) => {},
                                };
                                {
                                    let mut f = match OpenOptions::new().write(true).open(x) {
                                        Ok(y) => y,
                                        Err(e) => {
                                            todo!();
                                        },
                                    };
                                    _ = f.write(b"");
                                }
    	                        let mut f = match OpenOptions::new().append(true).open(x) {
                                    Ok(y) => y,
                                    Err(e) => {
                                        todo!();
                                    },
                                };
                                for i in &tokens {
                                    f.write(i.1.0.to_string().as_bytes()).unwrap();
                                    f.write(b":").unwrap();
                                    f.write(i.1.1.to_string().as_bytes()).unwrap();
                                    f.write(b":").unwrap();
                                    f.write(i.0.to_string().as_bytes()).unwrap();
                                    f.write(b"\n").unwrap();
                                }
                            },
                            None => {
                                for i in &tokens {
                                    println!("{}:{}:{:?}", i.1.0, i.1.1, i.0);
                                }
                            },
                        }
                    });
                },
                Mode::NONE => {
                    return;
                },
                _ => {
                    eprintln!("Unknown mode: \"{:?}\"", mode);
                },
            }
        },
        Err(x) => {
            eprint!("[command line arguments reading failed due to {} previous error", x);
            if x >= 2 {
                eprint!("s");
            }
            eprintln!("]");
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
