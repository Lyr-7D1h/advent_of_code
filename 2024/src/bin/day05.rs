use std::{cmp::Ordering, collections::HashMap};

use advent_of_code_2023::Aoc;

// Average Duration: 343.531µs
fn part1(input: String) -> usize {
    let lines = input.lines();
    let mut orders = HashMap::new();
    let mut i = 0;
    let mut sum = 0;
    for l in lines.clone() {
        i += 1;
        if l.is_empty() {
            break;
        }
        let mut p = l.split("|");
        let a: usize = p.next().unwrap().parse().unwrap();
        let b: usize = p.next().unwrap().parse().unwrap();
        orders.entry(a).or_insert(vec![]).push(b);
    }
    'n: for l in lines.skip(i) {
        let update: Vec<usize> = l.split(",").map(|s| s.parse().unwrap()).collect();

        let mut sorted_update = update.clone();
        sorted_update.sort_by(|a, b| {
            if let Some(o) = orders.get(a) {
                if o.contains(b) {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        });

        for i in 0..update.len() {
            if update[i] != sorted_update[i] {
                continue 'n;
            }
        }
        let mid = update[update.len() / 2];
        sum += mid;
    }

    return sum;
}

/// Average Duration: 348.406µs
fn part2(input: String) -> usize {
    let lines = input.lines();
    let mut orders = HashMap::new();
    let mut i = 0;
    let mut sum = 0;
    for l in lines.clone() {
        i += 1;
        if l.is_empty() {
            break;
        }
        let mut p = l.split("|");
        let a: usize = p.next().unwrap().parse().unwrap();
        let b: usize = p.next().unwrap().parse().unwrap();
        orders.entry(a).or_insert(vec![]).push(b);
    }
    for l in lines.skip(i) {
        let update: Vec<usize> = l.split(",").map(|s| s.parse().unwrap()).collect();

        let mut sorted_update = update.clone();
        sorted_update.sort_by(|a, b| {
            if let Some(o) = orders.get(a) {
                if o.contains(b) {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        });

        let is_valid = update
            .into_iter()
            .enumerate()
            .all(|(index, i)| i == sorted_update[index]);
        if !is_valid {
            let mid = sorted_update[sorted_update.len() / 2];
            sum += mid;
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
