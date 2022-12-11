use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

#[derive(Debug)]
enum InstructionType {
    Noop,
    Addx { value: i32 },
}

#[derive(Debug)]
struct Instruction {
    cycle: u8,
    instruction_type: InstructionType,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let mut value = value.split(" ");
        let op = value.next().unwrap();
        match op {
            "noop" => Instruction {
                cycle: 1,
                instruction_type: InstructionType::Noop,
            },
            "addx" => {
                let value = value.next().unwrap().parse().unwrap();
                Instruction {
                    cycle: 2,
                    instruction_type: InstructionType::Addx { value },
                }
            }
            op => panic!("Unknown op: {op}"),
        }
    }
}

// 90ns
fn part1(input: Input) -> i32 {
    let mut total_signal_strength = 0;

    let mut cycle = 0;
    let mut x: i32 = 1;

    let mut cycles = vec![220, 180, 140, 100, 60, 20];

    for line in input.lines() {
        let line = line.unwrap();
        let instruction = Instruction::from(line);

        cycle += instruction.cycle;

        let last_cycle = cycles[cycles.len() - 1];
        if cycle >= last_cycle {
            total_signal_strength += last_cycle as i32 * x;
            cycles.pop();
            if cycles.len() == 0 {
                break;
            }
        }

        match instruction.instruction_type {
            InstructionType::Noop => {}
            InstructionType::Addx { value } => x += value,
        }
    }

    return total_signal_strength;
}

// 113ns
fn part2(input: Input) -> String {
    let mut output = String::from("\n");

    // Sprite of 3 pixels
    let mut crt = 0;
    let mut sprite_mid_point: i32 = 1;

    for line in input.lines() {
        let line = line.unwrap();
        let instruction = Instruction::from(line);

        for _ in 0..instruction.cycle {
            if sprite_mid_point - 1 <= crt && crt <= sprite_mid_point + 1 {
                output += "#";
            } else {
                output += ".";
            }

            if crt == 39 {
                crt = 0;
                output += "\n";
            } else {
                crt += 1;
            }
        }

        match instruction.instruction_type {
            InstructionType::Noop => {}
            InstructionType::Addx { value } => sprite_mid_point += value,
        }
    }
    return output;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
