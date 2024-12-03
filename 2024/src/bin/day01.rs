use std::collections::HashMap;

use advent_of_code_2023::Aoc;

// 880ns
fn part1(input: String) -> u32 {
    let mut a = vec![];
    let mut b = vec![];
    for line in input.lines() {
        let mut p = line.split("   ");
        a.push(p.next().unwrap().parse::<u32>().unwrap());
        b.push(p.next().unwrap().parse::<u32>().unwrap());
    }
    a.sort();
    b.sort();
    return a
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, a)| b[i].abs_diff(a) as u32 + acc);
}

// 990ns
fn part2(input: String) -> u32 {
    let mut set = HashMap::new();
    let mut a = vec![];

    for line in input.lines() {
        let mut p = line.split("   ");
        a.push(p.next().unwrap().parse::<u32>().unwrap());
        let b = p.next().unwrap().parse::<u32>().unwrap();
        set.entry(b).and_modify(|b_i| *b_i += 1).or_insert(1);
    }

    return a
        .into_iter()
        .fold(0, |acc, a| a * set.get(&a).unwrap_or(&0) + acc);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
