use advent_of_code_2022::{Aoc, Input};
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

enum Instruction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Instruction::Right,
            "L" => Instruction::Left,
            "U" => Instruction::Up,
            "D" => Instruction::Down,
            c => panic!("Invalid instruction: {c}"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn attached(a: &Position, b: &Position) -> bool {
    let distance = (a.x - b.x).pow(2) + (a.y - b.y).pow(2);
    if distance > 1 {
        return false;
    }
    return true;
}

fn part1(input: Input) -> usize {
    let mut set = HashSet::new();

    let mut tail = Position { x: 0, y: 0 };
    let mut head = Position { x: 0, y: 0 };

    for line in input.lines() {
        println!("{set:?}");

        let line = line.unwrap();
        let mut split = line.split(" ");
        let instruction = Instruction::from(split.next().unwrap());
        let times = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..times {
            match instruction {
                Instruction::Right => {
                    head.x += 1;
                    if !attached(&tail, &head) {
                        tail.x += 1;
                        tail.y = head.y;
                    }
                }
                Instruction::Left => {
                    head.x -= 1;
                }
                Instruction::Up => {
                    head.y -= 1;
                }
                Instruction::Down => {
                    head.y += 1;
                }
            }
            set.insert(&tail);
        }
    }

    return set.len();
}

fn part2(input: Input) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
