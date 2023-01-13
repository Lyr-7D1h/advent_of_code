use advent_of_code_2022::{Aoc, Input};
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone, Copy)]
struct BitArray {
    mask: u64,
    value: u64, // 60 valves input
}

impl BitArray {
    fn new(length: usize) -> BitArray {
        BitArray {
            value: 0,
            mask: (1 << length) - 1,
        }
    }
    fn set(&mut self, index: usize) {
        self.value |= 1 << index;
    }
    fn all_set(&self) -> bool {
        self.value & self.mask == self.mask
    }
    fn get(&self, index: usize) -> bool {
        (self.value & 1 << index) > 0
    }
}

#[derive(Debug, Eq, Clone, Default)]
struct Valve {
    rate: usize,
    tunnels: Vec<usize>,
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rate.cmp(&other.rate)
    }
}

#[derive(Debug)]
struct Cave {
    /// the starting valve which might not exist in a minimized Cave
    start_index: usize,
    valves: Vec<Valve>,
}

impl Cave {
    fn get(&self, index: usize) -> &Valve {
        &self.valves[index]
    }

    fn branch_and_bound_recurs<F: Fn(usize, usize, &BitArray) -> usize>(
        &self,
        upper_bound: &F,
        current_lowerbound: &mut usize,
        index: [usize; 2],
        score: usize,
        mut minutes: [usize; 2],
        open_valves: BitArray,
    ) {
        minutes[0] -= 1;
        if minutes[0] == 0 || open_valves.all_set() {
            if score > *current_lowerbound {
                *current_lowerbound = score;
                // println!("{current_lowerbound}");
            }
            return;
        }

        let valve = self.get(index[0]);

        let bound = upper_bound(score, minutes[0], &open_valves);

        // open current valve if it has a rate and hasn't been opened already
        if open_valves.get(index[0]) == false && valve.rate > 0 {
            let mut minutes = minutes.clone();
            let mut index = index.clone();
            let mut score = score.clone();
            score += minutes[0] * valve.rate;
            let mut open_valves = open_valves.clone();
            open_valves.set(index[0]);
            if minutes[1] > minutes[0] {
                minutes.swap(1, 0);
                index.swap(1, 0);
            }

            if bound > *current_lowerbound {
                self.branch_and_bound_recurs(
                    upper_bound,
                    current_lowerbound,
                    index,
                    score,
                    minutes,
                    open_valves,
                );
            }
        }

        // go to all the tunnels
        for t in valve.tunnels.iter() {
            if bound > *current_lowerbound {
                let mut index = [*t, index[1]];
                let mut minutes = minutes.clone();
                if minutes[1] > minutes[0] {
                    minutes.swap(1, 0);
                    index.swap(1, 0);
                }
                self.branch_and_bound_recurs(
                    upper_bound,
                    current_lowerbound,
                    index,
                    score,
                    minutes,
                    open_valves,
                );
            }
        }
    }

    /// https://en.wikipedia.org/wiki/Branch_and_bound
    /// Without an heuristic having a dfs gives faster solutions hence faster result
    fn branch_and_bound_dfs(&self, minutes: [usize; 2]) -> usize {
        // valves sorted by flow rate and empty flow rates filtered
        let mut sorted_valves: Vec<(usize, &Valve)> = self
            .valves
            .iter()
            .filter(|v| v.rate != 0)
            .enumerate()
            .collect();
        sorted_valves.sort_by(|(_, a), (_, b)| b.cmp(a));
        let sorted_valves = &sorted_valves;

        let upper_bound = |score: usize, mut minutes: usize, open_valves: &BitArray| {
            let mut upper_bound = score;

            let mut svi = 0;
            while let Some((i, valve)) = sorted_valves.get(svi) {
                if minutes <= 0 {
                    break;
                }

                // if current valve is open skip to next one
                if open_valves.get(*i) {
                    svi += 1;
                    continue;
                }

                minutes -= 1;
                upper_bound += (minutes) as usize * valve.rate;
                if minutes <= 0 {
                    break;
                }
                minutes -= 1;
            }

            // println!("{score} {minutes} {open_valves:?} {upper_bound}");
            return upper_bound;
        };

        let current_lowerbound = &mut 0;

        self.branch_and_bound_recurs(
            &upper_bound,
            current_lowerbound,
            [self.start_index, self.start_index], // FIXME should be index
            0,
            minutes,
            BitArray::new(self.valves.len()),
        );

        return *current_lowerbound;
    }
}

impl From<Input> for Cave {
    fn from(value: Input) -> Self {
        let map: HashMap<String, (usize, usize, Vec<String>)> = value
            .lines()
            .enumerate()
            .map(|(index, l)| {
                let l = l.unwrap();
                let mut split = l.split(" ").skip(1);
                let name = split.next().unwrap().to_string();
                let mut split = split.skip(2);
                let rate = split
                    .next()
                    .unwrap()
                    .replace("rate=", "")
                    .replace(";", "")
                    .parse()
                    .unwrap();

                let mut split = split.skip(4);
                let mut tunnels = vec![];
                while let Some(tunnel) = split.next() {
                    tunnels.push(tunnel.replace(",", ""))
                }

                (name, (index, rate, tunnels))
            })
            .collect();

        let mut valves: Vec<Valve> = vec![Valve::default(); map.len()];

        let mut start_index = 0;
        for (name, (index, rate, tunnels)) in map.iter() {
            let tunnels = tunnels
                .iter()
                .map(|tunnel| map.get(tunnel).unwrap().0)
                .collect();

            let node = Valve {
                rate: *rate,
                tunnels,
            };

            if name == "AA" {
                start_index = index.clone();
            }

            valves[*index] = node;
        }

        Cave {
            valves,
            start_index,
        }
    }
}

/// 83ms 
fn part1(input: Input) -> usize {
    return Cave::from(input).branch_and_bound_dfs([30, 0]);
}

/// 500ms
fn part2(input: Input) -> usize {
    return Cave::from(input).branch_and_bound_dfs([26, 26]);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}

#[test]
fn test_bit_array() {
    let mut ba = BitArray::new(3);
    ba.set(0);
    ba.set(1);
    ba.set(2);
    assert!(ba.all_set() == true);
}
