use advent_of_code_2022::{Aoc, Input};
use std::{fmt::Display, io::BufRead};

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            c => panic!("Unknown instruction: {c}"),
        }
    }
}

struct RockIterator<'n> {
    map: &'n mut Map,
    current_rock_position: Vec<(usize, usize)>,
    rock: usize,
    instruction: usize,
}

impl<'n> RockIterator<'n> {
    fn new(map: &'n mut Map) -> RockIterator<'n> {
        RockIterator {
            map,
            current_rock_position: vec![],
            rock: 0,
            instruction: 0,
        }
    }
}

impl<'n> Iterator for RockIterator<'n> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_rock_position.len() == 0 {
            self.rock += 1;
            self.current_rock_position = self.map.drop_rock(&mut self.rock, &mut self.instruction);
            self.current_rock_position.reverse();
        }
        return self.current_rock_position.pop();
    }
}

struct Map {
    map: Vec<Vec<bool>>,
    top_level: usize,
    rocks: Vec<Rock>,
    instructions: Vec<Instruction>,
}

impl Map {
    fn new(rocks: Vec<Rock>, instructions: Vec<Instruction>) -> Map {
        Map {
            map: vec![vec![false; 7]; 7], // start with 3 levels
            top_level: 0,
            rocks,
            instructions,
        }
    }

    fn drop_rocks(&mut self, amount_of_rocks: usize) {
        let mut j = 0;
        for i in 0..amount_of_rocks {
            // println!("=================");
            // println!("{self}");
            let mut rock = self.rocks[i % self.rocks.len()].clone();
            // put it on top of the map
            rock.translate((2, self.top_level + 3));

            loop {
                match self.instructions[j % self.instructions.len()] {
                    Instruction::Left => {
                        rock.left(&self.map);
                    }
                    Instruction::Right => {
                        rock.right(&self.map);
                    }
                }
                j += 1;

                if let None = rock.down(&self.map) {
                    break;
                }
            }

            // set the new top
            let top = rock.top() + 1;
            if top > self.top_level {
                for _ in 0..top - self.top_level {
                    self.map.push(vec![false; 7]);
                }
                self.top_level = top;
            }

            // solidify structure
            for (x, y) in rock.positions.into_iter() {
                self.map[y][x] = true;
            }
        }
    }

    fn drop_rock(&mut self, i: &mut usize, j: &mut usize) -> Vec<(usize, usize)> {
        let mut rock = self.rocks[*i % self.rocks.len()].clone();
        // set the rock in the middle
        rock.translate((2, self.top_level + 3));

        loop {
            match self.instructions[*j % self.instructions.len()] {
                Instruction::Left => {
                    rock.left(&self.map);
                }
                Instruction::Right => {
                    rock.right(&self.map);
                }
            }
            *j += 1;

            if let None = rock.down(&self.map) {
                break;
            }
        }

        // set the new top
        let top = rock.top() + 1;
        if top > self.top_level {
            for _ in 0..top - self.top_level {
                self.map.push(vec![false; 7]);
            }
            self.top_level = top;
        }

        // solidify structure
        for (x, y) in rock.positions.iter() {
            self.map[*y][*x] = true;
        }

        return rock.positions;
    }

    fn drop_rocks_with_cycle_detection(&mut self, amount_of_rocks: usize) {
        let mut iterator = RockIterator::new(self).peekable();

        while let Some(p) = iterator.next() {
            println!("{}", p.0);
        }
        let mut history = Vec::from([iterator.next().unwrap().0]);

        for i in 0..amount_of_rocks {}

        let mut history = Vec::from([iterator.next().unwrap().0]);
        'n: while amount_of_rocks > 0 {
            let mut local_history = vec![];

            for a in history.iter() {
                let b = iterator.next().unwrap().0;
                if *a != b {
                    history.push(b);
                    if local_history.len() > 0 {
                        println!("{local_history:?}");
                        history.append(&mut local_history)
                    }
                    continue 'n;
                }
                local_history.push(b);
            }

            // local_history matches history
            assert!(history.len() == local_history.len());

            for a in history.iter() {
                let b = iterator.next().unwrap().0;
                if *a != b {
                    history.push(b);
                    if local_history.len() > 0 {
                        history.append(&mut local_history)
                    }
                    continue 'n;
                }
                local_history.push(b);
            }
            panic!("Pattern found!");
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map: Vec<String> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();

        map.reverse();

        write!(f, "{}", map.join("\n"))
    }
}

#[derive(Debug, Clone)]
struct Rock {
    positions: Vec<(usize, usize)>,
}

impl Rock {
    fn new(positions: Vec<(usize, usize)>) -> Rock {
        Rock { positions }
    }

    fn top(&self) -> usize {
        return self.positions.iter().fold(0, |a, b| a.max(b.1));
    }

    // validates and goes left when able
    fn left(&mut self, map: &Vec<Vec<bool>>) -> Option<()> {
        for (x, y) in self.positions.iter() {
            if *x == 0 || map[*y][x - 1] {
                return None;
            }
        }
        for (x, _) in self.positions.iter_mut() {
            *x -= 1;
        }

        return Some(());
    }

    // validates and goes right when able
    fn right(&mut self, map: &Vec<Vec<bool>>) -> Option<()> {
        for (x, y) in self.positions.iter() {
            if *x == map[0].len() - 1 || map[*y][x + 1] {
                return None;
            }
        }
        for (x, _) in self.positions.iter_mut() {
            *x += 1;
        }

        return Some(());
    }

    // go as down as possible until max
    fn down(&mut self, map: &Vec<Vec<bool>>) -> Option<()> {
        for (x, y) in self.positions.iter() {
            if *y == 0 || map[y - 1][*x] {
                return None;
            }
        }
        // translate down
        for (_, y) in self.positions.iter_mut() {
            *y -= 1;
        }

        return Some(());
    }

    /// Translate the position of the rock according the the down left corner
    fn translate(&mut self, position: (usize, usize)) {
        for p in self.positions.iter_mut() {
            p.0 += position.0;
            p.1 += position.1;
        }
    }
}

fn part1(input: Input) -> usize {
    let mut instructions = vec![];
    for c in input.lines().next().unwrap().unwrap().chars() {
        instructions.push(Instruction::from(c));
    }

    let rocks = vec![
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]), // -
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]), // +
        Rock::new(vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]), // _|
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]), // |
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]), // []
    ];

    let mut map = Map::new(rocks, instructions);

    map.drop_rocks(2022);

    return map.top_level;
}

fn part2(input: Input) -> usize {
    let mut instructions = vec![];
    for c in input.lines().next().unwrap().unwrap().chars() {
        instructions.push(Instruction::from(c));
    }

    let rocks = vec![
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]), // -
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]), // +
        Rock::new(vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]), // _|
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]), // |
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]), // []
    ];

    let mut map = Map::new(rocks, instructions);

    // map.drop_rocks_with_cycle_detection(1000000000000);
    map.drop_rocks_with_cycle_detection(200);

    return map.top_level;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
