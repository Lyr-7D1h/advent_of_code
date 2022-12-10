use advent_of_code_2022::{Aoc, Input};
use std::{collections::HashSet, io::BufRead};

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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn adjacent(a: &Position, b: &Position) -> bool {
    if (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1 {
        return false;
    }
    return true;
}

// 6ms
fn part1(input: Input) -> usize {
    let mut set = HashSet::from([Position { x: 0, y: 0 }]);

    let mut tail = Position { x: 0, y: 0 };
    let mut head = Position { x: 0, y: 0 };

    for line in input.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let instruction = Instruction::from(split.next().unwrap());
        let times = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..times {
            match instruction {
                Instruction::Right => {
                    head.x += 1;
                    if !adjacent(&tail, &head) {
                        tail.x += 1;
                        tail.y = head.y;
                    }
                }
                Instruction::Left => {
                    head.x -= 1;
                    if !adjacent(&tail, &head) {
                        tail.x -= 1;
                        tail.y = head.y;
                    }
                }
                Instruction::Up => {
                    head.y += 1;
                    if !adjacent(&tail, &head) {
                        tail.x = head.x;
                        tail.y += 1;
                    }
                }
                Instruction::Down => {
                    head.y -= 1;
                    if !adjacent(&tail, &head) {
                        tail.x = head.x;
                        tail.y -= 1;
                    }
                }
            }
            set.insert(tail.clone());
        }
    }

    return set.len();
}

/// Helper function to print the rope
fn print_rope(rope: &Vec<Position>) {
    for knot in rope.iter() {
        print!("({}, {})", knot.x, knot.y);
    }
    print!("\n");
}

// 17ms
fn part2(input: Input) -> usize {
    let mut set = HashSet::from([Position { x: 0, y: 0 }]);

    let mut rope = vec![Position { x: 0, y: 0 }; 10];

    for line in input.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let instruction = Instruction::from(split.next().unwrap());
        let times = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..times {
            match instruction {
                Instruction::Right => rope[0].x += 1,
                Instruction::Left => rope[0].x -= 1,
                Instruction::Up => rope[0].y += 1,
                Instruction::Down => rope[0].y -= 1,
            }

            for i in 1..rope.len() {
                let x_diff = rope[i - 1].x - rope[i].x;
                let y_diff = rope[i - 1].y - rope[i].y;

                if x_diff == 0 && y_diff.abs() > 1 {
                    rope[i].y += y_diff.signum();
                } else if y_diff == 0 && x_diff.abs() > 1 {
                    rope[i].x += x_diff.signum();
                } else if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    rope[i].x += x_diff.signum();
                    rope[i].y += y_diff.signum();
                }
            }
            set.insert(rope[9].clone());
        }
    }

    return set.len();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
