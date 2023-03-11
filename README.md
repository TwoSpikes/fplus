# F+ (Forth+)

Stack-based programming language made by me (TwoSpikes, 2022-2023) for studing purposes.\
There're my goals for the future:\
[X] Turing-completeness (`:` and `if` operations).\
[X] Multi-line comment support (now they're like in CSS, but not exactly).\
[X] Chars (`''`), strings (`""`), c-style strings (`""c`) and raw strings (`""b`).\
[X] repr(&str) and urepr(&str) functions.\
[X] Function predeclaration (linking).\
[ ] `include` keyword.\
[ ] `macro` keyword.\
[X] Drop-in documentation.\
[ ] Token dumping.\
[ ] Tokens to F+ code.\
[ ] Tokens compilation.\
[ ] Vim and Emacs syntax highlighting (later).\
[ ] Self-hosted compiler (this thing was abandoned, maybe forever).

Supported compilation modes:
- Simulation (default)

Installing:
```console
$ cargo init
$ cargo build --release
```

Usage:
./target/release/fplus SUBCOMMAND [OPTION]... [SOURCE]... -- [ARG]...

SUBCOMMAND:\
sim, s            simulate program\
version, ver, v   print version information and exit

EXAMPLES:
`$ ./target/release/fplus sim main.tspol` ALSO KNOWN AS `$ ./target/release/fplus sim main.tspol --`
F+ will simulate ./main.tspol file\

```console
$ ./target/release sim main.tspol -- a b c
```
F+ will simulate ./main.tspol file with `a b c` command line arguments\

OPTIONS:\
Options are not in the compiler yet
