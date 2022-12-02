use std::{collections::BinaryHeap, io::BufRead};

use advent_of_code_2022::{Aoc, Input};

// ~2.5ms
fn part1(input: Input) -> u32 {
    let mut max_calories = 0;
    let mut local_sum = 0;
    for line in input.lines() {
        let line = line.unwrap();

        if line == "" {
            if local_sum > max_calories {
                max_calories = local_sum;
            }
            local_sum = 0;
            continue;
        }

        local_sum += line.parse::<u32>().unwrap();
    }

    return max_calories;
}

// ~3ms
fn part2(input: Input) -> u32 {
    let lines: Vec<String> = input.lines().map(|l| l.unwrap()).collect();

    let mut lines: Vec<u32> = lines
        .split(|l| l == "")
        .map(|lines| {
            lines
                .into_iter()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    lines.sort();
    lines.reverse();

    return lines[..3].into_iter().sum();
}

// ~2.15ms
fn part2_imperative(input: Input) -> u32 {
    let mut max: Vec<u32> = vec![0; 3];
    let mut local_sum = 0;
    for line in input.lines() {
        let line = line.unwrap();

        if line == "" {
            for m in max.iter_mut().take(3) {
                if &local_sum > m {
                    *m = local_sum;
                    break;
                }
            }
            local_sum = 0;
            continue;
        }

        local_sum += line.parse::<u32>().unwrap();
    }

    let sum = max.into_iter().sum::<u32>();

    return sum;
}

// ~2.17
fn part2_binary_heap(input: Input) -> u32 {
    let heap = input
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .split(|l| l == "")
        .map(|l| {
            l.into_iter()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<BinaryHeap<_>>();

    heap.iter().take(3).sum()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.part("two_imperative", part2_imperative);
    aoc.part("two_binary_heap", part2_imperative);
    aoc.run();
}
