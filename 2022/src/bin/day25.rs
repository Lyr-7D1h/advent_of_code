#![feature(int_roundings)]
use advent_of_code_2022::{Aoc, Input};
use std::{
    io::BufRead,
    iter::Sum,
    ops::{Add, Div},
};

#[derive(Debug)]
struct Snafu {
    value: i64,
}

impl Snafu {
    fn to_string(&self) -> String {
        let i = (self.value as f64).log(5.0).round() as i32;

        let mut snafu = String::new();
        let mut remainder = self.value;

        for i in (0..i + 1).rev() {
            let power = (5 as i64).pow(i as u32);
            // println!("Power {remainder} {power} {i}");
            let diff = remainder.abs_diff(power);
            let ndiff = (-remainder).abs_diff(power);
            let div = (remainder.abs() as f64).div(power as f64).round() as i32;

            // println!("Div: {div} Rem: {remainder}");
            if div == 0 {
                snafu.push('0');
                continue;
            }
            if ndiff >= diff {
                if div >= 2 {
                    snafu.push('2');
                    remainder -= 2 * power;
                } else {
                    snafu.push('1');
                    remainder -= power;
                }
            } else if diff > ndiff {
                if div >= 2 {
                    snafu.push('=');
                    remainder += 2 * power;
                } else {
                    snafu.push('-');
                    remainder += power;
                }
            } else {
                snafu.push('0');
            }
        }

        // println!("{remainder}");
        debug_assert!(remainder == 0);
        return snafu;
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        Snafu {
            value: iter.map(|s| s.value).sum(),
        }
    }
}

impl Add for Snafu {
    type Output = i64;

    fn add(self, rhs: Self) -> Self::Output {
        self.value + rhs.value
    }
}

impl From<String> for Snafu {
    fn from(input: String) -> Self {
        let mut value = 0;
        for (i, c) in input.chars().rev().enumerate() {
            let power: i64 = (5 as i64).pow(i as u32);
            match c {
                '1' => value += power,
                '2' => value += 2 * power,
                '=' => value -= 2 * power,
                '-' => value -= power,
                '0' => {}
                c => panic!("Unknown snafu character {c}"),
            }
        }
        Snafu { value }
    }
}

fn part1(input: Input) -> String {
    let input: Vec<Snafu> = input
        .lines()
        .map(|line| Snafu::from(line.unwrap()))
        .collect();

    let sum: Snafu = input.into_iter().sum();

    return sum.to_string();
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
