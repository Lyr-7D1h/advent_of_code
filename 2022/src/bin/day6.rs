use advent_of_code_2022::{Aoc, Input};
use std::{io::BufRead, str::Chars};

fn start_packet(size: usize, data: Chars) -> usize {
    let mut queue = vec![];
    'outer: for (i, c) in data.enumerate() {
        queue.push(c);
        if queue.len() < size {
            continue;
        }

        for i in 0..size - 1 {
            for offset in 1..size {
                if queue[i] == queue[(i + offset) % size] {
                    queue.remove(0);
                    continue 'outer;
                }
            }
        }

        return i + 1;
    }

    panic!("Not found")
}

// 320 us
fn part1(mut input: Input) -> usize {
    let mut stream = String::new();
    input.read_line(&mut stream).unwrap();

    return start_packet(4, stream.chars());
}

// 310 us
fn part1_windows(mut input: Input) -> usize {
    let mut stream = String::new();
    input.read_line(&mut stream).unwrap();

    stream
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|window| {
            for i in 0..3 {
                for offset in 1..4 {
                    if window[i] == window[(i + offset) % 4] {
                        return false;
                    }
                }
            }
            return true;
        })
        .map(|pos| pos + 4)
        .unwrap()
}

// 3.2ms
fn part2(mut input: Input) -> usize {
    let mut stream = String::new();
    input.read_line(&mut stream).unwrap();

    return start_packet(14, stream.chars());
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("one_windows", part1_windows);
    aoc.part("two", part2);
    aoc.run();
}
