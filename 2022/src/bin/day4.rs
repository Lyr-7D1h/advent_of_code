use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

// 1.5ms
fn part1(input: Input) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();

        let line: Vec<u32> = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if (line[0] >= line[2] && line[1] <= line[3]) || (line[2] >= line[0] && line[3] <= line[1])
        {
            total += 1;
        }
    }

    return total;
}

// 1.5 ms
fn part2(input: Input) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let line = line.unwrap();

        let line: Vec<u32> = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        // if length of all ranges does not fit in between lowest min and highest max there is an
        // overlap
        if (line[1] - line[0]) + (line[3] - line[2])
            >= std::cmp::max(line[1], line[3]) - std::cmp::min(line[0], line[2])
        {
            total += 1;
        }
    }

    return total;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
