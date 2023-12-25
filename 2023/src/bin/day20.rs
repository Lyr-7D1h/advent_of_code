use std::collections::{HashMap, VecDeque};

use advent_of_code_2023::Aoc;

#[derive(Debug, Clone)]
enum Module {
    Conjunction { cache: HashMap<usize, bool> },
    FlipFlop { state: bool },
}

impl Module {
    fn receive(&mut self, pulse: bool, index: usize) -> bool {
        match self {
            Module::Conjunction { cache } => {
                cache.insert(index, false);
                if cache.values().all(|p| *p) {
                    true
                } else {
                    false
                }
            }
            Module::FlipFlop { state } => {
                if pulse == false {
                    *state = !*state;
                    if *state {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    pulse
                }
            }
        }
    }
}

fn part1(input: String) -> u32 {
    let mut named_map = HashMap::new();
    let mut start = vec![];
    let mut i = 0;
    for l in input.lines() {
        let mut parts = l.split(" -> ");
        let name = parts.next().unwrap();
        let to: Vec<&str> = parts.next().unwrap().split(", ").collect();
        if name == "broadcaster" {
            start = to;
            continue;
        }
        match name.chars().next().unwrap() {
            '&' => named_map.insert(
                &name[1..name.len()],
                (
                    i,
                    Module::Conjunction {
                        cache: HashMap::new(),
                    },
                    to,
                ),
            ),
            '%' => named_map.insert(
                &name[1..name.len()],
                (i, Module::FlipFlop { state: false }, to),
            ),
            _ => panic!(),
        };
        i += 1;
    }
    let start: Vec<usize> = start
        .into_iter()
        .map(|name| named_map.get(name).unwrap().0)
        .collect();
    let mut map = vec![(Module::FlipFlop { state: false }, vec![]); named_map.len()];
    for (i, module, to) in named_map.values() {
        map[*i] = (
            module.clone(),
            to.into_iter()
                .map(|name| named_map.get(name).unwrap().0)
                .collect(),
        )
    }

    let mut pulses_map = vec![vec![]; map.len()];
    for i in start {
        pulses_map[i].push(false);
    }
    for _ in 0..1000 {
        let mut new = vec![vec![]; map.len()];
        for (i, pulses) in pulses_map.into_iter().enumerate() {
            if pulses.len() == 0 {
                continue;
            }
            let (module, dest) = &mut map[i];

            for p in pulses {
                for i in dest.iter() {
                    let pulse = module.receive(p, *i);
                    new[*i].push(pulse);
                }
            }
        }
        pulses_map = new;
    }

    println!("{map:?}");
    todo!()
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
