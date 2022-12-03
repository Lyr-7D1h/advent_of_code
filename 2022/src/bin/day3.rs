use advent_of_code_2022::{Aoc, Input};
use std::{collections::VecDeque, io::BufRead};

fn get_score(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u8 - 64 + 26) as u32
    } else {
        (c as u8 - 96) as u32
    }
}

// 720us
fn part1(input: Input) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (comp1, comp2) = line.split_at(line.len() / 2);
        for c in comp1.chars() {
            if comp2.contains(c) {
                total += get_score(c);
                break;
            }
        }
    }
    return total;
}

// 520us
fn part2(input: Input) -> u32 {
    let mut total: u32 = 0;
    let mut groups: VecDeque<String> = VecDeque::new();
    for line in input.lines() {
        let line = line.unwrap();

        groups.push_back(line);

        if groups.len() == 3 {
            let start_group = groups.pop_front().unwrap();
            let start_group = start_group.chars();
            for c in start_group {
                if groups[0].contains(c) && groups[1].contains(c) {
                    total += get_score(c);
                    break;
                }
            }
            groups.clear();
        }
    }

    return total;
}

// each bit represents a character in the string
// be  a b c d e
//     0 1 0 0 1
fn to_bits(s: String) -> u64 {
    let mut bits = 0;
    for c in s.chars() {
        if c.is_lowercase() {
            bits |= 1 << (1 + c as u8 - b'a')
        } else if c.is_uppercase() {
            bits |= 1 << (27 + c as u8 - b'A')
        }
    }

    return bits;
}

// 320us
fn part2_bitwise(input: Input) -> u32 {
    let mut total = 0;

    let mut groups = [0; 3];
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();

        groups[i % 3] = to_bits(line);

        if groups.len() == 3 {
            let r = groups.into_iter().reduce(|a, b| a & b).unwrap();
            total += r.ilog2();
        }
    }

    return total;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.part("two_bitwise", part2_bitwise);
    aoc.run();
}
