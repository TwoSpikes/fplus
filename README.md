F+ (Forth+)

Stack-based programming language made by me (TwoSpikes, 2022-2023) for studing purposes.

Done:
- [x] Turing-completeness (`:` and `if` operations)
- [x] Multi-line comment support (`/* comment */`)
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
- [x] Ability to stderr printing
- [x] File reading
- [x] `gettime` operation — returns time in nanoseconds as i128
- [x] Colored debug output (Error (red) and Warning (yellow))
- [x] Variables that switch debug information on/off through file `debug.tsconf` or through command line options
- [x] Unsigned 64-bit numbers (same type with signed numbers) (like `30u`)

In progress:

To do:
- [ ] Float numbers (same type with "normal" numbers) (F64)
- [ ] Float numbers with double precision (same type with "normal" numbers) (D64)
- [ ] `macro` keyword
- [ ] `undump` subcommand
- [ ] `com` and `token-com` subcommand (for compilation)
- [ ] `addsource` keyword
- [ ] Vim and Emacs syntax highlighting (later)
- [ ] Colored output somehow through escaping
- [ ] C-like file reading
- [ ] Raw file reading
- [ ] File writing
- [ ] Self-hosted compiler (this thing was abandoned, maybe forever)
- [ ] One-line comment support (`// comment`)

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
- [x] I64ToU64 function ((val: I64) -> (U64))
- [x] U64ToI64 function ((val: I64) -> (I64))
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

Dependences:
* Cargo

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

#### VERSION:
```
F+, a stack-based interpreting programming language
written on Rust v.1.68.2
version: 0.1.0-4
download: https://github.com/TwoSpikes/fplus
2022-2023 @ TwoSpikes
```

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

Pushnth takes one argument: `(I64) a`\
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

Consumes 2 arguments: `(I64) a, (I64) b`\
Returns: `(I64)(a + b)`

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

Consumes 2 arguments: `(I64) a, (I64) b`\
Returns: `(I64)(a * b)`

```fplus
2 3 *
```
Stack: [6]

```fplus
-49 2 *
```
Stack: [-98]

#### Division `/`

Consumes 2 arguments: `(I64) a, (I64) b`\
Returns: `(I64)(a / b)`

```fplus
2 / 3
```
Stack: [0]

```fplus
48 -2 /
```
Stack: [-24]

#### Less than `<`

Consumes 2 arguments: `(I64) a, (I64) b`\
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

Consumes 2 arguments: `(I64) a, (I64) b`\
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

Consumes 1 argument: `(I64) a`\
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

Consumes 1 argument: `(I64) a`\
Print `a` without converting it from ASCII to string.\
Need for debugging purposes.

```fplus
69 dump
```
Stdout is empty.\
Stderr: "69"

#### Exit `exit`

Exit the program

Consumes 1 argument: `(I64) exitcode`\
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

Consumes 1 argument: `(I64) chr`\
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
| '\\r'                     | carriage return             |
| '\\n'                     | new string                  |
| '\\t'                     | tab                         |
| '\\\\'                    | backslash                   |
| '\\''                     | quote                       |
| '\\"'                     | double quote                |

Strings are inverted at the stack and has its length at the end.

Empty string:
```fplus
""
```
Stack: [0]

## Functions and calls

#### Function

First, you need to create a function

We will call it `foo`:

Syntax:
fn `{function name}`

```fplus
fn foo
```

#### Main function

`main` function is the start of the program

If you don't have `main` function then program will start from first line of file you given.

You cannot have the `main` function (maybe later I will implement it) in files that you [included](#include).

You will define it like this:
```fplus
fn main
```

With the [previous function we created: `foo`](#foo):

```fplus
fn foo

fn main
```

#### Function calling

You can call your function by just writing its name:
```fplus
fn foo

fn main
  foo
```

But this program will go into the infinite loop (because it is returing to `main` function) so you have to [exit](#exit) it somewhere.

```fplus
fn foo
  1 exit

fn main
  foo
```

Let's print string as indicator that we called it:
```fplus
fn foo
  "foo called" puts
  1 exit

fn main
  foo
```

Okay, now let's move on!

Actually you called it and save your next operation address to jump to it at the end of function.

Let's check:
```fplus
fn foo
  "foo called" puts
  dump
  1 exit

fn main
  foo
```
Stdout: "foo called\n16\n"

This address is too large because of our string.\
Every char is the 1 operation (and the length of string too).

We do not need it now so I will show how to not do that:
```fplus
fn foo
  "foo called" puts
  1 exit

fn main
  #foo
```

Yeah, you just need to put `#` sign before function name

#### Get function address

You can get function addresses by pushing `:` character before its name

Let's get address of `foo` function

```fplus
fn foo
  "foo called" puts
  1 exit

fn main
  :foo
```
Stack: [0] because `foo` starts at first operation

#### Scopes

You cannot access fns outside of its scope.

```fplus
fn a {
  fn a.1
  fn a.2
  fn a.3
}

fn main
  :a   /* 0 */
  :a.1 /* Error: label is private */
```

## Function arguments

#### Old way

You can provide arguments to function, placing them before calling:
```fplus
fn foo
  dump dump dump
  0 exit

fn main
  1 2 3 #foo
```
Stdout: "3\n2\n1\n"

#### New way

```fplus
fn foo
  dump dump dump
  0 exit

fn main
  #foo(1 2 3)
```
Stdout: "3\n2\n1\n"

## Comments

#### Multi-line

```fplus
/* this is a comment */

/* this is a nested comment
  /* yeah, nested comment */
*/
```

Yes, space (or new line or tab) before and after `/*` is neccesary
