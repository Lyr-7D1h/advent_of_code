use advent_of_code_2023::Aoc;

/// 180.469µs
fn part1(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut empty_rows = vec![true; map.len()];
    let mut empty_cols = vec![true; map[0].len()];

    let mut positions: Vec<(usize, usize)> = map
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.into_iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        empty_rows[y] = false;
                        empty_cols[x] = false;
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    // calculate expansion
    for (x, y) in positions.iter_mut() {
        for is_empty in &empty_cols[0..*x] {
            if *is_empty {
                *x += 1;
            }
        }
        for is_empty in &empty_rows[0..*y] {
            if *is_empty {
                *y += 1;
            }
        }
    }

    let mut sum = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (ax, ay) = positions[i];
            let (bx, by) = positions[j];

            sum += ax.abs_diff(bx) + ay.abs_diff(by);
        }
    }

    return sum;
}

/// 178.428µs
fn part2(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut empty_rows = vec![true; map.len()];
    let mut empty_cols = vec![true; map[0].len()];

    let mut positions: Vec<(usize, usize)> = map
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.into_iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        empty_rows[y] = false;
                        empty_cols[x] = false;
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    // calculate expansion
    for (x, y) in positions.iter_mut() {
        for is_empty in &empty_cols[0..*x] {
            if *is_empty {
                *x += 999999;
            }
        }
        for is_empty in &empty_rows[0..*y] {
            if *is_empty {
                *y += 999999;
            }
        }
    }

    let mut sum = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (ax, ay) = positions[i];
            let (bx, by) = positions[j];

            sum += ax.abs_diff(bx) + ay.abs_diff(by);
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
