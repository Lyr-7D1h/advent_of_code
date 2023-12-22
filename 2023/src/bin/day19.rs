use std::{cmp::Ordering, collections::HashMap};

use advent_of_code_2023::Aoc;

type Rule<'n> = (Option<(usize, Ordering, u32)>, &'n str);

/// 267.803µs
fn part1(input: String) -> u32 {
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
                    let value: u32 = parts[0][2..parts[0].len()].parse().unwrap();
                    (Some((part, comparison, value)), parts[1])
                }
            })
            .collect();
        ins.insert(name, rules);
    }

    let mut accepted: u32 = 0;
    for l in parts.next().unwrap().lines() {
        let mut item = [0; 4];
        for (i, a) in l[1..l.len() - 1].split(",").enumerate() {
            item[i] = a[2..a.len()].parse::<u32>().unwrap()
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
                            accepted += item.into_iter().sum::<u32>();
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
                                accepted += item.into_iter().sum::<u32>();
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

fn part2(input: String) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
