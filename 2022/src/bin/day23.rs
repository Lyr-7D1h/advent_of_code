use advent_of_code_2022::{Aoc, Input};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

// #[derive(Debug, Hash, Eq, PartialEq)]
// struct Position {
//     x: isize,
//     y: isize,
// }

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Position = (isize, isize);

struct UpdatePositonEntry {
    old_position: Position,
    is_double: bool,
}

#[derive(Debug)]
struct Map {
    positions: HashSet<Position>,
}

impl Map {
    /// Returns new position with the direction it took
    fn direction(&self, position: &Position, direction_list: &Vec<Direction>) -> Option<Position> {
        let (x, y) = *position;

        let north = (x, y - 1);
        let south = (x, y + 1);
        let east = (x + 1, y);
        let west = (x - 1, y);
        let possible_directions: Vec<&Direction> = direction_list
            .iter()
            .filter(|direction| !match direction {
                Direction::North => {
                    self.positions.contains(&(x - 1, y - 1))
                        || self.positions.contains(&north)
                        || self.positions.contains(&(x + 1, y - 1))
                }
                Direction::South => {
                    self.positions.contains(&(x - 1, y + 1))
                        || self.positions.contains(&south)
                        || self.positions.contains(&(x + 1, y + 1))
                }
                Direction::West => {
                    self.positions.contains(&(x - 1, y - 1))
                        || self.positions.contains(&west)
                        || self.positions.contains(&(x - 1, y + 1))
                }
                Direction::East => {
                    self.positions.contains(&(x + 1, y - 1))
                        || self.positions.contains(&east)
                        || self.positions.contains(&(x + 1, y + 1))
                }
            })
            .collect();

        // if can go all directions don't do anything
        if possible_directions.len() == 4 {
            return None;
        }

        if let Some(first) = possible_directions.first() {
            return Some(match first {
                Direction::North => north,
                Direction::South => south,
                Direction::West => west,
                Direction::East => east,
            });
        }

        return None;
    }

    fn n_rounds(&mut self, n: usize) {
        let mut direction_list = vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        for _ in 0..n {
            let mut new_positions: HashMap<(isize, isize), UpdatePositonEntry> = HashMap::new();

            for pos in self.positions.iter() {
                if let Some(new_position) = self.direction(pos, &direction_list) {
                    if let Some(entry) = new_positions.get_mut(&new_position) {
                        entry.is_double = true;
                        continue;
                    }

                    new_positions.insert(
                        new_position,
                        UpdatePositonEntry {
                            old_position: *pos,
                            is_double: false,
                        },
                    );
                }
            }

            let new_positions: Vec<(Position, UpdatePositonEntry)> = new_positions
                .into_iter()
                .filter(|(_, entry)| !entry.is_double)
                .collect();

            // remove old entries
            for (_, entry) in new_positions.iter() {
                self.positions.remove(&entry.old_position);
            }

            // insert new ones
            for (new_pos, _) in new_positions.into_iter() {
                self.positions.insert(new_pos);
            }

            // update direction list
            let direction = direction_list.remove(0);
            direction_list.push(direction);
        }
    }

    fn rounds_until_end(&mut self) -> usize {
        let mut direction_list = vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        let mut round = 1;
        loop {
            let mut new_positions: HashMap<(isize, isize), UpdatePositonEntry> = HashMap::new();

            for pos in self.positions.iter() {
                if let Some(new_position) = self.direction(pos, &direction_list) {
                    if let Some(entry) = new_positions.get_mut(&new_position) {
                        entry.is_double = true;
                        continue;
                    }

                    new_positions.insert(
                        new_position,
                        UpdatePositonEntry {
                            old_position: *pos,
                            is_double: false,
                        },
                    );
                }
            }

            if new_positions.len() == 0 {
                return round;
            }

            let new_positions: Vec<(Position, UpdatePositonEntry)> = new_positions
                .into_iter()
                .filter(|(_, entry)| !entry.is_double)
                .collect();

            // remove old entries
            for (_, entry) in new_positions.iter() {
                self.positions.remove(&entry.old_position);
            }

            // insert new ones
            for (new_pos, _) in new_positions.into_iter() {
                self.positions.insert(new_pos);
            }

            // update direction list
            let direction = direction_list.remove(0);
            direction_list.push(direction);

            round += 1;
        }
    }

    fn absolutes(&self) -> (isize, isize, isize, isize) {
        let mut x_min = isize::MAX;
        let mut y_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_max = isize::MIN;
        for pos in self.positions.iter() {
            let (x, y) = *pos;
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x;
            }
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }
        (x_min, x_max, y_min, y_max)
    }

    fn value_map(&self) -> Vec<Vec<bool>> {
        let (x_min, x_max, y_min, y_max) = self.absolutes();
        let mut map = vec![vec![false; x_min.abs_diff(x_max) + 1]; y_min.abs_diff(y_max) + 1];

        for pos in self.positions.iter() {
            let (x, y) = *pos;
            map[(y - y_min) as usize][(x - x_min) as usize] = true;
        }

        return map;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = self
            .value_map()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| if *tile { '#' } else { '.' })
                    .collect()
            })
            .collect();
        write!(f, "{}", map.join("\n"))
    }
}

impl From<Input> for Map {
    fn from(value: Input) -> Self {
        let mut elfs = HashSet::new();
        for (y, line) in value.lines().enumerate() {
            let line = line.unwrap();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elfs.insert((x as isize, y as isize));
                }
            }
        }

        Map { positions: elfs }
    }
}

// 6ms
fn part1(input: Input) -> usize {
    let mut map = Map::from(input);

    map.n_rounds(10);

    let length = map.positions.len();
    let (x_min, x_max, y_min, y_max) = map.absolutes();

    return (x_min.abs_diff(x_max) + 1) * (y_min.abs_diff(y_max) + 1) - length;
}

// 780ms
fn part2(input: Input) -> usize {
    let mut map = Map::from(input);

    return map.rounds_until_end();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
