use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

fn visible_left(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    for o in 0..x {
        if map[y][o] >= map[y][x] {
            return false;
        }
    }

    return true;
}
fn visible_right(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    for o in x + 1..map[0].len() {
        if map[y][o] >= map[y][x] {
            return false;
        }
    }

    return true;
}
fn visible_up(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    for o in 0..y {
        if map[o][x] >= map[y][x] {
            return false;
        }
    }

    return true;
}
fn visible_down(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    for o in y + 1..map.len() {
        if map[o][x] >= map[y][x] {
            return false;
        }
    }

    return true;
}

fn input_to_map(input: Input) -> Vec<Vec<u8>> {
    let mut map = vec![];

    for line in input.lines() {
        let line = line.unwrap();
        let mut row = vec![];
        for c in line.chars() {
            row.push(c as u8 - b'0')
        }
        map.push(row);
    }

    return map;
}

// 19ms
fn part1(input: Input) -> usize {
    let map = input_to_map(input);
    let column_length = map[0].len();
    let row_length = map.len();
    let mut visible_sides = 2 * (column_length + row_length) - 4;
    for y in 1..row_length - 1 {
        for x in 1..column_length - 1 {
            if visible_left(&map, x, y)
                || visible_right(&map, x, y)
                || visible_up(&map, x, y)
                || visible_down(&map, x, y)
            {
                visible_sides += 1;
            }
        }
    }

    return visible_sides;
}

fn distance_left(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    for o in 1..x + 1 {
        if map[y][x - o] >= map[y][x] {
            return o;
        }
    }

    return x;
}

fn distance_right(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let max = map[0].len();
    for o in x + 1..max {
        if map[y][o] >= map[y][x] {
            return o - x;
        }
    }

    return max - x - 1;
}
fn distance_up(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    for o in 1..y + 1 {
        if map[y - o][x] >= map[y][x] {
            return o;
        }
    }

    return y;
}
fn distance_down(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let max = map.len();
    for o in y + 1..max {
        if map[o][x] >= map[y][x] {
            return o - y;
        }
    }

    return max - y - 1;
}

// 8.9ms
fn part2(input: Input) -> usize {
    let map = input_to_map(input);

    let mut max = 0;

    let column_length = map[0].len();
    let row_length = map.len();
    for y in 1..row_length - 1 {
        for x in 1..column_length - 1 {
            // println!("left: {}", distance_left(&map, x, y));
            // println!("up: {}", distance_up(&map, x, y));
            // println!("rigth: {}", distance_right(&map, x, y));
            let score = distance_left(&map, x, y)
                * distance_up(&map, x, y)
                * distance_down(&map, x, y)
                * distance_right(&map, x, y);

            // println!("{} score {score}", map[y][x]);
            if score > max {
                max = score
            }
        }
    }

    return max;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
