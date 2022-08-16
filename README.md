# My Project Euler solutions

[Project Euler](https://projecteuler.net/) is a collection of programming problems.
I solve these problems using rust to exercise my problem solving and programming skills.
It's also an excuse to learn Rust.

## Warning

If you haven't solved these problems yourself, don't look at my solutions to cheat.
You'll only cheat yourself If you do. Additionally,
I can't guarrantee that they're the best way to solve them.
They're just the best possible solutions I could think off or the ones I like.

## Running

To run a solution use: 

```
$ cargo run -qr <problem_number>
```

To run all solutions use: 

```
$ cargo run -qr -- --all
```

## Directory Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs			// holds related functions used by solutions
    ├── main.rs			// runs solution based on choice
    ├── solutions
    │   └── pX.rs		// where X is a number related to problem number
    └── solutions.rs	// importing solutions
```

## Solutions file

Solutions are put in functions with a version number, eg. `s_v1()`, `s_v2()`, `s_v3()` etc.
Only the latest version number is executed as its supposed to be better.

## Problems Solved
- [x] problem 1 to 10
- [x] problem 11 to 20
- [x] problem 35
