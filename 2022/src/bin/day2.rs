use std::{char, io::BufRead};

use advent_of_code_2022::{Aoc, Input};

#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn round(&self, comparison: &RPS) -> u32 {
        match self {
            RPS::Rock => match comparison {
                RPS::Rock => 3,
                RPS::Paper => 6,
                RPS::Scissors => 0,
            },
            RPS::Paper => match comparison {
                RPS::Rock => 0,
                RPS::Paper => 3,
                RPS::Scissors => 6,
            },
            RPS::Scissors => match comparison {
                RPS::Rock => 6,
                RPS::Paper => 0,
                RPS::Scissors => 3,
            },
        }
    }
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            c => panic!("Unknown character {c}"),
        }
    }
}

// ~2.7ms
fn part1(input: Input) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let op = RPS::from(chars.next().unwrap());
        let mut chars = chars.skip(1);
        let cmp = RPS::from(chars.next().unwrap());

        let score = op.round(&cmp);
        total += score + (cmp as u32 + 1);
    }

    return total;
}

fn to_op(c: char) -> u32 {
    match c {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        c => panic!("Unknown char {c}"),
    }
}

// 2.6ms
fn part1_modulo(input: Input) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let op = to_op(chars.next().unwrap());
        let mut chars = chars.skip(1);
        let cmp = to_op(chars.next().unwrap());

        total += 1; // because we're counting from 0
        if (op + 2) % 3 == cmp {
            total += cmp;
        } else if op == cmp {
            total += 3 + cmp;
        } else {
            total += 6 + cmp
        }
    }

    return total;
}

// 2.7ms
fn part2(input: Input) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let op = match chars.next().unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            c => panic!("Unknown char {c}"),
        };
        let mut chars = chars.skip(1);
        let round = chars.next().unwrap();

        let score = match round {
            // lose (0) + losing rps score
            'X' => (op + 2) % 3 + 1,
            // draw + drawing rps score
            'Y' => 3 + op + 1,
            // win + winning rps score
            'Z' => 6 + (op + 1) % 3 + 1,
            c => panic!("Unknown char {c}"),
        };
        total += score;
    }

    return total;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("one_modulo", part1_modulo);
    aoc.part("two", part2);
    aoc.run();
}
