use std::collections::HashSet;

use advent_of_code_2023::Aoc;

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("start not found")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn go_to(&self, (mut x, mut y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North if y == 0 => return None,
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::West if x == 0 => return None,
            Direction::West => x -= 1,
            Direction::East => x += 1,
        }
        return Some((x, y));
    }
    fn is_left_relative(&self, dir: Direction) -> bool {
        match self {
            Direction::North if dir == Direction::East => true,
            Direction::North if dir == Direction::West => false,
            Direction::East if dir == Direction::North => false,
            Direction::East if dir == Direction::South => true,
            Direction::West if dir == Direction::North => true,
            Direction::West if dir == Direction::South => false,
            Direction::South if dir == Direction::West => true,
            Direction::South if dir == Direction::East => false,
            _ => panic!(),
        }
    }
    /// bend 90 degrees to the left
    fn bend_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
pub struct Tunnel {
    dir: [Direction; 2],
}

impl Tunnel {
    fn from_dir(dir: [Direction; 2]) -> Tunnel {
        Tunnel { dir }
    }

    fn from_char(c: char) -> Option<Tunnel> {
        let dir = match c {
            '|' => [Direction::North, Direction::South],
            '-' => [Direction::West, Direction::East],
            'F' => [Direction::East, Direction::South],
            '7' => [Direction::West, Direction::South],
            'L' => [Direction::North, Direction::East],
            'J' => [Direction::North, Direction::West],
            'S' => return None,
            '.' => return None,
            _ => panic!("wrong dir"),
        };

        Some(Tunnel { dir })
    }

    fn to_char(&self) -> char {
        match self.dir {
            [Direction::North, Direction::South] => '|',
            [Direction::South, Direction::North] => '|',
            [Direction::West, Direction::East] => '-',
            [Direction::East, Direction::West] => '-',
            [Direction::East, Direction::South] => 'F',
            [Direction::South, Direction::East] => 'F',
            [Direction::West, Direction::South] => '7',
            [Direction::South, Direction::West] => '7',
            [Direction::North, Direction::East] => 'L',
            [Direction::East, Direction::North] => 'L',
            [Direction::North, Direction::West] => 'J',
            [Direction::West, Direction::North] => 'J',
            _ => panic!("wrong dir"),
        }
    }

    /// follow the tunnel coming from `direction`
    fn follow(&self, direction: Direction) -> Option<Direction> {
        if self.dir[0] == direction {
            Some(self.dir[1])
        } else if self.dir[1] == direction {
            Some(self.dir[0])
        } else {
            None
        }
    }
}

/// 213.835µs
fn part1(input: String) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = find_start(&map);

    // {prev direction} {current position}
    let mut pos = [(Direction::North, (0, 0)); 2];
    for dir in [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        if let Some((x, y)) = dir.go_to(start) {
            let c = map[y][x];

            if let Some(tunnel) = Tunnel::from_char(c) {
                let dir = dir.opposite();

                if let Some(_) = tunnel.follow(dir) {
                    if pos[0].1 .0 == 0 && pos[0].1 .1 == 0 {
                        pos[0] = (dir, (x, y));
                    } else {
                        pos[1] = (dir, (x, y));
                    }
                }
            }
        }
    }

    let mut steps = 1;
    loop {
        let a = pos[0].1;
        let b = pos[1].1;
        if a.0 == b.0 && a.1 == b.1 {
            break;
        }
        for i in 0..2 {
            let (direction, (x, y)) = pos[i];
            let tunnel = Tunnel::from_char(map[y][x]).unwrap();
            let new_dir = tunnel.follow(direction).unwrap();
            let new_pos = new_dir.go_to((x, y)).unwrap();
            pos[i] = (new_dir.opposite(), new_pos);
        }
        steps += 1;
    }

    return steps;
}

fn part2(input: String) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = find_start(&map);

    // setup boundry
    // TODO simplify don't need two directions
    // {prev direction} {current position}
    let mut pos = [(Direction::North, (0, 0)); 2];
    for dir in [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        if let Some((x, y)) = dir.go_to(start) {
            let c = map[y][x];

            if let Some(tunnel) = Tunnel::from_char(c) {
                let dir = dir.opposite();

                if let Some(_) = tunnel.follow(dir) {
                    if pos[0].1 .0 == 0 && pos[0].1 .1 == 0 {
                        pos[0] = (dir, (x, y));
                    } else {
                        pos[1] = (dir, (x, y));
                    }
                }
            }
        }
    }

    let mut boundry = HashSet::new();

    // change start into tunnel
    let (x, y) = start;
    map[y][x] = Tunnel::from_dir([pos[0].0.opposite(), pos[1].0.opposite()]).to_char();

    boundry.insert(start);
    boundry.insert(pos[0].1);
    boundry.insert(pos[1].1);
    let mut new_start = None;
    loop {
        let a = pos[0].1;
        let b = pos[1].1;
        if a.0 == b.0 && a.1 == b.1 {
            break;
        }
        for i in 0..2 {
            let (direction, (x, y)) = pos[i];
            let c = map[y][x];
            // assume there will always be a vertical pipe
            if c == '|' {
                if new_start.is_none() {
                    new_start = Some((x, y));
                }
            }
            let tunnel = Tunnel::from_char(c).unwrap();
            let new_dir = tunnel.follow(direction).unwrap();
            let new_pos = new_dir.go_to((x, y)).unwrap();
            boundry.insert(new_pos);
            pos[i] = (new_dir.opposite(), new_pos);
        }
    }

    // tag all nodes that are contained into one side of the boundry
    // TODO don't need new start
    let new_start = new_start.unwrap();
    let (nx, ny) = new_start;
    let mut pos = (Direction::North, Direction::South.go_to(new_start).unwrap());

    let mut tagged = HashSet::new();

    let mut is_outside = false;
    let mut tag = |p: (usize, usize)| {
        let mut queue: Vec<(usize, usize)> = vec![p];
        // tag neighboring nodes in flood fill manner
        while let Some(p) = queue.pop() {
            if tagged.contains(&p) || boundry.contains(&p) {
                continue;
            }
            if p.0 >= map[0].len() || p.1 >= map.len() {
                is_outside = true;
                continue;
            }
            tagged.insert(p);
            for d in [
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
            ] {
                match d.go_to(p) {
                    Some(new_p) => queue.push(new_p),
                    None => is_outside = true,
                }
            }
        }
    };
    let mut viewport = [Direction::West, Direction::South];
    loop {
        let (direction, (x, y)) = pos;

        let c = map[y][x];
        let tunnel = Tunnel::from_char(c).unwrap();
        let new_dir = tunnel.follow(direction).unwrap();

        // change viewport if curve
        match c {
            'F' | '7' | 'J' | 'L' if direction.is_left_relative(new_dir) => {
                if let Some(p) = direction.bend_left().go_to((x, y)) {
                    tag(p);
                }
                viewport = [viewport[0].bend_left(), viewport[1].bend_left()]
            }
            'F' | '7' | 'J' | 'L' => {
                if let Some(p) = direction.bend_left().go_to((x, y)) {
                    tag(p);
                }
                viewport = [
                    viewport[0].bend_left().opposite(),
                    viewport[1].bend_left().opposite(),
                ]
            }
            _ => {}
        };

        for d in viewport {
            if let Some(p) = d.go_to((x, y)) {
                tag(p);
            }
        }

        let new_pos = new_dir.go_to((x, y)).unwrap();
        pos = (new_dir.opposite(), new_pos);

        if x == nx && y == ny {
            break;
        }
    }

    // for (x, y) in tagged.iter().cloned() {
    //     if is_outside {
    //         map[y][x] = 'O'
    //     } else {
    //         map[y][x] = 'I'
    //     }
    // }
    // let map_display: String = map
    //     .iter()
    //     .map(|l| l.iter().collect::<String>())
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // println!("{map_display}");

    if is_outside {
        let total: usize = map.into_iter().map(|l| l.len()).sum();
        return total - tagged.len() - boundry.len();
    }
    return tagged.len();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
