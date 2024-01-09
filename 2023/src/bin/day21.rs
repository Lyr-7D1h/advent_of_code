use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2023::Aoc;

enum Tile {
    Rock,
    Garden,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Rock,
            'S' | '.' => Tile::Garden,
            _ => panic!(),
        }
    }
}

/// Average Duration: 8.667248ms
fn part1(input: String) -> usize {
    let mut start = (0, 0);
    let map: Vec<Vec<Tile>> = input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    Tile::from(c)
                })
                .collect()
        })
        .collect();

    let mut queue = HashSet::new();
    queue.insert(start);
    for _ in 0..64 {
        let mut new = HashSet::new();
        for (x, y) in queue.into_iter() {
            let np = match (x, y) {
                (0, 0) => vec![(x + 1, y), (x, y + 1)],
                (0, y) => vec![(x + 1, y), (x, y + 1), (x, y - 1)],
                (x, 0) => vec![(x + 1, y), (x, y + 1), (x - 1, y)],
                _ => vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)],
            };
            for (x, y) in np {
                if x > map[0].len() - 1 || y > map.len() - 1 {
                    continue;
                }
                if let Tile::Garden = map[y][x] {
                    new.insert((x, y));
                }
            }
        }
        queue = new;
    }

    return queue.len();
}

/// hard one. I saw the parity and that by knowing where you enter into a new chunk/map you can now
/// the parity of the map. I did not see that 26501365 is an indicator of how many chunks there
/// are. In the end used
/// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21 for
/// my sol
///
/// Average Duration: 2.967714ms
fn part2(input: String) -> usize {
    let mut start = (0, 0);
    let map: Vec<Vec<Tile>> = input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    Tile::from(c)
                })
                .collect()
        })
        .collect();

    // get all distances from start
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));
    while let Some(((x, y), d)) = queue.pop_front() {
        if distances.contains_key(&(x, y)) {
            continue;
        }
        distances.insert((x, y), d);
        let np = match (x, y) {
            (0, 0) => vec![(x + 1, y), (x, y + 1)],
            (0, y) => vec![(x + 1, y), (x, y + 1), (x, y - 1)],
            (x, 0) => vec![(x + 1, y), (x, y + 1), (x - 1, y)],
            _ => vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)],
        };
        for (x, y) in np {
            if x > map[0].len() - 1 || y > map.len() - 1 {
                continue;
            }
            if let Tile::Garden = map[y][x] {
                queue.push_back(((x, y), d + 1));
            }
        }
    }

    let neven_corners = distances
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let nodd_corners = distances
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();
    let dim = map.len();
    let n = ((26501365 - (dim / 2)) / dim) as usize;
    let even = n * n;
    let odd = (n + 1) * (n + 1);

    return odd * distances.values().filter(|v| **v % 2 == 1).count()
        + even * distances.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * nodd_corners)
        + (n * neven_corners);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
