use std::collections::{BinaryHeap, HashSet};

use advent_of_code_2023::Aoc;

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    cost: u32,
    // history: [Direction; 4],
    horizontal: bool,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra without storing all distances and adding to queue in a grid like pattern
/// *---*
/// |+++|
/// |+++|
/// |+++|
/// 18.413094ms
fn part1(input: String) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        horizontal: false,
        cost: 0,
    });
    heap.push(State {
        pos: (0, 0),
        horizontal: true,
        cost: 0,
    });

    let mut seen = HashSet::new();

    while let Some(State {
        pos: (x, y),
        cost,
        horizontal,
    }) = heap.pop()
    {
        // return shortest path if at end goal
        if x == map[0].len() - 1 && y == map.len() - 1 {
            return cost;
        }

        if seen.contains(&((x, y), horizontal)) {
            continue;
        }
        seen.insert(((x, y), horizontal));

        for s in [true, false] {
            let mut cost = cost;
            for i in 1..4 {
                let (x, y) = if horizontal {
                    if s {
                        (x + i, y)
                    } else {
                        if i > x {
                            break;
                        }
                        (x - i, y)
                    }
                } else {
                    if s {
                        (x, y + i)
                    } else {
                        if i > y {
                            break;
                        }
                        (x, y - i)
                    }
                };

                if x > map[0].len() - 1 || y > map.len() - 1 {
                    break;
                }

                cost += map[y][x];

                if seen.contains(&((x, y), !horizontal)) {
                    continue;
                }

                heap.push(State {
                    cost,
                    horizontal: !horizontal,
                    pos: (x, y),
                })
            }
        }
    }

    panic!()
}

/// same as part 1 with changed limits
/// 42.033051ms
fn part2(input: String) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        horizontal: false,
        cost: 0,
    });
    heap.push(State {
        pos: (0, 0),
        horizontal: true,
        cost: 0,
    });

    let mut seen = HashSet::new();

    while let Some(State {
        pos: (x, y),
        cost,
        horizontal,
    }) = heap.pop()
    {
        // return shortest path if at end goal
        if x == map[0].len() - 1 && y == map.len() - 1 {
            return cost;
        }

        if seen.contains(&((x, y), horizontal)) {
            continue;
        }
        seen.insert(((x, y), horizontal));

        for s in [true, false] {
            let mut cost = cost;
            for i in 1..11 {
                let (x, y) = if horizontal {
                    if s {
                        (x + i, y)
                    } else {
                        if i > x {
                            break;
                        }
                        (x - i, y)
                    }
                } else {
                    if s {
                        (x, y + i)
                    } else {
                        if i > y {
                            break;
                        }
                        (x, y - i)
                    }
                };

                if x > map[0].len() - 1 || y > map.len() - 1 {
                    break;
                }

                cost += map[y][x];

                if i < 4 {
                    continue;
                }

                if seen.contains(&((x, y), !horizontal)) {
                    continue;
                }

                heap.push(State {
                    cost,
                    horizontal: !horizontal,
                    pos: (x, y),
                })
            }
        }
    }

    panic!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
