use advent_of_code_2023::Aoc;

// Average Duration: 182.463µs
fn part1(input: String) -> u32 {
    let mut sum = 0;
    'outer: for line in input.lines() {
        let mut diff = vec![];
        let mut parts = line.split(" ");
        let mut prev = parts.next().unwrap().parse::<i32>().unwrap();
        while let Some(i) = parts.next() {
            let i = i.parse::<i32>().unwrap();
            diff.push(i - prev);
            prev = i;
        }
        let sign = diff[0].signum();
        if diff[0].abs() > 3 {
            continue;
        }
        for i in diff[1..].iter() {
            if i.signum() != sign {
                continue 'outer;
            }
            if i.abs() > 3 {
                continue 'outer;
            }
        }
        sum += 1;
    }

    return sum;
}

/// Duration: 210.595µs
fn part2(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut diff = vec![];
        let mut parts = line.split(" ");
        let mut prev = parts.next().unwrap().parse::<i32>().unwrap();
        while let Some(i) = parts.next() {
            let i = i.parse::<i32>().unwrap();
            diff.push(prev - i);
            prev = i;
        }
        fn remove_invalid_levels(diff: &mut Vec<i32>, mut damper: bool) -> bool {
            // get majority sign to compare the list to
            let sign = diff[diff.len() - 3..]
                .iter()
                .map(|i| i.signum())
                .sum::<i32>()
                .signum();
            if sign == 0 {
                return false;
            }
            let mut c = 0;
            while let Some(i) = diff.pop() {
                c += 1;
                // if not majority sign or greater than 3 it is invalid
                if i.signum() != sign || i.abs() > 3 {
                    if damper {
                        // if first can also remove first item ignoring the first difference
                        if c == 1 {
                            let mut d = diff.clone();
                            if remove_invalid_levels(&mut d, false) {
                                return true;
                            }
                        }
                        if let Some(last) = diff.last_mut() {
                            *last += i;
                        }
                        damper = false;
                        continue;
                    }
                    return false;
                }
            }
            true
        }
        let mut dr = diff.clone();
        dr.reverse();
        // check both ways
        if remove_invalid_levels(&mut diff, true) || remove_invalid_levels(&mut dr, true) {
            sum += 1;
        }
    }

    return sum;
}

fn part2_o(contents: String) -> usize {
    apply_record_safety(
        |record| {
            if is_record_safe(record) || is_record_safe_without_one_element(record) {
                println!(
                    "{:?}",
                    record
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                true
            } else {
                false
            }
        },
        contents,
    )
}

fn apply_record_safety<F>(safety_func: F, contents: String) -> usize
where
    F: Fn(&Vec<i32>) -> bool,
{
    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(safety_func)
        .count()
}

fn is_record_safe(record: &Vec<i32>) -> bool {
    let is_desc = record[0] > record[1];

    for i in 0..(record.len() - 1) {
        let a = record[i];
        let b = record[i + 1];
        if is_desc && a < b || !is_desc && a > b {
            return false;
        }

        let diff = (a - b).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn is_record_safe_without_one_element(record: &Vec<i32>) -> bool {
    (0..record.len()).any(|i| {
        let mut tmp = record.clone();
        tmp.remove(i);
        is_record_safe(&tmp)
    })
}

pub fn is_line_valid(line: &Vec<u32>) -> bool {
    line.windows(2)
        .all(|w| 1 <= w[0].abs_diff(w[1]) && w[0].abs_diff(w[1]) <= 3)
        && (line.is_sorted() || line.iter().rev().is_sorted())
}
pub fn part2_(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| {
            if is_line_valid(line)
                || (0..line.len()).any(|skipped| {
                    let new_line = line
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, val)| if idx != skipped { Some(*val) } else { None })
                        .collect::<Vec<u32>>();
                    is_line_valid(&new_line)
                })
            {
                println!(
                    "{:?}",
                    line.into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                true
            } else {
                false
            }
        })
        .count() as u32
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
