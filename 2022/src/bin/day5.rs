use advent_of_code_2022::{Aoc, Input};
use std::{collections::VecDeque, io::BufRead};

#[derive(Debug)]
struct Procedure {
    table: Vec<VecDeque<char>>,
    // indexes to move
    instructions: Vec<[usize; 3]>,
}

impl From<Input> for Procedure {
    fn from(value: Input) -> Self {
        let mut lines = value.lines().into_iter();

        let mut table = vec![];
        let mut line = lines.next().unwrap().unwrap();
        while !line.starts_with(" 1") {
            let chars = line.chars().skip(1);
            for (i, c) in chars.step_by(4).enumerate() {
                if c != ' ' {
                    let diff: isize = i as isize + 1 - table.len() as isize;
                    if diff > 0 {
                        for _ in 0..diff {
                            table.push(VecDeque::new());
                        }
                    }
                    table[i].push_front(c)
                }
            }

            line = lines.next().unwrap().unwrap();
        }

        let mut instructions = vec![];
        for line in lines.skip(1) {
            let line = line.unwrap();

            let instruction: [usize; 3] = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|c| c.parse::<usize>().expect("Invalid number"))
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            instructions.push(instruction);
        }

        Procedure {
            table,
            instructions,
        }
    }
}

// 940us
fn part1(input: Input) -> String {
    let mut procedure = Procedure::from(input);

    for [n, from, to] in procedure.instructions.into_iter() {
        for _ in 0..n {
            let c = procedure.table[from - 1].pop_back().unwrap();
            procedure.table[to - 1].push_back(c)
        }
    }

    let mut result = String::new();
    for mut row in procedure.table {
        result.push(row.pop_back().unwrap());
    }

    return result;
}

// 1.1ms
fn part2(input: Input) -> String {
    let mut procedure = Procedure::from(input);

    for [n, from, to] in procedure.instructions.into_iter() {
        let mut crates = VecDeque::new();
        for _ in 0..n {
            let c = procedure.table[from - 1].pop_back().unwrap();
            crates.push_front(c);
        }
        procedure.table[to - 1].append(&mut crates);
    }

    let mut result = String::new();
    for mut row in procedure.table {
        result.push(row.pop_back().unwrap());
    }

    return result;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
