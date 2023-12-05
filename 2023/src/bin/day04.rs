use std::collections::VecDeque;

use advent_of_code_2023::Aoc;

/// 165ns
fn part1(input: String) -> u32 {
    let mut sum = 0;
    for l in input.lines() {
        let mut parts = l.split("|");
        let winning_string = parts.next().unwrap().split(":").skip(1).next().unwrap();
        let winning: Vec<u32> = winning_string[0..winning_string.len() - 1]
            .split(" ")
            .filter_map(|s| {
                if s.len() == 0 {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            })
            .collect();

        let mut found: u32 = 0;
        let card = parts.next().unwrap();
        for n in card[1..card.len()].split(" ") {
            if n.len() == 0 {
                continue;
            }
            let n: u32 = n.parse().unwrap();
            if winning.contains(&n) {
                found += 1;
            }
        }
        if found > 0 {
            sum += 2_u32.pow(found - 1);
        }
    }
    return sum;
}

/// 200ns
fn part2(input: String) -> u32 {
    let mut cards = 0;
    let mut repeat = VecDeque::new();
    for l in input.lines() {
        let mut parts = l.split("|");
        let winning_string = parts.next().unwrap().split(":").skip(1).next().unwrap();
        let winning: Vec<u32> = winning_string[0..winning_string.len() - 1]
            .split(" ")
            .filter_map(|s| {
                if s.len() == 0 {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            })
            .collect();

        let mut found: u32 = 0;
        let card = parts.next().unwrap();
        for n in card[1..card.len()].split(" ") {
            if n.len() == 0 {
                continue;
            }

            let n: u32 = n.parse().unwrap();
            if winning.contains(&n) {
                found += 1;
            }
        }

        let cloned = repeat.pop_front().unwrap_or(0);
        cards += 1 + cloned;
        // add clones
        if found > 0 {
            for i in 0..found {
                match repeat.get_mut(i as usize) {
                    Some(clones) => *clones += cloned + 1,
                    None => repeat.push_back(cloned + 1),
                }
            }
        }
    }
    return cards;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
