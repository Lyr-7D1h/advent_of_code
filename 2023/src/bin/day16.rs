use std::collections::HashMap;

use advent_of_code_2023::Aoc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    /// get direction to the left of current
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    /// get direction to the right of current
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn recurse(
    (mut x, mut y): (usize, usize),
    direction: Direction,
    map: &Vec<Vec<char>>,
    energized: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    // check for cycles and energize
    if let Some(p) = energized.get_mut(&(x, y)) {
        if p.contains(&direction) {
            return;
        }
        p.push(direction)
    } else {
        energized.insert((x, y), vec![direction]);
    }

    let new_dir = match map[y][x] {
        '.' => vec![direction],
        '|' if (direction == Direction::North || direction == Direction::South) => vec![direction],
        '|' => vec![Direction::North, Direction::South],
        '-' if (direction == Direction::East || direction == Direction::West) => vec![direction],
        '-' => vec![Direction::East, Direction::West],
        '\\' if direction == Direction::North || direction == Direction::South => {
            vec![direction.left()]
        }
        '\\' => vec![direction.right()],
        '/' if direction == Direction::East || direction == Direction::West => {
            vec![direction.left()]
        }
        '/' => vec![direction.right()],
        c => panic!("{c}"),
    };

    for d in new_dir {
        // update pos, returning on boundries
        match d {
            Direction::North if y == 0 => continue,
            Direction::North => y -= 1,
            Direction::South if y == map.len() - 1 => continue,
            Direction::South => y += 1,
            Direction::West if x == 0 => continue,
            Direction::West => x -= 1,
            Direction::East if x == map[0].len() - 1 => continue,
            Direction::East => x += 1,
        }
        recurse((x, y), d, map, energized);
    }
}

/// 1.846206ms
fn part1(input: String) -> usize {
    let map = input.lines().map(|l| l.chars().collect()).collect();

    let mut energized = HashMap::new();
    recurse((0, 0), Direction::East, &map, &mut energized);

    energized.len()
}

/// simple brute force 469.384817ms
fn part2(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut max = 0;
    for x in 0..map[0].len() {
        let mut energized = HashMap::new();
        recurse((x, 0), Direction::South, &map, &mut energized);
        max = max.max(energized.len());

        let mut energized = HashMap::new();
        recurse((x, map.len() - 1), Direction::North, &map, &mut energized);
        max = max.max(energized.len());
    }

    for y in 0..map.len() {
        let mut energized = HashMap::new();
        recurse((0, y), Direction::East, &map, &mut energized);
        max = max.max(energized.len());

        let mut energized = HashMap::new();
        recurse((map[0].len() - 1, y), Direction::West, &map, &mut energized);
        max = max.max(energized.len());
    }

    max
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
