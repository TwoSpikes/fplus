1 F+ (Forth+)

Stack-based programming language made by me (TwoSpikes, 2022-2023) for studing purposes.

Done:
- [x] Turing-completeness (`:` and `if` operations)
- [x] Multi-line comment support (they're like in CSS (`/* */`), but not 100%)
- [x] Chars (`''`), strings (`""`) and string postfixes (`""b` and `""c`)
- [x] repr(&str) and urepr(&str) functions (proper escaping like in C)
- [x] Function predeclaration (linking)
- [x] Negative numbers (`-1`, `---50`)
- [x] Drop-in documentation
- [x] `dump` subcommand (dump tokens)
- [x] Output debug information to stderr instead of stdout
- [x] Command line arguments (see [argc](#argc) and [argv](#argv))
- [x] Global variables that switch debug information on/off
- [x] `include` keyword
- [x] Nested label scopes

In progress:

To do:
- [ ] Float numbers (same type with "normal" numbers) (F64)
- [ ] Float numbers with double precision (same type with "normal" numbers) (D64)
- [ ] Unsigned 64-bit numbers (same type with "normal" numbers) (like `30u`)
- [ ] `macro` keyword
- [ ] `undump` subcommand
- [ ] `com` and `token-com` subcommand (for compilation)
- [ ] `native` keyword
- [ ] Vim and Emacs syntax highlighting (later)
- [ ] Ability to stderr printing
- [ ] Variables that switch debug information on/off through file `debug.tsconf` or through command line options
- [ ] Colored output somehow through escaping
- [ ] Colored debug output
- [ ] Self-hosted compiler (this thing was abandoned, maybe forever)

Standard library:
- [x] max\_2\_I64 function ((b: I64, a: I64) -> I64)
- [x] min\_2\_I64 function ((b: I64, a: I64) -> I64)
- [ ] I64ToStr function ((val: I64) -> (len: I64, I64[len]))
- [ ] U64ToStr function ((val: U64) -> (len: I64, I64[len]))
- [ ] F64ToStr function ((val: F64) -> (len: I64, I64[len]))
- [ ] D64ToStr function ((val: D64) -> (len: I64, I64[len]))
- [ ] StrToI64 function ((len: I64, arr: I64[len]) -> (I64))
- [ ] StrToU64 function ((len: I64, arr: I64[len]) -> (U64))
- [ ] StrToF64 function ((len: I64, arr: I64[len]) -> (F64))
- [ ] StrToD64 function ((len: I64, arr: I64[len]) -> (D64))
- [ ] I64ToU64 function ((val: I64) -> (U64))
- [ ] U64ToI64 function ((val: I64) -> (I64))
- [ ] I64ToF64 function ((val: I64) -> (F64))
- [ ] F64ToI64 function ((val: F64) -> (I64))
- [ ] U64ToF64 function ((val: U64) -> (F64))
- [ ] F64ToU64 function ((val: F64) -> (U64))
- [ ] I64ToD64 function ((val: I64) -> (D64))
- [ ] D64ToI64 function ((val: D64) -> (I64))
- [ ] U64ToD64 function ((val: U64) -> (D64))
- [ ] D64ToU64 function ((val: D64) -> (U64))
- [ ] F64ToD64 function ((val: F64) -> (D64))
- [ ] D64ToF64 function ((val: D64) -> (F64))

[\\]: # (TODO: print about functions and function calls)
[\\]: # (TODO: remove that stderr printing is not implemented yet when it is implemented)

Supported compilation modes:
- Simulation (default)
- C (not implemented yet)

# Compiler

#### Installing (Building):
```console
$ cargo build --release
```

#### Usage:
```console
$ ./target/release/fplus SUBCOMMAND [OPTION]... [SOURCE]... -- [ARG]...

SUBCOMMAND:
sim s                 Simulate program
version ver v         Print version information and exit
usage use u help h ? info information
		      Print help information and exit
dump d                Dump the tokens of the program.
error e               Print error codes and information about them
```

While building, you must be in `fplus` directory (it is important)

#### Error codes:
This message will be shown with `error` (or `e`) subcommand:
```console
errorcodes:
E0                    Cannot open file
```

## OPTIONS:
```
-o --output [FILE]            save output to program
-c --compiler [OPTION]        provide option to compiler (not implemented yet)
```

## EXAMPLES:

#### `sim` subcommand examples:
```console
$ ./target/release/fplus sim main.tspol

ALSO KNOWN AS

$ ./target/release/fplus sim main.tspol --
```
F+ will simulate ./main.tspol file

```console
$ ./target/release/fplus sim main.tspol -- a b c
```
F+ will simulate ./main.tspol file with `a b c` command line arguments
#### `dump` subcommand examples:

```console
$ ./target/release/fplus dump subc-dump.tspl -o subc-dump-file.txt
```
It will translate F+ code to tokens and write this to subc-dump-file.txt:

```
2:1:PUSHNTH
2:11:DROPNTH
2:19:NBROT
2:25:Push(2)
2:27:MUL
2:29:PLUS
3:1:DUMP
3:8:PUTS
3:13:Push(6969)
3:18:PRINT
-2:-2:Push(0)
```
If you will not provide `-o` option, tokens will dump on the stdout instead (with debug information).

#### `-o` option examples:
```console
$ ./target/release/fplus sim hello-world.tspl -o hello-world-output.txt
```
It will write
```
Hello, World!

```
into the `hello-world-output.txt` file.

If you will not provide `-o` option, Hello, World! will be printed to the stdout instead.

## VERSION:
```
F+, a stack-based interpreting programming language
written on Rust v.1.68.1
version: 0.1.0-4
download: https://github.com/TwoSpikes/fplus
2022-2023 @ TwoSpikes
```

# Programming language

## Stack

Stack is a dynamic array of 64-bit integer numbers (from -9223372036854775808 (-2^63) to 9223372036854775807 (2^63-1)).

#### Pushing numbers

```fplus
34 36
```
This program will push 34 and 36 on the stack.\
Stack: [34, 36]

#### Copying elements (numbers) — `pushnth`

Pushnth takes one argument: `(number) a`\
First, provide this argument, and second, write `pushnth`

If stack is [10, 11, 12, 13, 14],\
After providing argument (3), it will be [10, 11, 12, 13, 14, 3]\
It will consume argument and take third element from right counting from 0 (it is 11).

| Argument             | 0  | 1  | 2  | 3  | 4  | 5  |
| ---            | --- | --- | --- | --- | --- | --- |
| Index (from right)   | 4  | 3  | 2  | 1  | 0  | NA |
| *Element (from right)* | *14* | *13* | *12* | *11* | *10* | *NA* |
| Element (from left)  | 10 | 11 | 12 | 13 | 14 | NA |


```fplus
34 36
0 pushnth
```
Stack: [34, 36, 36]

## Arithmetic

#### Plus `+`

Consumes 2 arguments: `(number) a, (number) b`\
Returns: `(number)(a + b)`

```fplus
34 35 +
```
Stack: [69]

To sum up more than 2 numbers:
```fplus
1 1 + 1 +
```
Stack: [3]

To substract, use [Negative numbers](#Negative-numbers):
```fplus
69 -21 +
```
Stack: [48]

#### Multiply `*`

Consumes 2 arguments: `(number) a, (number) b`\
Returns: `(number)(a * b)`

```fplus
2 3 *
```
Stack: [6]

```fplus
-49 2 *
```
Stack: [-98]

#### Less than `<`

Consumes 2 arguments: `(number) a, (number) b`\
Returns: `(boolean)(a < b)`

```fplus
34 35 <
```
Stack: [1] because (34 < 35 <=> true)

```fplus
35 34 <
```
Stack: [0] because (35 < 34 <=> false)

```fplus
34 34 <
```
Stack: [0] because (34 < 34 <=> false)

#### Equals `=`

Consumes 2 arguments: `(number) a, (number) b`\
Returns: `(boolean)(a = b)`

```fplus
34 34 =
```
Stack: [1] because (34 = 34 <=> true)

```fplus
35 34 =
```
Stack: [0] because (35 = 34 <=> false)

## Boolean

#### Not `!`

Consumes one argument: `(boolean) a`\
Returns: `(boolean) !a`

```fplus
0 !
```
Stack: [1] because (!0 <=> 1)

```fplus
35 34 <
!
```
Stack: [1] because
1. 35 < 34 <=> false
2. !Ans <=> true

```fplus
1 ! !
```
Stack: [1] because
1. !1 <=> 0
2. !Ans <=> 1

```fplus
69 ! !
```
Stack: [1] because
1. !69 <=> 0
2. !Ans <=> 1

#### Or `|`

Consumes 2 arguments: `(boolean) a, (boolean) b`\
Returns: `(boolean)(a | b)`

```fplus
0 1 |
```
Stack: [1] because (0 | 1 <=> true)

```fplus
0 0 |
```
Stack: [0] because (0 | 0 <=> false)

```fplus
1 1 |
```
Stack: [1] because (1 | 1 <=> true)

Same will happen if all 1's will change to bigger numbers because `Or` anyways will cast arguments to boolean.

## Command line arguments

#### Providing
```console
$ ./target/release/fplus sim main.tspl -- a b c
```

This option will provide `main.tspl`, `a`, `b` and `c` command line arguments.
This is how to get access to him:

#### Argc `argc`

Consumes 0 arguments.\
Returns number of provided command line arguments.

```fplus
argc
```
Stack: [4]

#### Argv `argv`

Consumes 1 argument: `(number) a`\
Returns [String](#String) with a-th command line argument.

```fplus
2 argv
```
Stack: [98, 1] or "b"

All this programs will work like that only if you provide `a`, `b` and `c` command line arguments.

## Files

#### Reading `read`

Consumes 1 [String](#String).\
Return 1 [String](#String): file's content with given name.

```fplus
"main.tspl" read
```
Stack: [108, 112, 115, 116, 46, 110, 105, 97, 109, 9, 16] or '"main.tspl" read'

## Special

#### Empty op `empty_op`

Does nothing. Just need for debugging purposes.

#### Dump `dump`

Consumes 1 argument: `(number) a`\
Print `a` without converting it from ASCII to string.\
Need for debugging purposes.

```fplus
69 dump
```
Stdout is empty.\
Stderr: "69"

#### Exit `exit`

Exit the program

Consumes 1 argument: `(number) exitcode`\
Exits the program. It does not need to return anything.

```fplus
0 exit
```
Stderr: '[Simulation of "main.tspl" succed]'

```fplus
1 exit
```
Stderr: '[Simulation of "main.tspl" finished with exit code 1]'

## Input, Output (I/O)

#### Print char `putc`

Consumes 1 argument: `(number) chr`\
Returns nothing.\
Prints chr as ASCII to stdout ([printing to stderr](#stderr printing) is not implemented yet).

```fplus
69 putc
```
Stdout: "E"

#### Print string `puts`

I will write how to make [String](#String) later.

```fplus
65 66 67 3 args
```
Stdout: "abc"

```fplus
"Hello, World!" puts
```
Stdout: "Hello, World!"

## Literals

#### String

Strings are like:
```fplus
"Hello, World!"
```
Stack: [33, 100, 108, 114, 111, 87, 32, 44, 111, 108, 108, 101, 72, 13]

You can use simple escaping like in C like this:

| Code                      | Stdout                      |
| ---                       | ---                         |
| "\n"                      | new string                  |
| "\t"                      | tab                         |
| "\\"                      | backslash                   |
| "\'"                      | quote                       |
| "\""                      | double quote                |

Strings are inverted at the stack and has its length at the end.
