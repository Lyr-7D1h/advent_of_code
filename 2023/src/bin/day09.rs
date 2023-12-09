use advent_of_code_2023::Aoc;

/// 332.437µs
fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|l| {
            let values: Vec<i32> = l.split(" ").map(|i| i.parse().unwrap()).collect();

            let mut differences = vec![values];

            let mut diff_i = 0;
            loop {
                let mut all_zero = true;
                let mut row = vec![];
                for i in 1..differences[diff_i].len() {
                    let a = differences[diff_i][i];
                    let b = differences[diff_i][i - 1];
                    let diff = a - b;

                    if diff != 0 {
                        all_zero = false
                    }

                    row.push(diff);
                }
                if all_zero {
                    break;
                }
                differences.push(row);
                diff_i += 1;
            }

            differences
                .into_iter()
                .rev()
                .fold(0, |a, seq| a + seq.last().unwrap())
        })
        .sum()
}

/// 350.844µs
fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|l| {
            let values: Vec<i32> = l.split(" ").map(|i| i.parse().unwrap()).collect();

            let mut differences = vec![values];

            let mut diff_i = 0;
            loop {
                let mut all_zero = true;
                let mut row = vec![];
                for i in 1..differences[diff_i].len() {
                    let a = differences[diff_i][i];
                    let b = differences[diff_i][i - 1];
                    let diff = a - b;

                    if diff != 0 {
                        all_zero = false
                    }

                    row.push(diff);
                }
                if all_zero {
                    break;
                }
                differences.push(row);
                diff_i += 1;
            }

            differences
                .into_iter()
                .rev()
                .fold(0, |a, seq|  seq.first().unwrap() - a)
        })
        .sum()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
