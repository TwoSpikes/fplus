#[allow(non_camel_case_types)]

use std:: {
    io::Write,
    fmt,
};

static mut CARGO_VERSION: &str = "1.68.2";

// -- LEXER --
//Show resulting token array
static mut LEX_DEBUG: bool = false;
//Stop on lexing, do not parse
static mut ONLY_LEX: bool = false;

// -- LINKING --
//Show resulting program
static mut LINK_DEBUG: bool = false;
//Stop on linking, do not run
//(e.g. when infinite loop)
static mut ONLY_LINK: bool = false;
//print message "[linking succed]"
static mut LINK_DEBUG_SUCCED: bool = true;

// -- PARSING --
//show every token and some variables for parsing
static mut PARSE_DEBUG: bool = false;
//show debug state
static mut PARSE_DEBUG_STATE: bool = false;
//show scope_id
static mut PARSE_DEBUG_ID: bool = false;
//show callstack
static mut PARSE_DEBUG_CALL: bool = false;
//show debug information about strings
static mut PARSE_DEBUG_STRING: bool = false;
//show message about including each file
static mut PARSE_DEBUG_INCLUDE: bool = true;
//show message how many fns is being included in the specific including operation
static mut PARSE_DEBUG_INCLUDE_ADDING: bool = false;
//show message about every succed include operation
static mut PARSE_DEBUG_INCLUDE_SUCCED: bool = true;
//show message "[Parsing succed]"
static mut PARSE_DEBUG_SUCCED: bool = true;
//callmode without # operator
static mut CALLMODE_DEFAULT: Callmode = Callmode::WITH_ADDRESS_LEFT;
//callmode with # operator
static mut CALLMODE_ON_OPERATOR: Callmode = Callmode::WITHOUT_ADDRESS;
//access modifier without any operators ("pub" and "pri")
static mut CURMOD_DEFAULT: Mod = Mod::PRI;
//enable colors
static mut ENABLE_COLORS: bool = false;

// -- SIMULATION --
//disable simulation for smaller executable file (saves ~33K)
const SIM_ENABLE: bool = true;
//show every token on runtime and stack state
static mut SIM_DEBUG: bool = false;
//show stack state in puts command
static mut SIM_DEBUG_PUTS: bool = false;

// -- MAX LEVELS --
//maximum level of include recursion
static mut MAX_INCLUDE_LEVEL: usize = 500;

// -- COLOR CONSTANTS --
const RESET_COLOR: &str = "\x1b[0m";
const RED_COLOR: &str = "\x1b[91m";
const GREEN_COLOR: &str = "\x1b[92m";
const YELLOW_COLOR: &str = "\x1b[93m";

//get string from token
fn getstrfromtok(x: &String) -> Box<String> {
    return Box::new(if x.chars().nth(0) == Some('\"') {
        urepr(&x[1..x.len()-1])
    } else {
        x.to_string()
    });
}

#[derive(Clone)]
struct Formatstr {
    string: String,
    formatnums: Vec<usize>,
    formatters: Vec<String>,
    position: Vec<usize>,
}
#[derive(Debug)]
struct ComputeFormatNumbers {
    numbers: Vec<usize>,
    position: Vec<usize>,
    string: String,
}
impl Formatstr {
    fn from(x: &str) -> Option<Self> {
        let temp = Self::compute_format_numbers(&String::from(x))?;
        return Some(Self {
            string: temp.string,
            formatnums: temp.numbers,
            formatters: Vec::new(),
            position: temp.position,
        });
    }
    fn compute_format_numbers(string: &String) -> Option<ComputeFormatNumbers> {
        let mut result: ComputeFormatNumbers = ComputeFormatNumbers {
            numbers: Vec::new(),
            string: String::new(),
            position: Vec::new(),
        };
        let mut curly_bracket: bool = false;
        let mut temp_position: Option<usize> = None;
        let mut temp_num: usize = 0;
        let mut ind: usize = 0;
        for i in string.chars() {
            match i {
                '{' => {
                    if !curly_bracket {
                        curly_bracket = true;
                        temp_position = Some(ind);
                    }
                    result.string.push(i);
                    ind += 1;
                },
                '}' => {
                    match curly_bracket {
                        false => {
                            result.string.push(i);
                            ind += 1;
                        },
                        true => {
                            curly_bracket = false;
                            result.position.push(temp_position.unwrap());
                            result.numbers.push(temp_num);
                            temp_num = 0;
                            temp_position = None;
                            result.string.pop();
                        },
                    }
                },
                _ => {
                    match curly_bracket {
                        false => {
                            result.string.push(i);
                            ind += 1;
                        },
                        true => {
                            match i {
                                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                                    temp_num = temp_num*10 + strtoi64_signed(&String::from(i)).unwrap() as usize;
                                },
                                _ => {
                                    result.string.push(i);
                                    ind += 1;
                                    curly_bracket = false;
                                    temp_position = None;
                                    temp_num = 0;
                                },
                            }
                        },
                    }
                },
            }
        }
        return Some(result);
    }
    fn format(&mut self, x: &str) -> Option<Self> {
        self.formatters.push(String::from(x));
        if self.formatters.len() > match self.formatnums.iter().max() {
            Some(x) => *x,
            None => 0,
        }+1 {
            return None;
        }
        return Some(self.clone());
    }
    fn to_string(&self) -> String {
        let mut result: String = self.string.clone();
        let mut ind: usize = 0;
        let mut already_inserted: usize = 0;
        while ind < self.formatnums.len() {
            let string: &String = &self.formatters[self.formatnums[ind]];
            let mut ind2: usize = 0;
            while ind2 < string.len() {
                result.insert(self.position[ind]
                               +ind2
                               +already_inserted
                               -ind,
                              string.chars().nth(ind2).unwrap());
                ind2 += 1;
            }
            already_inserted += ind2;
            ind += 1;
        }
        return result;
    }
}

fn hi(x: u128) -> i64 {
    (((x >> 64) as i128)-9223372036854775807)as i64
}

fn lo(x: u128) -> i64 {
    ((x as i128)-9223372036854775807) as i64
}

fn covariant_right<T: std::cmp::PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() > b.len() {
        return false;
    }
    let mut ind: usize = 0;
    while ind < b.len()-1 {
        if a[ind] != b[ind] {
            return false;
        }
        ind += 1;
    }
    return true;
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum Callmode {
    WITHOUT_ADDRESS,    //like goto operator in C
                        //or jmp operator in asm
    WITH_ADDRESS_LEFT,  //save address in the top of the stack
                        //to jump there in the end of function
    #[allow(dead_code)]
    WITH_ADDRESS_RIGHT, //save address before arguments
}

fn parselexget(filename: &String,
               include_level: usize,
               scope_id: Vec<usize>) ->
                   Option<(Vec<(String,
                                Option<i64>,
                                Vec<usize>)>,
                            Vec<(Op, Loc)>,
                            Vec<usize>,
                            Box<Vec<i64>>)> {
    match parse(&mut {
use crate::Retlex::*;
        #[allow(unreachable_patterns)]
        match lex(&filename, &match get(&filename) {
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
            N(x) => {
                x
            },
            STOPPED => {
                eprintln!("[lexing stopped]");
                return None;
            },
            _ => {
                eprintln!("Unknown lexing return state");
                return None;
            },
    }}, &filename, include_level, scope_id) {
        Some(x) => {
            if unsafe { PARSE_DEBUG_SUCCED } {
                eprintln!("[Parsing succed]");
            }
            return Some((x.0, x.1, x.2, x.3));
        },
        None => {
            eprintln!("[Parsing failed]");
            return None;
        },
    }
}

fn matchlink<'a>(filename: &String,
             res: &Vec<(Result<Op, (String, Vec<usize>)>, Loc)>,
             labels: &Vec<(String, Option<i64>, Vec<usize>)>,
             main: &Option<usize>,
             data: &'a mut Vec<i64>,
             include_level: usize) -> Option<(Vec<(Op, Loc)>, &'a mut Vec<i64>)> {
    match link(&filename, &res, &labels, &main, data, include_level) {
        Some(x) => {
            if unsafe {LINK_DEBUG_SUCCED} {
                eprintln!("[linking succed]");
            }
            Some((x.0, data))
        },
        None => {
            eprintln!("[linking failed]");
            None
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
    #[allow(non_camel_case_types)]
    enum Argsstate {
        NONE,
        MAX_INCLUDE_LEVEL,
    }
    let mut state: Argsstate = Argsstate::NONE;
    let mut argv: Vec<String> = Vec::new();
    //fucking arguments
    let mut fargs: Vec<String> = Vec::new();
    let mut output_to_file: Option<Option<String>> = None;
    {
        let mut ind: isize = 1;
        let mut isargs: bool = false;
        while {ind+=1;ind} < args.len() as isize {
            let i = args[ind as usize].clone();

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
            match state {
                Argsstate::NONE => {
                    match i.to_lowercase().as_str() {
                        "-o"|"--output"|"-output" => {
                            output_to_file = Some(None);
                            continue;
                        },
                        "--lex-debug"|"-lex-debug" => {
                            unsafe {
                                LEX_DEBUG = !LEX_DEBUG;
                            }
                            continue;
                        },
                        "--only-lex"|"-only-lex" => {
                            unsafe {
                                ONLY_LEX = !ONLY_LEX;
                            }
                            continue;
                        },
                        "--link-debug"|"-link-debug" => {
                            unsafe {
                                LINK_DEBUG = !LINK_DEBUG;
                            }
                            continue;
                        },
                        "--only-link"|"-only-link" => {
                            unsafe {
                                ONLY_LINK = !ONLY_LINK;
                            }
                            continue;
                        },
                        "--link-debug-succed"|"-link-debug-succed" => {
                            unsafe {
                                LINK_DEBUG_SUCCED = !LINK_DEBUG_SUCCED;
                            }
                            continue;
                        },
                        "--parse-debug"|"-parse-debug" => {
                            unsafe {
                                PARSE_DEBUG = !PARSE_DEBUG;
                            }
                            continue;
                        },
                        "--parse-debug-state"|"-parse-debug-state" => {
                            unsafe {
                                PARSE_DEBUG_STATE = !PARSE_DEBUG_STATE;
                            }
                            continue;
                        },
                        "--parse-debug-id"|"-parse-debug-id" => {
                            unsafe {
                                PARSE_DEBUG_ID = !PARSE_DEBUG_ID;
                            }
                            continue;
                        },
                        "--parse-debug-call"|"-parse-debug-call" => {
                            unsafe {
                                PARSE_DEBUG_CALL = !PARSE_DEBUG_CALL;
                            }
                            continue;
                        },
                        "--parse-debug-string"|"-parse-debug-string" => {
                            unsafe {
                                PARSE_DEBUG_STRING = !PARSE_DEBUG_STRING;
                            }
                            continue;
                        },
                        "--parse-debug-include"|"-parse-debug-include" => {
                            unsafe {
                                PARSE_DEBUG_INCLUDE = !PARSE_DEBUG_INCLUDE;
                            }
                            continue;
                        },
                        "--parse-debug-include-adding"|"-parse-debug-include-adding" => {
                            unsafe {
                                PARSE_DEBUG_INCLUDE_ADDING = !PARSE_DEBUG_INCLUDE_ADDING;
                            }
                            continue;
                        },
                        "--parse-debug-include-succed"|"-parse-debug-include-succed" => {
                            unsafe {
                                PARSE_DEBUG_INCLUDE_SUCCED = !PARSE_DEBUG_INCLUDE_SUCCED;
                            }
                            continue;
                        },
                        "--parse-debug-succed"|"-parse-debug-succed" => {
                            unsafe {
                                PARSE_DEBUG_SUCCED = !PARSE_DEBUG_SUCCED;
                            }
                            continue;
                        },
                        "--sim-debug"|"-sim-debug" => {
                            unsafe {
                                SIM_DEBUG = !SIM_DEBUG;
                            }
                            continue;
                        },
                        "--sim-debug-puts"|"-sim-debug-puts" => {
                            unsafe {
                                SIM_DEBUG_PUTS = !SIM_DEBUG_PUTS;
                            }
                            continue;
                        },
                        "--max-include-level"|"-max-include-level" => {
                            state = Argsstate::MAX_INCLUDE_LEVEL;
                            continue;
                        },
                        "--enable-colors"|"-enable-colors" => {
                            unsafe {
                                ENABLE_COLORS = !ENABLE_COLORS;
                            }
                            continue;
                        },
                        &_ => {},
                    }
                },
                Argsstate::MAX_INCLUDE_LEVEL => {
                    unsafe {
                        MAX_INCLUDE_LEVEL = strtoi64(&i).unwrap() as usize;
                    }
                    state = Argsstate::NONE;
                    continue;
                },
            }
            argv.push(i);
        }
    }
    let mut ind: isize = -1;
    while {ind+=1;ind}<argv.len() as isize {
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

#[allow(dead_code)]
fn strcat(a: &str, b: &str) -> String {
    let mut res: String = String::new();
    for i in a.chars() {
        res.push(i);
    }
    for i in b.chars() {
        res.push(i);
    }
    return res;
}

fn repr(string: &str) -> String {
    let mut res: String = String::new();
    for i in string.chars() {
        res += &match i {
            '\r' => vec!['\\', 'r'],
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
    let mut res: String = String::new();
    let mut ind: isize = -1;
    while {ind+=1;ind} < string.len() as isize {
        let i: char = match string.chars().nth(ind as usize) {
            Some(x) => x,
            None => break,
        };
        res += &match i {
            '\\' => {
                res += &vec![match match string.chars().nth((ind+1) as usize) {
                    Some(x) => x,
                    _ => panic!("Escape character not found"),
                }{
                'r' => '\r',
                'n' => '\n',
                't' => '\t',
                '\\' => '\\',
                '\'' => '\'',
                '\"' => '\"',
                _ => {
                    panic!("Unknown escaping character: \'{}\'", vec![i, string.chars().nth((ind+1) as usize).unwrap()].iter().collect::<String>());
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
    println!("{}", Formatstr::from("Usage:
$ fplus SUBCOMMAND [OPTION]... [SOURCE]... -- [ARG]...

SUBCOMMAND (insensitive to register):
{sim s}                 Simulate program
{version ver v}         Print version information and exit
{usage use u help h ?} info information
		                Print help information and exit
{dump d}                Dump the tokens of the program.
{error e}               Print error code and information about them

OPTION (insensitive to register):
{-o --output -output} FILE      dump output to FILE
{--lex-debug -lex-debug}        show debug information during lexing
{--only-lex -only-lex}          stop on lexing (for debugging purposes)
{--link-debug -link-debug}      show debug information during linking
{--only-link -only-link}        stop on linking (for debugging purposes)
{--link-debug-succed -link-d...}show [linking succed]
{--parse-debug -parse-debug}    show debug information during parsing
{--parse-debug-state --parse...}show State information during parsing
{--parse-debug-id --parse-de...}show scope-id information during parsing
{--parse-debug-call -parse-d...}show function calling information during parsing
{--parse-debug-string -parse...}show string information during parsing
{--parse-debug-include -pars...}show including information
{--parse-debug-include-adding -parse-debug-include-adding}
                                TODO
{--parse-debug-include-succed -parse-debug-include-succed}
                                TODO
{--sim-debug -sim-debug}        show debug information during simulation
{--sim-debug-puts -sim-debuf...}show debug information debore printing
{--max-include-level -max-include-level} NUMBER
                                set max include level (now is {0})
{--enable-colors -enable-colors}enable terminal colors").unwrap()
.format(&unsafe { MAX_INCLUDE_LEVEL }.to_string()).unwrap()
.to_string());
}
fn version() {
    println!("F+, a stack-based interpreting programming language
written on Rust v.{}
version: 0.1.0-5
download: https://github.com/TwoSpikes/fplus
2022-2023 @ TwoSpikes", unsafe { CARGO_VERSION });
}
fn errorcodes() {
    println!("errorcodes:
E0                    Cannot open file");
}

fn compile_insructions() {
    println!("\nDownload source code from https://github.com/TwoSpikes/fplus/#/main.rs and recompile it using Cargo v.{}", unsafe { CARGO_VERSION });
}

#[derive(Debug)]
enum Mode {
    NONE,
    SIM,
    DUMP,
}
fn cla(args: &Vec<String>) -> Result<Mode, i32> {
    let mut err: i32 = 0;
    if args.len() <= 1 {
        eprintln!("No subcommand provided");
        usage();
        return Err({err += 1; err});
    }
    match args[1].to_lowercase().as_str() {
        "sim"|"s" => {
            if SIM_ENABLE {
                if args.len() <= 2 {
                    eprintln!("No source file provided");
                    usage();
                    return Err({err+=1; err});
                }
                return Ok(Mode::SIM);
            } else {
                eprintln!("Simulation is disabled.");
                compile_insructions();
                Ok(Mode::NONE)
            }
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
            eprintln!("Cannot read file {}", repr(name));
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
    STOPPED,
}
#[derive(Debug)]
enum Quotes {
    NO,
    IN,
    POSTF,
}
/*
 * Warning!: Legacy code warning.
 */
fn lex(filename: &String, file: &String) -> Retlex {
use crate::Retlex::*;
use crate::Quotes::*;
    if file.len() == 0 {
        return EMPTY;
    }
    let mut res: Vec<Tok> = vec![];
    let mut tmp: String = String::new();
    let mut ploc: Loc = Loc(1, 1);
    let mut loc:  Loc = Loc(1, 1);
    let mut quotes: Quotes = Quotes::NO;
    for i in file.chars() {
        loc.1 += 1;
        //" then remember it
        if i == '"' {
            tmp.push(i);
            #[allow(unreachable_patterns)]
            match quotes {
                Quotes::NO => {
                    quotes = Quotes::IN;
                },
                Quotes::IN => {
                    quotes = Quotes::POSTF;
                },
                Quotes::POSTF => {
                    res.push(Tok(ploc, tmp.to_owned()));
                    tmp = String::new();
                    ploc = loc.clone();
                    quotes = Quotes::NO;
                },
                _ => {
                    eprintln!("lex: unknown quotes: {:?}", quotes);
                    return E;
                },
            };
            continue;
        }
        #[allow(unreachable_patterns)]
        match quotes {
            NO => {},
            IN => {
                tmp.push(i);
                continue;
            },
            POSTF => {
                if i == '\n' || i == ' ' {
                    quotes = NO;
                    res.push(Tok(ploc, tmp.to_owned()));
                    tmp = String::new();
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
        if i == '\n' {
            loc.1 = 1;
            loc.0 += 1;
        }
        //push special symbols as special symbols
        if i == '\n' || i == ':' || i == '(' || i == ')'{
            res.push(Tok(ploc, tmp.to_owned()));
            res.push(Tok(loc, String::from(i)));
            tmp = String::new();
            ploc = loc.clone();
            continue;
        }
        //' ' or '\t' then push tmp
        if i == ' ' || i == '\t' {
            if tmp.len() > 0 {
                res.push(Tok(ploc, tmp.to_owned()));
                tmp = String::new();
            }
            ploc = loc.clone();
            continue;
        }
        tmp.push(i);
    }
    if tmp.len() > 0 {
        res.push(Tok(ploc, tmp.to_owned()));
    }

    if unsafe { LEX_DEBUG } {
        eprintln!("{}: Lexing result: [", filename);
        for i in &res {
            eprintln!("  {}:{}: {:?}", i.0.0, i.0.1, i.1);
        }
        eprintln!("]");
    }
    if unsafe { ONLY_LEX } {
        return STOPPED;
    }

    return N(res);
}

fn strtoi64(x: &String) -> Option<i64> {
    return if x.len() > 0 && x.chars().nth(x.len()-1).unwrap() == 'u' {
        strtoi64_unsigned(&x[..x.len()-1].to_string())
    } else {
        strtoi64_signed(&x)
    };
}
fn strtoi64_signed(x: &String) -> Option<i64> {
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
            _ => return None,
        };
        if i == '-' {
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
fn strtoi64_unsigned(x: &String) -> Option<i64> {
    let mut res: u64 = 0;
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
            _ => return None,
        };
    }
    return Some(res as i64 - 9223372036854775807);
}

#[derive(Debug, Clone)]
enum Op {
    Push(i64), //push number to the stack
    PRINT,  //print char (same as number)
    EPRINT, //print char to stderr (see ::PRINT)
    PUTS,   //print string
    EPUTS,  //print string to stderr (see ::PUTS)
    PUTSLN, //print string with a new line (see ::PUTS)
    EPUTSLN,//print string with a new line to stderr (see ::PUTSLN)
    FLUSH,  //print stdout buffer and clear it
    EFLUSH, //print stderr buffer and clear it (see ::EFLUSH)
    INP,    //read line from stdin
    PLUS,   // +
    INVERT, // x - 2x
    MUL,    // *
    DIV,    // 1
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
    DBGMSG(Box<String>), //print debug message
    DUMP,   //print stack top
    ARGC,   //command line arguments: get length
    ARGV,   //command line arguments: get element by index
    READ,   //read file to string
    GETTIME,//returns u128 with number of nanoseconds
    DEREF,  //dereference the pointer
    EMPTY,  //does nothing
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
//access modifier for macros and functions
#[derive(Clone, Copy, Debug)]
enum Mod {
    UNK, //unknown
    PRI, //only in this file
    PUB, //anywhere
}
macro_rules! parsemsg_loop {
    () => {
        eprintln!();
    };
    ($head:expr, $($tail:expr,)*) => {
        eprint!("{}", $head);
        parsemsg_loop!($($tail,)*);
    };
}
macro_rules! parsemsg {
    ($msg:expr, $lin:expr, $index:expr, $filename:expr, $($tail:expr),*) => {
        eprint!("{}:{}:{}: {}: ", $filename, $lin, $index, $msg);
        parsemsg_loop!($($tail,)*);
    };
}
macro_rules! parseerrmsg {
    ($lin:expr, $index:expr, $filename:expr, $($tail:expr),*) => {
        parsemsg!("{}Error{}",
                  RED_COLOR,
                  $lin,
                  $index,
                  $filename,
                  $($tail),*,
                  RESET_COLOR);
    };
}
macro_rules! parsewarnmsg {
    ($lin:expr, $index:expr, $filename:expr, $($tail:expr),*) => {
        parsemsg!("{}Warning{}",
                  YELLOW_COLOR,
                  $lin,
                  $index,
                  $filename,
                  $($tail),*,
                  RESET_COLOR);
    };
}
//////////////////////////////////////////////////////////////////////
fn parse(pr: &mut Vec<Tok>,
         filename: &String,
         include_level: usize,
         mut scope_id: Vec<usize>) -> Option<(
             Vec<(String,
                  Option<i64>,
                  Vec<usize>)>,
             Vec<(Op, Loc)>,
             Vec<usize>,
             Box<Vec<i64>>,
         )> {
    let mut data: Vec<i64> = Vec::new();
use crate::Op::*;
    if include_level > unsafe { MAX_INCLUDE_LEVEL } {
        eprintln!("exceeded max include level: {}", unsafe { MAX_INCLUDE_LEVEL });
    }
    if false {
        eprintln!("[parsing loc={:?} val={:?}]", pr.iter().map(|x| vec![x.0.0, x.0.1]), pr.iter().map(|x| x.1.clone()));
    } else {
        eprintln!("[parsing...]");
    }
    let mut result: Vec<(Op, Loc)> = Vec::new();
    let mut res: Vec<(Result<Op, (String, Vec<usize>)>, Loc)> = Vec::new();
    #[derive(Debug)]
    enum State {
        NONE,   //no special commands
        LBL,    //label without definition (maybe useless)
        FN,     //label with definition
        DBGMSG, //print debug message
        FNADDR, //function address
        INCLUDE,//recursively include file
    }
    let mut state: State = State::NONE;
    let mut labels: Vec<(String, Option<i64>, Vec<usize>)> = Vec::new();
    let mut main: Option<usize> = None;
    //multi-line comment
    let mut mlc: u32 = 0;
    let mut callstk: Vec<usize> = Vec::new();
    let mut stksim: Vec<usize> = Vec::new();
    let mut callmode: Callmode = unsafe { CALLMODE_DEFAULT };
    //current access modifier
    let mut curmod: Mod = Mod::UNK;
    //access modifiers for every element of labels array
    let mut labmod: Vec<Mod> = Vec::new();
    let mut ind: isize = -1;
    while {ind+=1;ind} < pr.len() as isize{
        let i: &mut Tok = &mut pr[ind as usize];
        let val: &mut String = &mut i.1;
        let loc: &Loc = &i.0;
        let lin: &i64 = &loc.0;
        let index: &i64 = &loc.1;
        macro_rules! parseerr {
            ($($tail:expr),*) => {
                parseerrmsg!(lin, index, filename, $($tail),*);
            };
        }
        macro_rules! parsewarn {
            ($($tail:expr)*) => {
                parsewarnmsg!(lin, index, filename, $($tail),*);
            };
        }
        match val.as_str() {
            "/*" => {
                mlc += 1;
            },
            "*/" => {
                if mlc <= 0 {
                    parseerr!((Formatstr::from("Comment underflow! {0}").unwrap()
                               .format(&mlc.to_string()).unwrap()
                               .to_string()));
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
        if unsafe { PARSE_DEBUG } {
            eprintln!("parse: callstk={:?} val={} callmode={:?}",
                      callstk, repr(val.as_str()), callmode);
        }
        if unsafe { PARSE_DEBUG_STATE } {
            eprintln!("State: {:?}", state);
        }
        if unsafe { PARSE_DEBUG_ID } {
            eprintln!("scope_id: {:?}", scope_id);
        }

        res.append(&mut if val.as_str().chars().nth(0) == Some('\'') && matches!(state, State::NONE) {
            vec![
                (Ok(Op::Push(match val.as_str() {
                    "'" => ' ',
                    _ => {
                        let repred_string: String = urepr(&val[1..]);
                        if repred_string.len() > 1 {
                            parseerr!((Formatstr::from("Char is more than one symbol: {0}").unwrap()
                                       .format(&repred_string).unwrap()
                                       .to_string()));
                            return None;
                        }
                        repred_string.chars().nth(0).unwrap()
                    },
                } as i64)), *loc),
            ]
        } else if val.chars().nth(0) == Some('\"') && matches!(state, State::NONE) {
            let mut postfix: Option<usize> = None;
            let tmp: Vec<i64> = {
                let mut res: Vec<i64> = Vec::new();
                let mut jnd: isize = -1;
                while {jnd+=1;jnd} < val[1..].len() as isize {
                    let j: char = match val[1..].chars().nth(jnd as usize) {
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
            let tmp_str: String = urepr(tmp.iter().map(|x| char::from(*x as u8)).collect::<Vec<char>>().iter().collect::<String>().as_str());
            let _tmpstr: &str = tmp_str.as_str();
            let mut tmpres: Vec<i64> = tmp_str.chars().take(postfix.unwrap()).collect::<String>().chars().rev().collect::<String>().chars().map(|x| x as i64).collect();
            match tmp_str.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>().as_str() {
                "" => tmpres.push(tmp_str.len() as i64),
                "r" => {},
                "c" => tmpres.push('\0' as i64),
                _ => {
                    eprintln!("custom string postfixes are not implemented yet: {}", tmp_str.chars().rev().collect::<String>().chars().take(tmp.len()-postfix.unwrap()-0).collect::<String>());
                    return None;
                },
            }
            if unsafe { PARSE_DEBUG_STRING } {
                eprintln!("tmpres={:?}", tmpres);
            }
            data.append(&mut tmpres);
            vec![(Ok(Op::Push(data.len() as i64 - 1)), Loc(-1,-1))]
        } else {
            let check_for_hash = || -> Option<(String, Callmode)> {
                if val.chars().nth(0) == Some('#') {
                    return Some((val[1..].to_string(), unsafe { CALLMODE_ON_OPERATOR }));
                }
                return None;
            };
            match strtoi64(&val) {
            Some(x) => {
                res.append(&mut vec![
                    (Ok(Op::Push(x)), *loc),
                ]);
                stksim.push(res.len());
                continue;
            },
            None => {
                #[allow(unreachable_patterns)]
                match state {
                    State::NONE => {
                let matchresult: Op = match val.as_str() {
                    ""|"\n" => continue,
                    "pri" => {
                        curmod = Mod::PRI;
                        continue;
                    },
                    "pub" => {
                        curmod = Mod::PUB;
                        continue;
                    },
                    "+" => PLUS,
                    "--" => INVERT,
                    "*" => MUL,
                    "/" => DIV,
                    "putc" => PRINT,
                    "eputc" => EPRINT,
                    "puts" => PUTS,
                    "eputs" => EPUTS,
                    "putsln" => PUTSLN,
                    "eputsln" => EPUTSLN,
                    "flush" => FLUSH,
                    "eflush" => EFLUSH,
                    "input" => INP,
                    "lbl" => {
                        state = State::LBL;
                        curmod = Mod::UNK;
                        continue;
                    },
                    "fn" => {
                        state = State::FN;
                        continue;
                    },
                    "include" => {
                        state = State::INCLUDE;
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
                    "addr" => Push(res.len() as i64),
                    "paddr" => {
                        println!("paddr: {}", res.len());
                        continue;
                    },
                    "paddre" => {
                        println!("paddre: {}", res.len());
                        ind = res.len() as isize;
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
                            None => {
                                parseerr!(("call underflow!"));
                                return None;
                            },
                        };
                        if unsafe { PARSE_DEBUG_CALL } {
                            eprintln!("insertion_index={} res={:?}", insertion_index, res);
                        }

                        match callmode {
                            WITHOUT_ADDRESS => {
                                
                            },
                            WITH_ADDRESS_LEFT => {
                                res.insert(insertion_index, (Ok(Push((res.len()+2+result.len()) as i64)), *loc));
                            },
                            WITH_ADDRESS_RIGHT => {
                                res.push((Ok(Push((res.len()+1) as i64)), *loc));
                            },
                        }
                        callmode = unsafe { CALLMODE_DEFAULT };

                        //remove address to jump in
                        let element = res.remove(insertion_index-1);
                        //push it to the top
                        res.push(element);

                        res.push((Ok(G), *loc));
                        continue;
                    },
                    "#" => {
                        callmode = unsafe { CALLMODE_ON_OPERATOR };
                        continue;
                    },
                    "argc" => ARGC,
                    "argv" => ARGV,
                    "read" => READ,
                    "{" => {
                        scope_id.push(0);
                        continue;
                    },
                    "}" => {
                        _ = scope_id.pop().unwrap();
                        continue;
                    },
                    "gettime" => GETTIME,
                    "->" => DEREF,
                    "empty_op" => EMPTY,
                    _ => {
use crate::Callmode::*;
                        match callmode {
                            WITHOUT_ADDRESS => {
                                
                            },
                            WITH_ADDRESS_LEFT|WITH_ADDRESS_RIGHT => {
                                res.push((Ok(Push((result.len()+res.len()+1) as i64)), *loc));
                            },
                        }
                        callmode = unsafe { CALLMODE_DEFAULT };

                        res.append(&mut vec![
                            (Err((val.to_string(), scope_id.clone())), *loc),
                            (Ok(G), *loc),
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
                        if matches!(curmod, Mod::UNK) {
                            curmod = unsafe { CURMOD_DEFAULT };
                        }
                        if let "main" = &*val.as_str() {
                            main = Some(res.len() as usize);
                        }
                        labels.push((val.to_string(), None, scope_id.clone()));
                        labmod.push(curmod);
                        state = State::NONE;
                        continue;
                    },
                    State::FN => {
                        {
                            let len: usize = scope_id.len()-1;
                            scope_id[len] += 1;
                        }
                        let pos: usize = match labels.iter().position(|x| String::from(x.0.clone()).eq(val)) {
                            Some(pos) => pos,
                            None => {
                                if matches!(curmod, Mod::UNK) {
                                    curmod = unsafe { CURMOD_DEFAULT };
                                }
                                if let "main" = &*val.as_str() {
                                    main = Some((res.len()+result.len()) as usize);
                                }
                                labels.push((val.to_string(), Some((result.len()+res.len()) as i64), scope_id.clone()));
                                labmod.push(curmod);
                                state = State::NONE;
                                continue;
                            }
                        };
                        labels[pos].1 = Some(res.len() as i64);
                        if !matches!(curmod, Mod::UNK) {
                            parsewarn!(("access modifier does not need to be in definition of declared already function"));
                            curmod = Mod::UNK;
                        }
                        state = State::NONE;
                        continue;
                    },
                    State::INCLUDE => {
                        if unsafe { PARSE_DEBUG_INCLUDE } {
                            eprintln!("{}:{}:{}: including {}...", filename, lin, index, repr(&val));
                        }
                        let mut tokens = match parselexget(&(getstrfromtok(val)), include_level+1, scope_id.clone()) {
                            Some(x) => x,
                            None => {
                                return None;
                            },
                        };
                        scope_id = tokens.2;
                        // FIXME: implement including with access modifiers
                        let mut loopindex: usize = 0;
                        if unsafe { PARSE_DEBUG_INCLUDE_ADDING } {
                            eprintln!("adding {} fns...", tokens.0.len());
                        }
                        while loopindex < tokens.0.len() {
                            labmod.push(Mod::PUB);
                            loopindex += 1;
                        }
                        result.append(&mut tokens.1);
                        labels.append(&mut tokens.0);
                        if unsafe { PARSE_DEBUG_INCLUDE_SUCCED } {
                            eprintln!("{}:{}:{}: succed include {}", filename, lin, index, repr(&val));
                        }
                        state = State::NONE;
                        continue;
                    },
                    State::FNADDR => {
                        match check_for_hash() {
                            Some(x) => {
                                *val = x.0;
                                callmode = x.1;
                            },
                            None => {},
                        }
                        res.append(&mut vec![
                            (Err((val.to_string(), scope_id.clone())), *loc),
                        ]);
                        state = State::NONE;
                        continue;
                    },
                    State::DBGMSG => {
                        res.push((Ok(Op::DBGMSG(getstrfromtok(val))), *loc));
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
        }
    );}
    #[allow(unused_macros)]
    macro_rules! parseerr {
        ($($tail:expr),*) => {
            parseerrmsg!("EOF", "EOF", filename, $($tail),*);
        }
    }
    #[allow(unused_macros)]
    macro_rules! parsewarn {
        ($($tail:expr),*) => {
            parsewarnmsg!("EOF", "EOF", filename, $($tail),*);
        }
    }
    if !matches!(state, State::NONE) {
        parseerr!(("Parsing is ended but state is not none"));
    }
    if callstk.len() > 0 {
        parseerr!(("Callstk is not empty"));
    }

    res.append(&mut vec![
        (Ok(Push(0)), Loc(-2,-2)),
        (Ok(EXIT), Loc(-2,-2)),
    ]);

    result.append(&mut match matchlink(&filename,
                                       &res,
                                       &labels,
                                       &main,
                                       &mut data,
                                       include_level) {
        Some(x) => x.0,
        None => return None,
    });

    {
        if labels.len() != labmod.len() {
            eprintln!("{}: lengths are not the same: {} and {}:\n   {:?}\n  {:?}", filename, labels.len(), labmod.len(), labels, labmod);
            todo!();
        }
        let mut ind: usize = 0;
        while ind < labels.len() {
            if !matches!(labmod[ind], Mod::PUB) {
                labels.remove(ind);
                labmod.remove(ind);
            } else {
                ind += 1;
            }
        }
    }

    return Some((labels,
                 result,
                 scope_id,
                 Box::new(data)));
}
fn link<'a>(filename: &String,
        res: &Vec<(Result<Op, (String, Vec<usize>)>, Loc)>,
        labels: &Vec<(String, Option<i64>, Vec<usize>)>,
        main: &Option<usize>,
        data: &'a mut Vec<i64>,
        include_level: usize) -> Option<(Vec<(Op, Loc)>, &'a mut Vec<i64>)> {
    eprintln!("[linking {}...[recursion_level: {}]]",
              repr(filename), include_level);
    let mut linkres: Vec<(Op, Loc)> = Vec::new();
    #[allow(unused_variables)]
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
                    //if (Label)(j.name) = (Op)(x.Err.String)
                    if String::from(j.0.clone()).eq(&String::from(x.0.clone())) {
                        if covariant_right(&j.2, &x.1) {
                            match j.1 {
                                //found definition
                                Some(def) => {
                                    linkres.push((Op::Push(def), loc));
                                },
                                //not found definition
                                None => {
                                    eprintln!("{}:{}:{}: Error: label is declared, but has no definition", filename, lin, index);
                                    return None;
                                }
                            }
                        } else {
                            eprintln!("{}:{}:{}: Warning: label is private", filename, lin, index);
                            //return None;
                        }
                    } else {
                        ret += 1;
                    }
                }
                if ret >= labels.len() as i64 - 1 {
                    parseerrmsg!(lin, index, filename,
                                 (Formatstr::from("label not found: {0}").unwrap()
                                  .format(&repr(&x.0)).unwrap()
                                  .to_string()));
                    return None;
                }
            },
        };
    }
    if include_level == 0 {
        linkres.push((Op::Push(match main {
            Some(x) => *x as i64,
            None => 0,
        }), Loc(-2,-2)));
    }
    return Some((linkres, data));
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum simResult {
    ok(i32),
    err,
    errs(String),
    stopped,
}
fn sim(pr: &mut Vec<(Op, Loc)>,
       filename: &String,
       argv: Vec<String>,
       data: Box<Vec<i64>>,
       output_to_file: Option<String>) -> simResult {
use simResult::*;
use std::fs::{File, OpenOptions};
use crate::Op::*;
    if !unsafe { LINK_DEBUG } {
        eprintln!("[simulation...]");
    } else {
        eprintln!("[simulation:");
        let mut ind: usize = 0;
        for i in &mut *pr {
            eprintln!("  {}  {}:{}:{:?}",
                      ind, i.1.0, i.1.1, i.0);
            ind += 1;
        }
        eprintln!("]");
    }
    if unsafe { ONLY_LINK } {
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
    let f: Option<File> = match output_to_file {
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
    while ind != pr.len() as i64 {
        ind += 1;
        let i: &Op = &pr[{let tmp: usize = ind as usize; if tmp >= pr.len() {break;} else {tmp}}].0;
        let loc: &Loc = &pr[ind as usize].1;
        let lin: i64 = loc.0;
        let index: i64 = loc.1;
        if unsafe { SIM_DEBUG } {
            eprintln!("{}------------ {}. {}:{}:{}:{:?}{}\n{:?}",
                      YELLOW_COLOR,
                      ind,
                      repr(&filename),
                      lin,
                      index,
                      i,
                      RESET_COLOR,
                      stack);
        }
        #[allow(unreachable_patterns)]
        match i {
            Push(x) => {
                stack.push(*x);
            },
            PRINT|EPRINT => {
                match output_to_file {
                    Some(_) => {
                        _ = f.as_ref().unwrap().write(&[stack.pop().unwrap() as u8]);
                    },
                    None => {
                        let chr: char = char::from_u32(stack.pop().unwrap() as u32).unwrap();
                        match i {
                            PRINT => print!("{}", chr),
                            EPRINT => eprint!("{}", chr),
                            _ => todo!(),
                        }
                    },
                }
            },
            PUTS|EPUTS|PUTSLN|EPUTSLN => {
                if unsafe { SIM_DEBUG_PUTS } && !unsafe { SIM_DEBUG } {
                    eprintln!("debug: puts: {:?}", stack);
                }
                let strptr: usize = stack.pop().unwrap() as usize;
                let strlen: usize = data[strptr] as usize;
                if data.len() < strlen {
                    return errs("puts underflow".to_owned());
                }
                let mut string: String = String::new();
                {
                    let mut ind2: usize = 0;
                    while ind2 <= strlen {
                        let chr = char::from_u32(data[strptr-ind2] as u32).unwrap();
                        string.push(chr);
                        ind2 += 1;
                    }
                }
                match output_to_file {
                    Some(_) => {
                        _ = f.as_ref().unwrap().write(string.as_bytes());
                    },
                    None => {
                        match i {
                            PUTS => { print!("{}", string); },
                            EPUTS => { eprint!("{}", string); },
                            PUTSLN => { println!("{}", string); },
                            EPUTSLN => { eprintln!("{}", string); },
                            _ => todo!(),
                        }
                    },
                }
            },
            FLUSH => {
                _ = std::io::stdout().flush();
            },
            EFLUSH => {
                _ = std::io::stderr().flush();
            },
            INP => {
                let mut input: String = String::new();
use std::io::stdin;
                _ = stdin().read_line(&mut input);
                stack.append(&mut from(&input).iter().rev().map(|x| *x).collect::<Vec<i64>>());
                stack.push(input.len() as i64);
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
            INVERT => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for INVERT intrinsic not found".to_string());
                    },
                };
                stack.push(-a);
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
            DIV => {
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for DIV intrinsic not found".to_string());
                    },
                };
                if b == 0 {
                    return errs("Cannot divide by zero (0)".to_string());
                }
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for DIV intrinsic not found".to_string());
                    },
                };
                stack.push(a/b);
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
                    ind = addr as i64;
                }
            },
            G => {
                let addr: i64 = match stack.pop() {
                    Some(x) => x-1,
                    None => {
                        return errs("Operand `addr` for G intrinsic not found".to_string());
                    },
                };
                ind = addr as i64;
            },
            PUSHNTH => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for PUSHNTH intrinsic not found".to_string());
                    },
                };
                if a >= stack.len() as i64 {
                    return errs(Formatstr::from("pushnth overflow: operand `a` is {0}, len is {1}").unwrap()
                                .format(&a.to_string()).unwrap()
                                .format(&stack.len().to_string()).unwrap()
                                .to_string());
                }
                let b: i64 = stack[stack.len()-1-a as usize];
                stack.push(b);
            },
            DROPNTH => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for DROPNTH intrinsic not found".to_string());
                    },
                };
                if a >= stack.len() as i64 {
                    return errs("dropnth overflow".to_string());
                }
                stack.remove(stack.len()-1-a as usize);
            },
            NBROT => {
                let l: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `l` for NBROT intrinsic not found".to_string());
                    },
                };
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for NBROT intrinsic not found".to_string());
                    },
                };
                stack.insert(stack.len()-0-l as usize, a);
            },
            LT => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for LT intrinsic not found".to_string());
                    },
                };
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for LT intrinsic not found".to_string());
                    },
                };
                stack.push((b < a) as i64);
            },
            EQ => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for EQ intrinsic not found".to_string());
                    },
                };
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for EQ intrinsic not found".to_string());
                    },
                };
                stack.push((b == a) as i64);
            },
            NOT => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for NOT intrinsic not found".to_string());
                    },
                };
                stack.push((a == 0) as i64);
            },
            OR => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for OR intrinsic not found".to_string());
                    },
                };
                let b: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `b` for OR intrinsic not found".to_string());
                    },
                };
                stack.push(((a != 0) || (b != 0)) as i64);
            },
            EXIT => {
                println!();
                let errorcode: i32 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `errorcode` for EXIT intrinsic not found".to_string());
                    },
                } as i32;
                return ok(errorcode);
            },
            PSTK => {
                //debugging with `??#` and `???` is not important when every step is already debugged
                if !unsafe { SIM_DEBUG } {
                    println!("{}:{}: pstk  {:?}", lin, index, stack);
                }
            },
            PSTKE => {
                //see PSTK
                if !unsafe { SIM_DEBUG } {
                    println!("{}:{}: pstke {:?}", lin, index, stack);
                }
                return err;
            },
            DUMP => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for DUMP intrinsic not found".to_owned());
                    },
                };
                println!("dump: {}", a);
            },
            ARGC => {
                stack.push(argv.len() as i64);
            },
            ARGV => {
                let a: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `a` for ARGV intrinsic not found".to_owned());
                    },
                };
                if a >= argv.len() as i64 {
                    return errs("Argv overflow".to_owned());
                }
                if a < 0 {
                    return errs("Argv underflow".to_owned());
                }
                for j in argv[a as usize].chars().rev() {
                    stack.push(j as i64);
                }
                stack.push(argv[a as usize].len() as i64);
            },
            READ => {
                let mut filename: String = String::new();
                let filename_len: usize = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `filename_len` for READ intrinsic not found".to_owned());
                    },
                } as usize;
                let mut ind: usize = 0;
                while {ind+=1;ind} < filename_len+1 {
                    let i: i64 = stack.pop().unwrap();
                    filename.push(i as u8 as char);
                }
                let file: String = match std::fs::read_to_string(filename) {
                    Ok(x) => x.chars().rev().collect(),
                    Err(_x) => {
                        stack.push(-1);
                        continue;
                    },
                };
                stack.append(&mut file.chars().map(|x| x as i64).collect::<Vec<i64>>());
                stack.push(file.len() as i64);
            },
            GETTIME => {
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};
                let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("lol").as_nanos();
                stack.push(hi(time));
                stack.push(lo(time));
            },
            DBGMSG(x) => {
                println!("dbgmsg: {}", repr(x));
            },
            DEREF => {
                let ptr: i64 = match stack.pop() {
                    Some(x) => x,
                    None => {
                        return errs("Operand `ptr` for DEREF intrinsic not found".to_string());
                    },
                };
                stack.push(data[ptr as usize]);
            },
            EMPTY => {},
            _ => {
                return errs(Formatstr::from("Unknown op: {0}").unwrap()
                            .format(&i.to_string()).unwrap()
                            .to_string());
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
            #[allow(unreachable_patterns)]
            match mode {
                Mode::SIM => {
                    for_each_arg(&args,
                                 |i: &String,
                                 ind: isize,
                                 argv: &Vec<String>,
                                 fargs: &Vec<String>,
                                 args: &Vec<String>,
                                 output_to_file: Option<String>| {
use simResult::*;
                        #[allow(unused_assignments)]
                        let mut data: Option<Box<Vec<i64>>> = None;
                        let error: simResult = sim(&mut match parselexget(&i, 0, vec![0,]) {
                            Some(x) => {
                                data = Some(x.3);
                                x.1
                            },
                            None => return,
                        }, &i, if ind==(argv.len()-1) as isize {
                            fargs.clone()
                        } else {
                            vec![
                                args[0].clone(),
                            ]
                        }, data.unwrap(), output_to_file);
                        #[allow(unreachable_patterns)]
                        match error {
                            ok(x) => {
                                if x == 0 {
                                eprintln!("{}", Formatstr::from("[Simulation of {0} {1}succed{2}]").unwrap()
                                          .format(&repr(&i)).unwrap()
                                          .format(GREEN_COLOR).unwrap()
                                          .format(RESET_COLOR).unwrap()
                                          .to_string());
                                } else {
                                    eprintln!("[Simulation of {} was finished with exit code {}]", repr(&i), x);
                                }
                            },
                            err => {
                                eprintln!("{}", Formatstr::from("[Simulation of {0} {1}failed{2}]").unwrap()
                                          .format(&repr(&i)).unwrap()
                                          .format(RED_COLOR).unwrap()
                                          .format(RESET_COLOR).unwrap()
                                          .to_string());
                            },
                            errs(x) => {
                                eprintln!("{}", Formatstr::from("[Simulation of {0} {1}failed{2} due to this error: {3}]").unwrap()
                                          .format(&repr(&i)).unwrap()
                                          .format(RED_COLOR).unwrap()
                                          .format(RESET_COLOR).unwrap()
                                          .format(&repr(&x)).unwrap()
                                          .to_string());
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
                                        _ind: isize,
                                        _argv: &Vec<String>,
                                        _fargs: &Vec<String>,
                                        _args: &Vec<String>,
                                        output_to_file: Option<String>| {
                        let tokens: Vec<(Op, Loc)> = match parselexget(&i, 0, vec![0,]) { 
                            Some(x) => x.1,
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
                                        Err(_e) => {
                                            todo!();
                                        },
                                    };
                                    _ = f.write(b"");
                                }
    	                        let mut f = match OpenOptions::new().append(true).open(x) {
                                    Ok(y) => y,
                                    #[allow(unused_variables)]
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
    println!("{} => {}", repr("Hello, {0} (fuck {1}) {0}!
    {abc} def {2}"), repr(&Formatstr::from("Hello, {0} (fuck {1}) {0}!
                         {abc} def {2}").unwrap()
         .format("world").unwrap()
         .format("you").unwrap()
         .format("ebal").unwrap()
         .to_string()
    ));
}
fn _main() {
    let args: Vec<String> = std::env::args().collect();
    clah(&args);
}
fn main() {
    _main();
}
//bip bop. this is the end of the code
