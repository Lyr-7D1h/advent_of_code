use advent_of_code_2023::Aoc;

fn part1(input: String) -> u32 {
    return input
        .lines()
        .filter_map(|l| {
            let mut spaced = l.split(" ");
            spaced.next();

            let mut id = spaced.next().unwrap();
            id = &id[0..id.len() - 1];
            let id = id.parse::<u32>().unwrap();

            Some(id)
        })
        .sum();
}

fn part2(input: String) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
