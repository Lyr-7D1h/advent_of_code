#![feature(iterator_try_collect)]
use advent_of_code_2022::{Aoc, Input};
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

// 9ms
fn part1(input: Input) -> u32 {
    let positions: HashSet<(usize, usize, usize)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let pos: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            (pos[0], pos[1], pos[2])
        })
        .collect();

    let mut surface_area = 0;
    for (x, y, z) in positions.iter() {
        let mut area = 6;
        if positions.contains(&(x + 1, *y, *z)) {
            area -= 1;
        }
        if x != &0 && positions.contains(&(x - 1, *y, *z)) {
            area -= 1;
        }
        if positions.contains(&(*x, y + 1, *z)) {
            area -= 1;
        }
        if y != &0 && positions.contains(&(*x, y - 1, *z)) {
            area -= 1;
        }
        if positions.contains(&(*x, *y, z + 1)) {
            area -= 1;
        }
        if z != &0 && positions.contains(&(*x, *y, z - 1)) {
            area -= 1;
        }

        surface_area += area;
    }

    return surface_area;
}

/// total surface area - total air pocket area
/// for some reason does not work
fn part2(input: Input) -> u32 {
    let positions: HashSet<(usize, usize, usize)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let pos: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            (pos[0], pos[1], pos[2])
        })
        .collect();

    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;
    let mut z_min = usize::MAX;
    let mut z_max = 0;
    for (x, y, z) in positions.iter() {
        if x < &x_min {
            x_min = *x;
        }
        if y < &y_min {
            y_min = *y
        }
        if z < &z_min {
            z_min = *z;
        }

        if x > &x_max {
            x_max = *x;
        }
        if y > &y_max {
            y_max = *y;
        }
        if z > &z_max {
            z_max = *z;
        }
    }

    let mut air_pockets = HashSet::new();

    let mut surface_area = 0;
    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            'n: for z in z_min..z_max + 1 {
                if positions.contains(&(x, y, z)) {
                    let mut area = 6;
                    for n in neighbors((x, y, z)).into_iter() {
                        if positions.contains(&n) {
                            area -= 1;
                        }
                    }
                    surface_area += area;
                } else {
                    for n in neighbors((x, y, z)) {
                        if !positions.contains(&n) {
                            continue 'n;
                        }
                    }
                    println!("{x} {y} {z}");
                    air_pockets.insert((x, y, z));
                }
            }
        }
    }

    let mut surface_area_airpockets = 0;
    for c in air_pockets.iter() {
        let mut area = 6;
        for n in neighbors(*c).into_iter() {
            if air_pockets.contains(&n) {
                area -= 1;
            }
        }
        surface_area_airpockets += area;
    }

    return surface_area - surface_area_airpockets;
}

fn neighbors(pos: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let (x, y, z) = pos;
    let mut r = vec![(x + 1, y, z), (x, y + 1, z), (x, y, z + 1)];

    if x > 0 {
        r.push((x - 1, y, z))
    }
    if y > 0 {
        r.push((x, y - 1, z))
    }
    if z > 0 {
        r.push((x, y, z - 1))
    }
    return r;
}

// 50ms
fn part2_flood_fill(input: Input) -> u32 {
    let positions: HashSet<(usize, usize, usize)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let pos: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap() + 1) // shift everything up by 1
                .collect();

            (pos[0], pos[1], pos[2])
        })
        .collect();

    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;
    let mut z_min = usize::MAX;
    let mut z_max = 0;
    for (x, y, z) in positions.iter() {
        if x < &x_min {
            x_min = *x;
        }
        if y < &y_min {
            y_min = *y
        }
        if z < &z_min {
            z_min = *z;
        }

        if x > &x_max {
            x_max = *x;
        }
        if y > &y_max {
            y_max = *y;
        }
        if z > &z_max {
            z_max = *z;
        }
    }

    // bounding box +1
    x_min -= 1;
    y_min -= 1;
    z_min -= 1;

    x_max += 1;
    y_max += 1;
    z_max += 1;

    let mut queue = VecDeque::from([(x_min, y_min, z_min)]);
    let mut visited = HashSet::from([(x_min, y_min, z_min)]);

    let mut exterior_area = 0;
    while let Some(n) = queue.pop_front() {
        for neighbor in neighbors(n).into_iter() {
            if x_min > neighbor.0
                || neighbor.0 > x_max + 1
                || y_min > neighbor.1
                || neighbor.1 > y_max + 1
                || z_min > neighbor.2
                || neighbor.2 > z_max + 1
            {
                continue;
            }

            if positions.contains(&neighbor) {
                exterior_area += 1;
            } else if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    return exterior_area;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.part("two_flood_fill", part2_flood_fill);
    aoc.run();
}
