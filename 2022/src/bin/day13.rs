use advent_of_code_2022::{Aoc, Input};
use std::{io::BufRead, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: String) -> Option<Packet> {
        return Self::parse_recursive(&mut line.chars().peekable());
    }

    // Parse by character
    fn parse_recursive(chars: &mut Peekable<Chars>) -> Option<Packet> {
        match chars.next() {
            Some(c) if c.is_ascii_digit() => {
                if let Some('0') = chars.peek() {
                    chars.next();
                    Some(Self::Value(10))
                } else {
                    Some(Self::Value(c as u8 - b'0'))
                }
            }
            Some('[') => {
                let mut list = vec![];
                while let Some(v) = Self::parse_recursive(chars) {
                    list.push(v);
                    // will consume commas too, only break on closing of array
                    if let Some(']') = chars.next() {
                        break;
                    }
                }
                Some(Self::List(list))
            }
            _ => None,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
            (Packet::Value(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(_), Packet::Value(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(a), Packet::List(b)) => a
                .iter()
                .zip(b.iter())
                .find_map(|(a, b)| {
                    let res = a.cmp(b);
                    res.is_ne().then_some(res)
                })
                .unwrap_or(a.len().cmp(&b.len())),
        }
    }
}

// Total ordering
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 2ms
fn part1(input: Input) -> usize {
    let mut pairs = vec![];
    let lines = &mut input.lines();
    while let (Some(a), Some(b)) = (lines.next(), lines.next()) {
        pairs.push((Packet::parse(a.unwrap()), Packet::parse(b.unwrap())));
        lines.next();
    }

    let mut total = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if a.cmp(b).is_le() {
            total += i + 1;
        }
    }

    return total;
}

// 3.4ms 
fn part2(input: Input) -> usize {
    let mut packets = vec![];
    let lines = &mut input.lines();
    while let Some(Ok(l)) = lines.next() {
        if l != "" {
            if let Some(p) = Packet::parse(l) {
                packets.push(p)
            }
        }
    }

    let d6: Packet = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    let d2: Packet = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);

    packets.push(d6.clone());
    packets.push(d2.clone());

    packets.sort();

    return packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| p == &d6 || p == &d2)
        .map(|(i, _)| i + 1)
        .reduce(|a, b| a * b)
        .unwrap();
}

// NOTE: solution heavily inspired by: https://github.com/quat1024/hatchery/blob/trunk/advent2022/src/day13.rs
fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
