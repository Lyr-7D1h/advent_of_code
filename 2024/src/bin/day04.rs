use advent_of_code_2023::Aoc;

const DIRECTION: [(i8, i8); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];
const WORD: &'static str = "MAS";
// Average Duration: 382.064µs
fn part1(input: String) -> u32 {
    let word: Vec<char> = WORD.chars().collect();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'X' {
                'n: for (dx, dy) in DIRECTION.iter() {
                    for i in 0..WORD.len() {
                        let c = (i + 1) as i32;
                        let y = y as i32 + *dy as i32 * c;
                        let x = x as i32 + *dx as i32 * c;
                        if x < 0 || y < 0 || y >= map.len() as i32 || x >= map[0].len() as i32 {
                            continue 'n;
                        }
                        if map[y as usize][x as usize] != word[i] {
                            continue 'n;
                        }
                    }
                    sum += 1;
                }
            }
        }
    }

    return sum;
}

// Average Duration: 137.578µs
fn part2(input: String) -> u32 {
    let direction: Vec<((i32, i32), (i32, i32))> = vec![((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut sum = 0;
    for y in 0..map.len() {
        'n: for x in 0..map[0].len() {
            if map[y][x] == 'A' {
                for (d1, d2) in direction.iter() {
                    let x1 = x as i32 + d1.0;
                    let y1 = y as i32 + d1.1;
                    if x1 < 0 || y1 < 0 || x1 >= map.len() as i32 || y1 >= map[0].len() as i32 {
                        continue 'n;
                    }
                    let x2 = x as i32 + d2.0;
                    let y2 = y as i32 + d2.1;
                    if x2 < 0 || y2 < 0 || x2 >= map.len() as i32 || y2 >= map[0].len() as i32 {
                        continue 'n;
                    }
                    if (map[y1 as usize][x1 as usize] == 'M'
                        && map[y2 as usize][x2 as usize] == 'S')
                        || (map[y1 as usize][x1 as usize] == 'S'
                            && map[y2 as usize][x2 as usize] == 'M')
                    {
                        continue;
                    }
                    continue 'n;
                }
                sum += 1
            }
        }
    }

    return sum;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
