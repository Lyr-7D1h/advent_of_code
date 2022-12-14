use advent_of_code_2022::{Aoc, Input};
use std::{cmp, collections::VecDeque, fmt::Display, io::BufRead};

struct Map {
    map: Vec<VecDeque<bool>>,
    drop_point: usize,
    infinite_floor_enabled: bool,
}

enum Side {
    Left,
    Right,
}

impl Map {
    fn add_column(&mut self, side: Side) {
        for i in 0..self.map.len() - 1 {
            match side {
                Side::Left => self.map[i].push_front(false),
                Side::Right => self.map[i].push_back(false),
            }
        }
        match side {
            Side::Left => self.map.last_mut().unwrap().push_front(true),
            Side::Right => self.map.last_mut().unwrap().push_back(true),
        }
    }

    // returns true if sand places, if it fell of the map returns false
    fn drop_sand(&mut self, mut x: usize, y: usize) -> bool {
        // validate if down exists
        if y + 1 == self.map.len() {
            return false;
        }
        // keep going down if nothing is under
        if !self.map[y + 1][x] {
            return self.drop_sand(x, y + 1);
        }

        // validate the diagnol left exists
        if x == 0 {
            if self.infinite_floor_enabled {
                x += 1;
                self.drop_point += 1;
                self.add_column(Side::Left)
            } else {
                return false;
            }
        }
        // keep going diagonally left if possible
        if !self.map[y + 1][x - 1] {
            return self.drop_sand(x - 1, y + 1);
        }

        if x + 1 == self.map[0].len() {
            if self.infinite_floor_enabled {
                self.add_column(Side::Right)
            } else {
                return false;
            }
        }
        // keep going diagonally right if possible
        if !self.map[y + 1][x + 1] {
            return self.drop_sand(x + 1, y + 1);
        }

        self.map[y][x] = true;
        true
    }

    fn add_infinite_floor(&mut self) {
        self.map
            .push(VecDeque::from(vec![false; self.map[0].len()]));
        self.map.push(VecDeque::from(vec![true; self.map[0].len()]));
        self.infinite_floor_enabled = true;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut drop_indicator = String::new();
        for i in 0..self.map[0].len() {
            if i == self.drop_point {
                drop_indicator.push('+');
            } else {
                drop_indicator.push('.');
            }
        }
        let mut map: Vec<String> = vec![drop_indicator];
        map.append(
            &mut self
                .map
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|o| if *o { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<String>>(),
        );

        write!(f, "{}", map.join("\n"))
    }
}

impl From<Input> for Map {
    fn from(value: Input) -> Self {
        let lines: Vec<Vec<(usize, usize)>> = value
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(" -> ")
                    .map(|p| {
                        let mut split = p.split(",");
                        (
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        )
                    })
                    .collect()
            })
            .collect();

        // TODO make more efficient
        let x_min = lines.iter().fold(usize::MAX, |min, l| {
            min.min(l.iter().fold(usize::MAX, |min, c| c.0.min(min)))
        });
        let x_max = lines
            .iter()
            .fold(0, |max, l| max.max(l.iter().fold(0, |max, c| c.0.max(max))));
        // let y_min = lines.iter().fold(usize::MAX, |min, l| {
        //     min.min(l.iter().fold(usize::MAX, |min, c| c.1.min(min)))
        // });
        let y_max = lines
            .iter()
            .fold(0, |max, l| max.max(l.iter().fold(0, |max, c| c.1.max(max))));

        // give extra space for dropped sand
        let mut map = vec![VecDeque::from(vec![false; x_max - x_min + 1]); y_max + 1];

        for l in lines.iter() {
            let mut coordinates = l.iter().peekable();
            while let Some(from) = coordinates.next() {
                if let Some(to) = coordinates.peek() {
                    if from.1 == to.1 {
                        // hold y constant
                        let y = from.1;
                        let start_x = cmp::min(from.0, to.0) - x_min;
                        let distance = from.0.abs_diff(to.0);
                        for i in 0..distance + 1 {
                            map[y][start_x + i] = true;
                        }
                    } else if from.0 == to.0 {
                        // hold x constant
                        let x = from.0 - x_min;
                        let start_y = cmp::min(from.1, to.1);
                        let distance = from.1.abs_diff(to.1);
                        for i in 0..distance + 1 {
                            map[start_y + i][x] = true;
                        }
                    }
                }
            }
        }

        Map {
            map,
            infinite_floor_enabled: false,
            drop_point: 500 - x_min,
        }
    }
}

// 8ms
fn part1(input: Input) -> u32 {
    let mut map = Map::from(input);

    let mut i = 0;
    while map.drop_sand(map.drop_point, 0) {
        i += 1;
    }
    return i;
}

fn part2(input: Input) -> u32 {
    let mut map = Map::from(input);

    map.add_infinite_floor();

    let mut i = 0;
    loop {
        if !map.drop_sand(map.drop_point, 0) {
            panic!("infinite floor not working");
        }
        i += 1;
        if map.map[0][map.drop_point] {
            return i;
        }
    }
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
