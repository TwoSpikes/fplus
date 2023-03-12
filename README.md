# F+ (Forth+)

Stack-based programming language made by me (TwoSpikes, 2022-2023) for studing purposes.
There're my goals for the future:
- [x] Turing-completeness (`:` and `if` operations).
- [x] Multi-line comment support (now they're like in CSS, but not exactly).
- [x] Chars (`''`), strings (`""`), c-style strings (`""c`) and raw strings (`""b`).
- [x] repr(&str) and urepr(&str) functions.
- [x] Function predeclaration (linking).
- [ ] `include` keyword.
- [ ] `macro` keyword.
- [x] Drop-in documentation.
- [ ] Token dumping.
- [ ] Tokens to F+ code.
- [ ] Tokens compilation.
- [ ] Vim and Emacs syntax highlighting (later).
- [ ] Self-hosted compiler (this thing was abandoned, maybe forever).

Supported compilation modes:
- Simulation (default)

# Compiler

Installing:
```console
$ cargo init
$ cargo build --release
```

Usage:
```console
$ ./target/release/fplus SUBCOMMAND [OPTION]... [SOURCE]... -- [ARG]...
```

```
SUBCOMMAND:
sim, s            simulate program
version, ver, v   print version information and exit
usage, use, u, help, h, ?, info, information
		  print help information and exit
```

EXAMPLES:
```console
$ ./target/release/fplus sim main.tspol
```
ALSO KNOWN AS
```console
$ ./target/release/fplus sim main.tspol --
```
F+ will simulate ./main.tspol file


```console
$ ./target/release sim main.tspol -- a b c
```
F+ will simulate ./main.tspol file with `a b c` command line arguments

OPTIONS:\
Options are not in the compiler yet

# Programming language

## Stack

Stack is a dynamic array of 64-bit integer numbers (from -9223372036854775808 (-2^63) to 9223372036854775807 (2^63-1)).

### Pushing numbers

```fplus
34 36
```
This program will push 34 and 36 on the stack.\
Stack: [34, 36]

### Copying elements (numbers) — `pushnth`

Pushnth takes one argument.\
First, provide this argument, and second, write `pushnth`

If stack is [10, 11, 12, 13, 14],\
After providing argument (3), it will be [10, 11, 12, 13, 14, 3]\
It will consume argument and take third element from right counting from 0 (it is 11).

| Argument             | 0  | 1  | 2  | 3  | 4  | 5  |
| ---                  | --- | --- | --- | --- | --- | --- |
| Index (from right)   | 4  | 3  | 2  | 1  | 0  | NA |
| ---                  | --- | --- | --- | --- | --- | --- |
| Element (from right) | 14 | 13 | 12 | 11 | 10 | NA |
| ---                  | --- | --- | --- | --- | --- | --- |
| Element (from left)  | 10 | 11 | 12 | 13 | 14 | NA |


```
34 36
0 pushnth
```
Stack: [34, 36, 36]
