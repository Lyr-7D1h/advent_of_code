# Solution to Advent of Code 2022

Written in rust.

All solutions are seperate binaries in `src/bin` which use `src/lib.rs` to parse arguments given and to track performance.


## Usage

```bash
cargo run --bin=day{n} -- {part} {input}
```

or 

```bash
./run day{n} {part} {input}
./run day01 one example
```

Example

```bash
cargo run --bin=day1 -- two_imperative input/day1.txt
```
