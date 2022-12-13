use advent_of_code_2022::{Aoc, Input};
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

#[derive(Debug)]
struct Node {
    value: u8,
    neighbors: Vec<usize>,
}

#[derive(Debug)]
struct HeightMap {
    nodes: Vec<Node>,
    start_index: usize,
    end_index: usize,
}

impl HeightMap {
    fn shortest_distance(&self, start_index: usize) -> usize {
        let mut visited_set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_index, 0));

        while let Some((index, step)) = queue.pop_front() {
            if visited_set.get(&index).is_none() {
                visited_set.insert(index);
                let node = &self.nodes[index];
                if index == self.end_index {
                    return step;
                }
                for neighbor in node.neighbors.iter() {
                    if visited_set.get(neighbor).is_none() {
                        queue.push_back((*neighbor, step + 1));
                    }
                }
            }
        }

        return 0;
    }

    /// returns the shortest distance from any starting point
    fn shortest_distance_any_starting_point(&self) -> usize {
        let mut shortest_distance = usize::MAX;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.value == 0 {
                let distance = self.shortest_distance(i);
                if distance != 0 && distance < shortest_distance {
                    shortest_distance = distance;
                }
            }
        }
        return shortest_distance;
    }
}

impl From<Input> for HeightMap {
    fn from(value: Input) -> Self {
        let mut start_position = (0, 0);
        let mut end_position = (0, 0);
        // height map with raw values and x,y coordinates
        let raw_value_map: Vec<Vec<u8>> = value
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start_position = (x, y);
                            0
                        } else if c == 'E' {
                            end_position = (x, y);
                            25
                        } else {
                            c as u8 - b'a'
                        }
                    })
                    .collect()
            })
            .collect();

        let mut nodes = vec![];
        let y_max = raw_value_map.len();
        let x_max = raw_value_map[0].len();

        for y in 0..y_max {
            for x in 0..x_max {
                let mut neighbors = vec![];
                let value = raw_value_map[y][x];

                // up
                if y > 0 {
                    let cmp = raw_value_map[y - 1][x];
                    if cmp <= value + 1 {
                        neighbors.push(y * x_max + x - x_max);
                    }
                }
                // down
                if y < y_max - 1 {
                    let cmp = raw_value_map[y + 1][x];
                    if cmp <= value + 1 {
                        neighbors.push(y * x_max + x + x_max);
                    }
                }
                // left
                if x > 0 {
                    let cmp = raw_value_map[y][x - 1];
                    if cmp <= value + 1 {
                        neighbors.push(y * x_max + x - 1);
                    }
                }
                // right
                if x < x_max - 1 {
                    let cmp = raw_value_map[y][x + 1];
                    if cmp <= value + 1 {
                        neighbors.push(y * x_max + x + 1);
                    }
                }

                nodes.push(Node {
                    value, // reduce value to 0-26
                    neighbors,
                })
            }
        }

        HeightMap {
            nodes,
            start_index: start_position.1 * x_max + start_position.0,
            end_index: end_position.1 * x_max + end_position.0,
        }
    }
}

// 15ms
fn part1(input: Input) -> usize {
    let map = HeightMap::from(input);
    return map.shortest_distance(map.start_index);
}

// 5s
fn part2(input: Input) -> usize {
    let map = HeightMap::from(input);
    return map.shortest_distance_any_starting_point();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
