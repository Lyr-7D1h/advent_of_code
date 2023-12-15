use std::collections::HashMap;

use advent_of_code_2023::Aoc;

/// 44.75µ
fn part1(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for x in 0..map[0].len() {
        let mut top = 0;
        for y in 0..map.len() {
            match map[y][x] {
                'O' => {
                    sum += map.len() - top;
                    top += 1;
                }
                '#' => top = y + 1,
                _ => {}
            }
        }
    }

    sum
}

/// debug function to print map
fn print_map(balls: &Vec<(usize, usize)>, rows: &Vec<Vec<usize>>, col_len: usize) {
    let mut map: Vec<Vec<char>> = rows
        .iter()
        .map(|r| {
            let mut l = vec!['.'; col_len];
            for x in r.iter().cloned() {
                l[x] = '#'
            }
            l
        })
        .collect();

    for (x, y) in balls.iter().cloned() {
        map[y][x] = 'O'
    }

    println!();
    for l in map {
        println!("{}", l.into_iter().collect::<String>());
    }
}

/// 30.941063ms
fn part2(input: String) -> u32 {
    // all positions of rollebable balls
    let mut balls: Vec<(usize, usize)> = vec![];
    let mut rows: Vec<Vec<usize>> = vec![];
    let mut columns: Vec<Vec<usize>> = vec![];
    for (y, l) in input.lines().enumerate() {
        if y == 0 {
            for _ in 0..l.len() {
                columns.push(vec![]);
            }
        }
        rows.push(vec![]);
        for (x, c) in l.chars().enumerate() {
            match c {
                'O' => balls.push((x, y)),
                '#' => {
                    rows[y].push(x);
                    columns[x].push(y);
                }
                _ => {}
            }
        }
    }
    let max = rows.len().max(columns.len());

    let mut cache: HashMap<Vec<(usize, usize)>, u32> = HashMap::new();
    let mut i: u32 = 1;
    loop {
        for direction in 0..4 {
            let mut limits = vec![0; max];
            let mut next = vec![0; max];

            let reversed = direction >= 2;
            let (lookup, swap, lookup_max) = if direction % 2 == 0 {
                // north and south
                balls.sort_by(|(_, ay), (_, by)| if reversed { by.cmp(ay) } else { ay.cmp(by) });
                (&columns, false, rows.len() - 1)
            } else {
                // east and west
                balls.sort_by(|(ax, _), (bx, _)| if reversed { bx.cmp(ax) } else { ax.cmp(bx) });
                (&rows, true, columns.len() - 1)
            };

            for (x, y) in &mut balls {
                let (x, y) = if swap { (y, x) } else { (x, y) };

                if reversed {
                    while next[*x] < lookup[*x].len() {
                        // the highest # in this column
                        let n = lookup[*x][lookup[*x].len() - 1 - next[*x]];
                        // if # is higher then y set a new limit for this column
                        if n > *y {
                            limits[*x] = lookup_max - n + 1;
                            next[*x] += 1;
                            continue;
                        }
                        break;
                    }
                    *y = lookup_max - limits[*x];
                    limits[*x] = limits[*x] + 1;
                } else {
                    while let Some(n) = lookup[*x].get(next[*x]) {
                        if n < y {
                            limits[*x] = *n + 1;
                            next[*x] += 1;
                            continue;
                        }
                        break;
                    }
                    *y = limits[*x];
                    limits[*x] += 1;
                }
            }
        }

        if let Some(start_cycle) = cache.get(&balls) {
            let cycle = i - start_cycle;
            // calculate whatever index it is on after n times
            let left = (1000000000 - start_cycle) % cycle;
            // the index after n iterations
            let i: u32 = start_cycle + left;
            // set balls at this index
            balls = cache.into_iter().find(|(_, v)| *v == i).unwrap().0.clone();
            break;
        } else {
            cache.insert(balls.clone(), i);
        }
        i += 1;
    }

    // get score from north
    balls
        .into_iter()
        .fold(0, |a, (_, y)| a + (columns.len() - y) as u32)
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
