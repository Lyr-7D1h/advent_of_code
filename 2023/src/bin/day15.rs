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

/// optimized to not use copy but string slices
/// 155.767µs
fn part2(input: String) -> usize {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    for s in input.trim_end().split(",") {
        if s.ends_with("-") {
            let label = s.get(..s.len() - 1).unwrap();
            let hash = label
                .chars()
                .fold(0, |value, c| ((value + c as usize) * 17) % 256);

            if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
                boxes[hash].remove(i);
            }
        } else {
            let label = s.get(..s.len() - 2).unwrap();
            let hash = label
                .chars()
                .fold(0, |value, c| ((value + c as usize) * 17) % 256);
            let focal: u32 = s.get(s.len() - 1..).unwrap().parse().unwrap();

            if let Some((_, f)) = boxes[hash].iter_mut().find(|(l, _)| *l == label) {
                *f = focal;
            } else {
                boxes[hash].push((label, focal));
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
