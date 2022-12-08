use advent_of_code_2022::{Aoc, Input};
use std::{io::BufRead};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Node {
    size: u64,
    parent: Option<usize>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

/// A memory arena (https://en.wikipedia.org/wiki/Region-based_memory_management)
struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    fn new() -> Arena {
        Arena { nodes: vec![] }
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn get(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    fn traverse_up<F: FnMut(&mut Node) -> ()>(&mut self, mut start: usize, mut f: F) {
        loop {
            let node = &mut self.nodes[start];
            f(node);

            match node.parent {
                Some(parent) => start = parent,
                None => break,
            }
        }
    }
}

fn parse_to_arena(input: Input) -> Arena {
    let mut arena = Arena::new();

    let mut lines = input.lines().skip(1).peekable();

    // Assuming you start with /
    let mut current = arena.add_node(Node {
        size: 0,
        parent: None,
    });

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("$ cd") {
            let name = &line[5..line.len()];
            if name == ".." {
                current = arena.get(current).parent.unwrap();
            } else {
                current = arena.add_node(Node {
                    size: 0,
                    parent: Some(current),
                });
            }
        } else if line.starts_with("$ ls") {
            let mut sum = 0;
            // read until new command
            loop {
                match lines.peek() {
                    None => {
                        if sum > 0 {
                            arena.traverse_up(current, |n| n.size += sum);
                        }
                        break;
                    }
                    Some(Ok(line)) if { line.starts_with("$") } => {
                        // Break if next is a command
                        if sum > 0 {
                            arena.traverse_up(current, |n| n.size += sum);
                        }
                        break;
                    }
                    _ => {}
                }

                let line = lines.next().unwrap().unwrap();
                // ignore directories
                if !line.starts_with("dir") {
                    sum += line.split(" ").nth(0).unwrap().parse::<u64>().unwrap();
                }
            }
        }
    }

    return arena;
}

// 900ns
fn part1(input: Input) -> u64 {
    let arena = parse_to_arena(input);

    return arena
        .nodes
        .iter()
        .filter(|n| n.size < 100000)
        .map(|n| n.size)
        .sum();
}

// 900ns
fn part2(input: Input) -> u64 {
    let arena = parse_to_arena(input);

    let root = arena.get(0);
    let limit = 30000000 - (70000000 - root.size);
    return arena
        .nodes
        .iter()
        .filter(|n| n.size > limit)
        .min()
        .unwrap()
        .size;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
