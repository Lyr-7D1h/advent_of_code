use advent_of_code_2023::Aoc;

/// 70.859µs
fn part1(input: String) -> u32 {
    input
        .trim_end() // remove trailing newline
        .split(",")
        .map(|s| {
            s.chars()
                .fold(0, |value, c| ((value + c as u32) * 17) % 256)
        })
        .sum()
}

/// 228.742µs
fn part2(input: String) -> usize {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for s in input.trim_end().split(",") {
        let mut chars = s.chars();
        let mut label = String::new();
        let mut hash = 0;
        while let Some(c) = chars.next() {
            match c {
                '=' => {
                    let focal: u32 = chars.next().unwrap().to_digit(10).unwrap();
                    if let Some((_, f)) = boxes[hash].iter_mut().find(|(l, _)| *l == label) {
                        *f = focal;
                    } else {
                        boxes[hash].push((label, focal))
                    }
                    break;
                }
                '-' => {
                    if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
                        boxes[hash].remove(i);
                    }
                    break;
                }
                _ => {
                    hash = ((hash + c as usize) * 17) % 256;
                    label.push(c);
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(bi, b)| {
            b.into_iter()
                .enumerate()
                .map(|(si, (_, f))| (bi + 1) * (si + 1) * f as usize)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
