use std::str::Chars;

use advent_of_code_2023::Aoc;

fn parse_digit(chars: Chars) -> Option<(usize, u32)> {
    let text: String = chars.take_while(|c| c.is_digit(10)).collect();
    if text.len() == 0 {
        return None;
    }
    let digit: u32 = text.parse().ok()?;
    Some((text.len(), digit))
}

fn parse_mul(mut chars: Chars) -> Option<(usize, Instruction)> {
    let mut c = 0;
    if chars.as_str().starts_with("ul(") {
        chars.nth(2);
        if let Some((n, a)) = parse_digit(chars.clone()) {
            c += n;
            chars.nth(n - 1);
            if let Some(',') = chars.next() {
                if let Some((n, b)) = parse_digit(chars.clone()) {
                    c += n;
                    chars.nth(n - 1);
                    if let Some(')') = chars.next() {
                        return Some((3 + 1 + 1 + c, Instruction::Mul(a, b)));
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

/// custom char parser
fn parse(chars: &mut Chars) -> Option<Instruction> {
    while let Some(c) = chars.next() {
        match c {
            'm' => {
                if let Some((len, i)) = parse_mul(chars.clone()) {
                    chars.nth(len - 1);
                    return Some(i);
                }
            }
            'd' => {
                if chars.as_str().starts_with("o()") {
                    chars.nth(2);
                    return Some(Instruction::Do);
                }
                if chars.as_str().starts_with("on't()") {
                    chars.nth(5);
                    return Some(Instruction::Dont);
                }
            }
            _ => {}
        }
    }
    None
}

/// Average Duration: 70µs
fn part1(input: String) -> u32 {
    let mut sum = 0;
    let mut chars = input.chars();
    while let Some(i) = parse(&mut chars) {
        match i {
            Instruction::Mul(a, b) => sum += a * b,
            _ => {}
        }
    }

    return sum;
}

/// Average Duration: 77.884µs
fn part2(input: String) -> u32 {
    let mut on = true;
    let mut sum = 0;
    let mut chars = input.chars();
    while let Some(i) = parse(&mut chars) {
        match i {
            Instruction::Mul(a, b) => {
                if on {
                    sum += a * b;
                }
            }
            Instruction::Do => on = true,
            Instruction::Dont => on = false,
        }
    }

    return sum;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
