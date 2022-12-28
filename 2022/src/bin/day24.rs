use advent_of_code_2022::{Aoc, Input};
use std::{collections::HashSet, fmt::Display, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Blizzards(Vec<Direction>),
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'v' => Tile::Blizzards(vec![Direction::South]),
            '>' => Tile::Blizzards(vec![Direction::East]),
            '<' => Tile::Blizzards(vec![Direction::West]),
            '^' => Tile::Blizzards(vec![Direction::North]),
            _ => panic!("Unknown tile: {value}"),
        }
    }
}

impl Tile {
    fn into_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Blizzards(directions) => {
                if directions.len() > 1 {
                    char::from_digit(directions.len() as u32, 10).unwrap()
                } else {
                    match directions[0] {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::West => '<',
                    }
                }
            }
            Tile::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = self
            .map
            .iter()
            .map(|row| row.iter().map(|tile| tile.into_char()).collect::<String>())
            .collect();
        write!(f, "{}", map.join("\n"))
    }
}

type Position = (usize, usize);

impl Map {
    /// update the position of a blizzard on the map
    fn update_blizzard(&mut self, direction: &Direction, from: Position, to: Position) {
        let (x, y) = from;
        if let Tile::Blizzards(directions) = &mut self.map[y][x] {
            if directions.len() == 1 {
                self.map[y][x] = Tile::Empty;
            } else {
                let i = directions.iter().position(|d| d == direction).unwrap();
                directions.remove(i);
            }
        } else {
            panic!("Old position is not a blizzard")
        }

        // add to existing list or create new one
        let (x, y) = to;
        if let Tile::Blizzards(directions) = &mut self.map[y][x] {
            directions.push(*direction);
        } else {
            self.map[y][x] = Tile::Blizzards(vec![*direction]);
        }
    }

    /// Update all the positions of the blizzards on the map
    fn update_blizzards(&mut self) {
        let mut moves = vec![];
        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[0].len() - 1 {
                if let Tile::Blizzards(directions) = &self.map[y][x] {
                    for direction in directions.clone().iter() {
                        let to = match direction {
                            Direction::North => (
                                x,
                                (((y as isize - 2).rem_euclid(self.map.len() as isize - 2)) + 1)
                                    as usize,
                            ),
                            Direction::South => (x, (y % (self.map.len() - 2)) + 1),
                            Direction::East => ((x % (self.map[0].len() - 2)) + 1, y),
                            Direction::West => (
                                (((x as isize - 2).rem_euclid(self.map[0].len() as isize - 2)) + 1)
                                    as usize,
                                y,
                            ),
                        };
                        moves.push((direction.clone(), (x, y), to));
                    }
                }
            }
        }

        for (direction, from, to) in moves.into_iter() {
            self.update_blizzard(&direction, from, to)
        }
    }

    /// Walk through the map and return the amount of steps it took
    fn walk_map(&mut self, start_position: Position, end_position: Position) -> usize {
        let mut positions = HashSet::new();
        positions.insert(start_position);

        let mut step = 1;
        loop {
            // move blizzards
            self.update_blizzards();

            // validate and walk shortest path
            let mut new_positions = vec![];
            positions.retain(|pos| {
                let (x, y) = *pos;
                // can go north?
                if y != 0 {
                    if let Tile::Empty = self.map[y - 1][x] {
                        new_positions.push((x, y - 1));
                    }
                }
                // south
                if y != self.map.len() - 1 {
                    if let Tile::Empty = self.map[y + 1][x] {
                        new_positions.push((x, y + 1));
                    }
                }
                // west
                if let Tile::Empty = self.map[y][x - 1] {
                    new_positions.push((x - 1, y));
                }
                // east
                if let Tile::Empty = self.map[y][x + 1] {
                    new_positions.push((x + 1, y));
                }

                // remove positions that have blizzards on them now
                if let Tile::Blizzards(_) = self.map[y][x] {
                    false
                } else {
                    true
                }
            });

            // if one of the new positions is at the end finish
            for pos in new_positions.iter() {
                if pos.0 == end_position.0 && pos.1 == end_position.1 {
                    return step;
                }
            }

            positions.extend(new_positions);

            step += 1;
        }
    }
}

impl From<Input> for Map {
    fn from(value: Input) -> Self {
        let map = value
            .lines()
            .into_iter()
            .map(|l| l.unwrap().chars().map(|c| Tile::from(c)).collect())
            .collect();

        Map { map }
    }
}

// 45.7 ms
fn part1(input: Input) -> usize {
    let mut map = Map::from(input);
    let start = (1, 0);
    let end = (map.map[0].len() - 2, map.map.len() - 1);
    return map.walk_map(start, end);
}

// 128.7 ms
fn part2(input: Input) -> usize {
    let mut map = Map::from(input);
    let start = (1, 0);
    let end = (map.map[0].len() - 2, map.map.len() - 1);
    return map.walk_map(start, end) + map.walk_map(end, start) + map.walk_map(start, end);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
