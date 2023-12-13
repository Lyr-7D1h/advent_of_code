use std::collections::HashMap;

use advent_of_code_2023::Aoc;

/// check if entry is valid
fn is_valid(data: &Vec<char>, counts: &Vec<usize>) -> bool {
    // println!("{data:?}");
    let mut count_i = 0;
    let mut i = 0;
    while i < data.len() {
        match data.get(i) {
            Some('#') => {
                let mut count = 1;
                while let Some('#') = data.get(i + count) {
                    count += 1;
                }
                // println!("{count}");
                match counts.get(count_i) {
                    Some(c) if *c != count => return false,
                    Some(_) => {}
                    None => return false,
                }
                // matched group
                count_i += 1;
                i += count + 1;
            }
            Some(_) => i += 1,
            None => return false,
        }
    }

    // too little matches
    if count_i < counts.len() {
        return false;
    }

    true
}

/// brute force all unkowns and check if it is valid
/// 351.990585ms
fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let mut unknowns = vec![];
            let mut data: Vec<char> = parts
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == '?' {
                        unknowns.push(i);
                    }

                    c
                })
                .collect();
            let counts: Vec<usize> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|i| i.parse().unwrap())
                .collect();

            let mut perms = 0;
            for i in 0..2_u32.pow(unknowns.len() as u32) {
                for j in 0..unknowns.len() {
                    if i & 0x1 << j > 0 {
                        data[unknowns[j]] = '#';
                    } else {
                        data[unknowns[j]] = '.';
                    }
                }
                if is_valid(&data, &counts) {
                    perms += 1;
                }
            }

            perms
        })
        .sum()
}

fn generate_damaged(
    data: &[char],
    counts: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let count = counts[0];
    if count > data.len() {
        return 0;
    }
    if data[..count].iter().any(|c| *c == '.') {
        return 0;
    };
    if let Some('#') = data.get(count) {
        return 0;
    }
    return count_possibilities(&data[data.len().min(count + 1)..], &counts[1..], cache);
}

fn count_possibilities(
    data: &[char],
    counts: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let hash = (data.len(), counts.len());
    if let Some(i) = cache.get(&hash) {
        return *i;
    }
    let r = count_possibilities_inner(data, counts, cache);
    cache.insert(hash, r);
    r
}

fn count_possibilities_inner(
    data: &[char],
    counts: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if counts.len() == 0 {
        if data.into_iter().any(|c| *c == '#') {
            return 0;
        }
        return 1;
    }

    if data.len() == 0 {
        return 0;
    }

    if data.len() < counts.iter().sum::<usize>() + counts.len() - 1 {
        return 0;
    }

    match data[0] {
        '?' => {
            let operational = count_possibilities(&data[1..data.len()], counts, cache);
            let damaged = generate_damaged(data, counts, cache);
            return operational + damaged;
        }
        '.' => count_possibilities(&data[1..data.len()], counts, cache),
        '#' => generate_damaged(data, counts, cache),
        _ => panic!(),
    }
}

/// 1.4ms
fn part1_heuristic(input: String) -> usize {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let data: Vec<char> = parts.next().unwrap().chars().collect();
            let counts: Vec<usize> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|i| i.parse().unwrap())
                .collect();

            let mut cache = HashMap::new();
            let p = count_possibilities(&data, &counts, &mut cache);

            p
        })
        .sum()
}

/// 16.052016ms
fn part2(input: String) -> usize {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let mut unknowns = vec![];

            let data_string = parts.next().unwrap();
            let data_string = vec![data_string; 5].join("?");
            let data: Vec<char> = data_string
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == '?' {
                        unknowns.push(i);
                    }

                    c
                })
                .collect();
            let counts: Vec<usize> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>()
                // multiply by 5
                .repeat(5);

            let mut cache = HashMap::new();
            count_possibilities(&data, &counts, &mut cache)
        })
        .sum()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("one_heuristic", part1_heuristic);
    aoc.part("two", part2);
    aoc.run();
}
