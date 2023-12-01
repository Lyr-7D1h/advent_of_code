use advent_of_code_2022::{Aoc, Input};
use std::{fmt::Display, io::BufRead};

#[repr(isize)]
#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn left(&mut self) -> Direction {
        let i = ((*self as isize) - 1).rem_euclid(4);
        return unsafe { ::std::mem::transmute(i) };
    }
    fn rigth(&mut self) -> Direction {
        let i = ((*self as isize) + 1).rem_euclid(4);
        return unsafe { ::std::mem::transmute(i) };
    }
}

enum Tile {
    Empty,
    Open,
    Wall,
    Step(Direction),
}

struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn start_position(&self) -> (usize, usize) {
        for (y, r) in self.map.iter().enumerate() {
            for (x, t) in r.iter().enumerate() {
                if let Tile::Open = t {
                    return (x, y);
                }
            }
        }
        panic!("No start position found")
    }

    fn step(&self, position: &mut (usize, usize), direction: &Direction) {
        let (x, y) = position;
        match direction {
            Direction::Right => *x = (*x + 1) % self.map[*y].len(),
            Direction::Down => *y = (*y + 1) % self.map.len(),
            Direction::Left => {
                *x = ((*x as isize - 1).rem_euclid(self.map[*y].len() as isize)) as usize
            }
            Direction::Up => *y = ((*y as isize - 1).rem_euclid(self.map.len() as isize)) as usize,
        }
    }

    fn follow_instructions(
        &mut self,
        instructions: Vec<Instruction>,
    ) -> ((usize, usize), Direction) {
        let mut direction = Direction::Right;
        let mut pos = self.start_position();
        for instruction in instructions.into_iter() {
            match instruction {
                Instruction::Step(i) => {
                    for _ in 0..i {
                        let mut new_pos = pos.clone();
                        self.step(&mut new_pos, &direction);

                        // skip empty or non existing spots
                        loop {
                            match self
                                .map
                                .get(new_pos.1)
                                .map(|row| row.get(new_pos.0))
                                .flatten()
                            {
                                None | Some(Tile::Empty) => {
                                    self.step(&mut new_pos, &direction);
                                }
                                _ => break,
                            }
                        }

                        let (x, y) = new_pos;
                        if let Tile::Wall = self.map[y][x] {
                            break;
                        }

                        self.map[y][x] = Tile::Step(direction);
                        pos = new_pos;
                    }
                }
                Instruction::Left => direction = direction.left(),
                Instruction::Rigth => direction = direction.rigth(),
            }
        }
        return (pos, direction);
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let mut map = vec![];
        for l in value.into_iter() {
            let mut row = vec![];
            for c in l.chars() {
                match c {
                    ' ' => row.push(Tile::Empty),
                    '.' => row.push(Tile::Open),
                    '#' => row.push(Tile::Wall),
                    c => panic!("Unknown tile: {c}"),
                }
            }
            map.push(row);
        }

        Map { map }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: Vec<String> = self
            .map
            .iter()
            .map(|row| {
                row.into_iter()
                    .map(|t| match t {
                        Tile::Empty => ' ',
                        Tile::Open => '.',
                        Tile::Wall => '#',
                        Tile::Step(direction) => match direction {
                            Direction::Right => '>',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Up => '^',
                        },
                    })
                    .collect()
            })
            .collect();

        write!(f, "{}", map.join("\n"))
    }
}

#[derive(Debug)]
enum Instruction {
    Step(usize),
    Left,
    Rigth,
}

fn instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut prev = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            prev.push(c);
            continue;
        }

        if prev.len() > 0 {
            instructions.push(Instruction::Step(prev.parse().unwrap()));
            prev.clear();
        }

        match c {
            'R' => {
                instructions.push(Instruction::Rigth);
            }
            'L' => {
                instructions.push(Instruction::Left);
            }
            c => panic!("Unknown instruction {c}"),
        }
    }

    return instructions;
}

fn part1(input: Input) -> usize {
    let lines: Vec<String> = input.lines().collect::<Result<_, _>>().unwrap();

    let mut map = Map::from(&lines[..lines.len() - 2]);
    let instructions = instructions(lines.last().unwrap());

    let ((x, y), direction) = map.follow_instructions(instructions);

    println!("{map}");
    return (y + 1) * 1000 + (x + 1) * 4 + direction as usize;
}

fn part2(input: Input) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
