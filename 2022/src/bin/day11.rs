use advent_of_code_2022::{Aoc, Input};
use std::{fmt::Debug, io::BufRead};

#[derive(Debug)]
struct DivisionTest {
    divisor: u64,
    on_true: usize,
    on_false: usize,
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    inspected: u64,
    division_test: DivisionTest,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspected", &self.inspected)
            .field("division_test", &self.division_test)
            .finish()
    }
}

impl From<Vec<String>> for Monkey {
    fn from(value: Vec<String>) -> Self {
        let items = value[1]
            .trim()
            .split(" ")
            .skip(2)
            .map(|s| s.replace(",", "").parse::<u64>().unwrap())
            .collect();

        let mut raw_operation = value[2].trim().split(" ").skip(4);
        let op = raw_operation.next().unwrap();
        let b = raw_operation.next().unwrap();

        let divisor = value[3]
            .trim()
            .split(" ")
            .skip(3)
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let on_true = value[4]
            .trim()
            .split(" ")
            .skip(5)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let on_false = value[5]
            .trim()
            .split(" ")
            .skip(5)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let division_test = DivisionTest {
            divisor,
            on_true,
            on_false,
        };

        match op {
            "*" if b == "old" => Monkey {
                items,
                operation: Box::new(move |x| x * x),
                division_test,
                inspected: 0,
            },
            "*" => {
                let num: u64 = b.parse().unwrap();
                Monkey {
                    items,
                    operation: Box::new(move |x| x * num),
                    division_test,
                    inspected: 0,
                }
            }
            "+" => {
                let num: u64 = b.parse().unwrap();
                Monkey {
                    items,
                    operation: Box::new(move |x| x + num),
                    division_test,
                    inspected: 0,
                }
            }
            _ => panic!("Invalid operation: {op}"),
        }
    }
}

// 270ns
fn part1(input: Input) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .into_iter()
        .fold(vec![vec![]], |mut acc, s| {
            let s = s.unwrap();
            if s == "" {
                acc.push(vec![]);
            } else {
                acc.last_mut().unwrap().push(s);
            }
            acc
        })
        .into_iter()
        .map(|lines| Monkey::from(lines))
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                item = (monkeys[i].operation)(item) / 3;

                let DivisionTest {
                    divisor,
                    on_true,
                    on_false,
                } = monkeys[i].division_test;
                if item % divisor as u64 == 0 {
                    monkeys[on_true].items.push(item);
                } else {
                    monkeys[on_false].items.push(item);
                }
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    return monkeys[0].inspected * monkeys[1].inspected;
}

// 90ms
fn part2(input: Input) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .into_iter()
        .fold(vec![vec![]], |mut acc, s| {
            let s = s.unwrap();
            if s == "" {
                acc.push(vec![]);
            } else {
                acc.last_mut().unwrap().push(s);
            }
            acc
        })
        .into_iter()
        .map(|lines| Monkey::from(lines))
        .collect();

    let modulo: u64 = monkeys.iter().map(|m| m.division_test.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                item = (monkeys[i].operation)(item) % modulo;

                let DivisionTest {
                    divisor,
                    on_true,
                    on_false,
                } = monkeys[i].division_test;
                if item % divisor as u64 == 0 {
                    monkeys[on_true].items.push(item);
                } else {
                    monkeys[on_false].items.push(item);
                }
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    return monkeys[0].inspected * monkeys[1].inspected;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
