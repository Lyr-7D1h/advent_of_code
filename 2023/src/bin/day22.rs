use std::collections::{HashSet, VecDeque};

use advent_of_code_2023::Aoc;

#[derive(Debug)]
struct Brick {
    id: usize,
    sx: usize,
    ex: usize,
    sy: usize,
    ey: usize,
    height: usize,
}

impl Brick {
    /// check if another brick overlaps on xy-plane
    fn overlaps(&self, b: &Brick) -> bool {
        if self.sx <= b.ex && self.ex >= b.sx && self.sy <= b.ey && self.ey >= b.sy {
            true
        } else {
            false
        }
    }
}

fn parse(input: String) -> Vec<Vec<Brick>> {
    let mut bricks: Vec<(usize, Brick)> = input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(id, l)| {
            let mut parts = l.split("~");
            let a: Vec<usize> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let b: Vec<usize> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            // split bricks into multiple parts in case of dz
            let height = a[2].abs_diff(b[2]) + 1;
            (0..height).map(move |i| {
                (
                    a[2] + i,
                    Brick {
                        id,
                        sx: a[0],
                        ex: b[0],
                        sy: a[1],
                        ey: b[1],
                        height,
                    },
                )
            })
        })
        .flatten()
        .collect();

    // sort by z axis
    bricks.sort_by(|(az, _), (bz, _)| az.cmp(bz));

    // build height map
    let mut height_map: Vec<Vec<Brick>> = vec![];
    'n: for (_, brick) in bricks {
        // from top to bottom, if overlaps put it above
        for (z, level) in height_map.iter().enumerate().rev() {
            // println!("{z} {level:?}");
            if level.iter().any(|b| brick.overlaps(b)) {
                match height_map.get_mut(z + 1) {
                    Some(bricks) => bricks.push(brick),
                    None => height_map.push(vec![brick]),
                }
                continue 'n;
            }
        }
        // if no overlap found in current levels add to ground
        match height_map.get_mut(0) {
            Some(bricks) => bricks.push(brick),
            None => height_map.push(vec![brick]),
        }
    }

    return height_map;
}

/// Average Duration: 507.148µs
fn part1(input: String) -> u32 {
    let height_map = parse(input);

    // safe to disintegrate
    // - if no brick on top
    // - if there is more than one brick supporting top
    let mut count = 0;
    for z in 0..height_map.len() {
        for brick in &height_map[z] {
            match height_map.get(z + 1) {
                Some(top_bricks) => {
                    // check for every top brick if it has multiple supporting bricks
                    if top_bricks
                        .iter()
                        .filter(|b| b.overlaps(brick))
                        .all(|top| height_map[z].iter().filter(|b| b.overlaps(top)).count() >= 2)
                    {
                        count += 1;
                    }
                }
                // no level on top so safe
                None => count += 1,
            }
        }
    }

    return count;
}

/// Average Duration: 50.389189ms
fn part2(input: String) -> usize {
    let height_map = parse(input);

    // disintegrate
    let mut count = 0;
    for z in 0..height_map.len() {
        for brick in &height_map[z] {
            // for all in this set check which top ones would fall
            // only count unique bricks
            let mut set = HashSet::new();

            let mut queue = VecDeque::new();
            queue.push_front((z, brick));

            while let Some((z, b)) = queue.pop_front() {
                // println!("SET {set:?}");
                if set.contains(&b.id) {
                    continue;
                }
                set.insert(b.id);

                match height_map.get(z + 1) {
                    Some(top_bricks) => {
                        // println!("{top_bricks:?}");
                        let top_bricks: Vec<(usize, &Brick)> = top_bricks
                            .iter()
                            .filter_map(|top| {
                                // if this top brick overlaps only with the current bricks count
                                if height_map[z]
                                    .iter()
                                    // get all children for this brick
                                    .filter(|b| b.overlaps(top))
                                    // all children have to be in the removed set
                                    .all(|b| set.contains(&b.id))
                                {
                                    return Some((z + top.height, top));
                                }
                                return None;
                            })
                            .collect();
                        // println!("A {top:?}");
                        queue.extend(top_bricks);
                    }
                    // no level on top so safe
                    None => {}
                }
            }

            // without current brick
            count += set.len() - 1;
        }
    }

    return count;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
