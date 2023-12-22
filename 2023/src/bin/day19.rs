use std::{cmp::Ordering, collections::HashMap};

use advent_of_code_2023::Aoc;

type Rule<'n> = (Option<(usize, Ordering, u64)>, &'n str);

/// 267.803µs
fn part1(input: String) -> u64 {
    let mut parts = input.split("\n\n");
    let mut ins: HashMap<&str, Vec<Rule>> = HashMap::new();
    for l in parts.next().unwrap().lines() {
        let mut parts = l.split("{");
        let name = parts.next().unwrap();
        let rules = parts.next().unwrap();
        let rules: Vec<Rule> = rules[0..rules.len() - 1]
            .split(",")
            .map(|rule| {
                let parts: Vec<&str> = rule.split(":").collect();
                if parts.len() == 1 {
                    (None, parts[0])
                } else {
                    let mut chars = parts[0].chars();
                    let part = match chars.next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!(),
                    };
                    let comparison = match chars.next().unwrap() {
                        '>' => Ordering::Greater,
                        '<' => Ordering::Less,
                        _ => panic!(),
                    };
                    let value: u64 = parts[0][2..parts[0].len()].parse().unwrap();
                    (Some((part, comparison, value)), parts[1])
                }
            })
            .collect();
        ins.insert(name, rules);
    }

    let mut accepted: u64 = 0;
    for l in parts.next().unwrap().lines() {
        let mut item = [0; 4];
        for (i, a) in l[1..l.len() - 1].split(",").enumerate() {
            item[i] = a[2..a.len()].parse().unwrap()
        }

        let mut instruction = "in";
        'outer: loop {
            for (comparison, name) in ins[instruction].iter() {
                match comparison {
                    None => {
                        if *name == "R" {
                            break 'outer;
                        }
                        if *name == "A" {
                            accepted += item.into_iter().sum::<u64>();
                            break 'outer;
                        }
                        instruction = name;
                        break;
                    }
                    Some((part, ordering, value)) => {
                        if item[*part].cmp(value) == *ordering {
                            if *name == "R" {
                                break 'outer;
                            }
                            if *name == "A" {
                                accepted += item.into_iter().sum::<u64>();
                                break 'outer;
                            }
                            instruction = name;
                            break;
                        }
                    }
                }
            }
        }
    }

    accepted
}

fn recurse(
    instruction: &str,
    ins: &HashMap<&str, Vec<Rule>>,
    mut boundries: [(u64, u64); 4],
) -> u64 {
    ins[instruction].iter().fold(0, |a, (comparison, name)| {
        // update boundries to current comparison
        let mut new_boundries = boundries.clone();
        if let Some((part, ordering, value)) = comparison {
            match ordering {
                // update upper boundry
                Ordering::Less => new_boundries[*part].1 = *value - 1,
                // update lower boundry
                Ordering::Greater => new_boundries[*part].0 = *value + 1,
                _ => panic!(),
            }
            // if boundry is invalid skip
            if new_boundries[*part].1 < new_boundries[*part].0 {
                return a;
            }
            // following rules have different boundries
            match ordering {
                Ordering::Less => boundries[*part].0 = *value,
                Ordering::Greater => boundries[*part].1 = *value,
                _ => panic!(),
            }
        };

        // accepted count all possibilities with boundries
        if *name == "A" {
            return a + new_boundries
                .into_iter()
                .fold(1, |a, (min, max)| a * (max - min + 1));
        }
        // rejected
        if *name == "R" {
            return a;
        }

        return a + recurse(name, ins, new_boundries);
    })
}

/// 241.52µs
fn part2(input: String) -> u64 {
    let mut parts = input.split("\n\n");
    // NOTE: performance can be increase by switching to vectors
    let mut ins: HashMap<&str, Vec<Rule>> = HashMap::new();
    for l in parts.next().unwrap().lines() {
        let mut parts = l.split("{");
        let name = parts.next().unwrap();
        let rules = parts.next().unwrap();
        let rules: Vec<Rule> = rules[0..rules.len() - 1]
            .split(",")
            .map(|rule| {
                let parts: Vec<&str> = rule.split(":").collect();
                if parts.len() == 1 {
                    (None, parts[0])
                } else {
                    let mut chars = parts[0].chars();
                    let part = match chars.next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!(),
                    };
                    let comparison = match chars.next().unwrap() {
                        '>' => Ordering::Greater,
                        '<' => Ordering::Less,
                        _ => panic!(),
                    };
                    let value: u64 = parts[0][2..parts[0].len()].parse().unwrap();
                    (Some((part, comparison, value)), parts[1])
                }
            })
            .collect();
        ins.insert(name, rules);
    }

    return recurse("in", &ins, [(1, 4000); 4]);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
