use advent_of_code_2023::Aoc;

// 25us
fn part1(input: String) -> u32 {
    return input
        .lines()
        .filter_map(|l| {
            let mut spaced = l.split(" ");
            spaced.next();

            let mut id = spaced.next().unwrap();
            id = &id[0..id.len() - 1];
            let id = id.parse::<u32>().unwrap();

            while let Some(n) = spaced.next() {
                let n = n.parse::<u32>().unwrap();
                let color = spaced.next().unwrap();
                match color.chars().next().unwrap() {
                    'r' if n > 12 => return None,
                    'g' if n > 13 => return None,
                    'b' if n > 14 => return None,
                    _ => {}
                };
            }

            Some(id)
        })
        .sum();
}

// 40us
fn part2(input: String) -> u32 {
    return input
        .lines()
        .map(|l| {
            let mut spaced = l.split(" ").skip(2);

            let mut max = (0, 0, 0);
            while let Some(n) = spaced.next() {
                let n = n.parse::<u32>().unwrap();
                let color = spaced.next().unwrap();
                match color.chars().next().unwrap() {
                    'r' => max.0 = max.0.max(n),
                    'g' => max.1 = max.1.max(n),
                    'b' => max.2 = max.2.max(n),
                    _ => {}
                };
            }

            max.0 * max.1 * max.2
        })
        .sum();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
