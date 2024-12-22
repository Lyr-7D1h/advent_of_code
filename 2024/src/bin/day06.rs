use advent_of_code_2023::Aoc;

pub enum Direction {
    North,
    South,
    East,
    West,
}
//Average Duration: 52.172µs
fn part1(input: String) -> u32 {
    let mut direction: (i32, i32) = (0, -1);
    let mut position: (i32, i32) = (0, 0);
    let mut map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            let chars = l.chars();
            if let Some((x, _)) = chars.clone().enumerate().find(|c| c.1 == '^') {
                position = (x as i32, y as i32);
            }
            chars.collect()
        })
        .collect();

    let mut count = 1;
    loop {
        let (mut x, mut y) = position;
        x += direction.0;
        y += direction.1;
        if x < 0 || y < 0 || (x as usize) >= map[0].len() || (y as usize) >= map.len() {
            break;
        }
        if map[y as usize][x as usize] == '#' {
            direction = (-direction.1, direction.0);
            continue;
        }
        position = (x, y);

        if map[y as usize][x as usize] != 'X' {
            count += 1;
        }
        map[y as usize][x as usize] = 'X';
    }
    for l in map {
        let s: String = l.into_iter().collect();
        println!("{s}");
    }
    return count;
}

fn part2(input: String) -> u32 {}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
