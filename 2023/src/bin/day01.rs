use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

// 103us
fn part1(input: Input) -> u32 {
    let mut sum = 0;
    let mut pair: (Option<char>, Option<char>) = (None, None);
    for line in input.lines() {
        for c in line.unwrap().chars() {
            if c.is_ascii_digit() {
                if pair.0.is_none() {
                    pair = (Some(c), Some(c))
                } else {
                    pair.1 = Some(c)
                }
            }
        }
        let mut a = pair.0.unwrap().to_string();
        a.push(pair.1.unwrap());
        sum += a.parse::<u32>().unwrap();
        pair = (None, None);
    }
    return sum;
}

// 359.898µs
fn part2(input: Input) -> u32 {
    let numbers: [Vec<char>; 9] = [
        vec!['n', 'e'],
        vec!['w', 'o'],
        vec!['h', 'r', 'e', 'e'],
        vec!['o', 'u', 'r'],
        vec!['i', 'v', 'e'],
        vec!['i', 'x'],
        vec!['e', 'v', 'e', 'n'],
        vec!['i', 'g', 'h', 't'],
        vec!['i', 'n', 'e'],
    ];

    let mut sum = 0;
    let mut pair: (Option<char>, Option<char>) = (None, None);
    let mut comparisons: Vec<(usize, usize)> = vec![];
    for line in input.lines() {
        for c in line.unwrap().chars() {
            if c.is_ascii_digit() {
                // set pair
                if pair.0.is_none() {
                    pair = (Some(c), Some(c))
                } else {
                    pair.1 = Some(c)
                }
                continue;
            }

            comparisons = comparisons
                .into_iter()
                .filter_map(|(number, mut index)| {
                    if numbers[number][index] != c {
                        return None;
                    }

                    index += 1;
                    if index == numbers[number].len() {
                        let number = char::from_digit((number + 1) as u32, 10).unwrap();
                        // set pair
                        if pair.0.is_none() {
                            pair = (Some(number), Some(number))
                        } else {
                            pair.1 = Some(number)
                        }
                        return None;
                    }
                    Some((number, index))
                })
                .collect();

            match c {
                'o' => comparisons.push((0, 0)),
                't' => {
                    comparisons.push((1, 0));
                    comparisons.push((2, 0));
                }
                'f' => {
                    comparisons.push((3, 0));
                    comparisons.push((4, 0));
                }
                's' => {
                    comparisons.push((5, 0));
                    comparisons.push((6, 0));
                }
                'e' => comparisons.push((7, 0)),
                'n' => comparisons.push((8, 0)),
                _ => {}
            }
        }

        let mut a = pair.0.unwrap().to_string();
        a.push(pair.1.unwrap());

        sum += a.parse::<u32>().unwrap();
        pair = (None, None);
    }
    return sum;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
