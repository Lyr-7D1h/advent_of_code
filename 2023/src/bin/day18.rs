use std::collections::HashSet;

use advent_of_code_2023::Aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// using a flood fill over possible intirior or exterior tiles
/// 1.135577ms
fn part1(input: String) -> i32 {
    let mut coords: Vec<((i32, i32), (i32, i32))> = vec![];
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut xmax, mut xmin, mut ymax, mut ymin) = (0, 0, 0, 0);

    let mut tagged = HashSet::new();
    let mut boundry_count = 0;

    for line in input.lines() {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap().chars().next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        boundry_count += value;

        let start = (x, y);
        match dir {
            'R' => {
                for x in x..x + value {
                    tagged.insert((x, y + 1));
                }
                x += value;
                xmax = xmax.max(x);
            }
            'L' => {
                for x in x - value..x {
                    tagged.insert((x, y - 1));
                }
                x -= value;
                xmin = xmin.min(x);
            }
            'D' => {
                for y in y..y + value {
                    tagged.insert((x - 1, y));
                }
                y += value;
                ymax = ymax.max(y)
            }
            'U' => {
                for y in y - value..y {
                    tagged.insert((x + 1, y));
                }
                y -= value;
                ymin = ymin.min(y)
            }
            _ => panic!(),
        }
        coords.push((start, (x, y)));
    }

    // create a n by n map
    let mut map = vec![vec![false; (xmax - xmin + 1) as usize]; (ymax - ymin + 1) as usize];
    for ((x, y), (ax, ay)) in coords {
        let y = (y - ymin) as usize;
        let ay = (ay - ymin) as usize;
        let x = (x - xmin) as usize;
        if y == ay {
            let ax = (ax - xmin) as usize;
            let (start, end) = if x > ax { (ax, x) } else { (x, ax) };
            for x in start..end + 1 {
                map[y][x] = true;
            }
            continue;
        }
        let (start, end) = if y > ay { (ay, y) } else { (y, ay) };
        for y in start..end + 1 {
            map[y][x] = true;
        }
    }

    // flood fill all tagged coords
    let mut count = 0;
    let mut outside = false;
    for (x, y) in tagged {
        let (x, y) = ((x - xmin) as usize, (y - ymin) as usize);
        match map.get(y).map(|l| l.get(x)) {
            Some(Some(true)) => continue,
            Some(Some(false)) => {
                let mut queue = vec![(x, y)];
                while let Some((x, y)) = queue.pop() {
                    if !map[y][x] {
                        count += 1;
                        map[y][x] = true;
                    }
                    for d in [
                        Direction::North,
                        Direction::South,
                        Direction::East,
                        Direction::West,
                    ] {
                        let (x, y) = match d {
                            Direction::North if y == 0 => continue,
                            Direction::North => (x, y - 1),
                            Direction::South if y == map.len() - 1 => continue,
                            Direction::South => (x, y + 1),
                            Direction::West if x == 0 => continue,
                            Direction::West => (x - 1, y),
                            Direction::East if x == map[0].len() - 1 => continue,
                            Direction::East => (x + 1, y),
                        };
                        if map[y][x] {
                            continue;
                        }
                        queue.push((x, y))
                    }
                }
            }
            _ => outside = true,
        }
    }

    // let map = map
    //     .into_iter()
    //     .map(|l| {
    //         l.into_iter()
    //             .map(|b| if b { '#' } else { '.' })
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // println!("{map}");

    if outside {
        return (xmax - xmin + 1) * (ymax - ymin + 1) - count;
    }

    count + boundry_count
}

/// using the Trapezoid formula for counting area
/// 39.665µs
fn part1_hlines(input: String) -> i32 {
    let mut hlines: Vec<(i32, (i32, i32))> = vec![];
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut ymax, mut ymin) = (0, 0);

    // if pos intirior is on the right side if neg it is on the right side
    let mut orientation = 0;
    // should equal direction of the last instruction
    let mut last_dir = Direction::North;

    let mut left_corners = vec![];
    let mut right_corners = vec![];

    for line in input.lines() {
        let mut parts = line.split(" ");
        let dir = match parts.next().unwrap().chars().next().unwrap() {
            'R' => Direction::East,
            'L' => Direction::West,
            'U' => Direction::North,
            'D' => Direction::South,
            _ => panic!(),
        };
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        match dir {
            Direction::East => {
                let index = hlines.len();
                if last_dir == Direction::North {
                    right_corners.push((index, 0));
                    orientation += 1;
                } else {
                    left_corners.push((index, 0));
                    orientation -= 1;
                }

                let ox = x;
                x += value as i32;
                hlines.push((y, (ox, x)));
            }
            Direction::West => {
                let index = hlines.len();
                if last_dir == Direction::North {
                    left_corners.push((index, 1));
                    orientation -= 1;
                } else {
                    right_corners.push((index, 1));
                    orientation += 1;
                }

                let ox = x;
                x -= value as i32;
                hlines.push((y, (ox, x)));
            }
            Direction::North => {
                let index = hlines.len().saturating_sub(1);
                if last_dir == Direction::East {
                    left_corners.push((index, 1));
                    orientation -= 1;
                } else {
                    right_corners.push((index, 0));
                    orientation += 1;
                }

                y -= value;
                ymin = ymin.min(y);
            }
            Direction::South => {
                let index = hlines.len().saturating_sub(1);
                if last_dir == Direction::East {
                    right_corners.push((index, 1));
                    orientation += 1;
                } else {
                    left_corners.push((index, 0));
                    orientation -= 1;
                }

                y += value;
                ymax = ymax.max(y);
            }
        }
        last_dir = dir;
    }
    debug_assert!(orientation == -4 || orientation == 4);
    let inner_corners = if orientation == 4 {
        left_corners
    } else {
        right_corners
    };

    for (index, p) in inner_corners {
        let r = &mut hlines[index].1;
        if p == 0 {
            if r.0 < r.1 {
                r.0 += 1;
            } else {
                r.1 += 1;
            }
        } else {
            if r.0 > r.1 {
                r.0 -= 1;
            } else {
                r.1 -= 1;
            }
        }
    }

    // tried a very complicated boundry collapsing algortihm
    // collapse each horizontal boundry onto a lower one and count
    // let mut count = 0;
    // hlines.sort_by(|a, b| a.0.cmp(&b.0));
    // hlines.reverse();
    // println!("{hlines:?}");
    // while let Some((from, (mut a, mut b))) = hlines.pop() {
    //     // println!("");
    //     // println!("{hlines:?}");
    //     // println!("({from}) {a} - {b} ({count})");
    //     if b < a {
    //         continue;
    //     }
    //
    //     // for each range in current layer
    //     for (i, (till, (mut aa, mut bb))) in hlines.clone().into_iter().enumerate().rev() {
    //         debug_assert!(a <= b);
    //         // println!("{aa} {bb}      {hlines:?}");
    //         // remove range if invalid
    //         if bb < aa {
    //             hlines.remove(i);
    //             continue;
    //         }
    //
    //         // aabb overlaps ab on its right side
    //         if aa <= a && (bb >= a && bb <= b) {
    //             println!("B {a} - {b}     |       {aa} - {bb}");
    //             count += (bb.abs_diff(a) as i32 + 1) * (till - from + 1);
    //             let oa = a;
    //             a = bb + 1;
    //             bb = oa - 1;
    //             // println!(" AAA {aa} {bb}");
    //             println!("A {a} - {b}     |       {aa} - {bb}");
    //         }
    //         // aabb overlaps ab on its left side
    //         else if bb >= b && (aa >= a && aa <= b) {
    //             count += (aa.abs_diff(b) as i32 + 1) * (till - from + 1);
    //             let ob = b;
    //             b = aa - 1;
    //             aa = ob + 1;
    //         }
    //         // ab fits entirely
    //         else if aa <= a && bb >= b {
    //             count += (b - a + 1) * (till - from + 1);
    //             // split aabb in two
    //             let end = (b + 1, bb);
    //             bb = a - 1;
    //             // invalidate ab
    //             b = a - 1;
    //             hlines.insert(i + 1, (till, end));
    //             // println!("{hlines:?}");
    //         }
    //         // aabb fits entirely
    //         else if a <= aa && b >= bb {
    //             count += (bb - aa + 1) * (till - from + 1);
    //             // split ab in two
    //             b = aa - 1;
    //             hlines.push((from, (bb + 1, b)));
    //
    //             // invalidate aabb
    //             bb = aa - 1;
    //         }
    //
    //         // remove range if invalid
    //         if bb < aa {
    //             hlines.remove(i);
    //         // otherwise update
    //         } else {
    //             hlines[i].1 = (aa, bb);
    //         }
    //
    //         // if ab range is invalid it is consumed
    //         if b < a {
    //             break;
    //         }
    //     }
    //     // every top must contain a buttom
    //     println!("{hlines:?}");
    //     println!("{a} - {b}");
    //     debug_assert!(b < a);
    // }

    let mut count = 0;
    // modified Trapezoid formula (https://en.wikipedia.org/wiki/Shoelace_formula)
    for (mut y, (x, ax)) in hlines {
        y -= ymin;
        if x < ax {
            let xdiff = ax - x + 1;
            count += xdiff * (ymax - y + 1);
        } else {
            let xdiff = x - ax + 1;
            count -= xdiff * (ymax - y);
        }
    }

    count
}

/// Same as one_hlines with modified input
/// 56.202µs
fn part2(input: String) -> i64 {
    let mut hlines: Vec<(i64, (i64, i64))> = vec![];
    let (mut x, mut y) = (0, 0);
    let (mut ymax, mut ymin) = (0, 0);

    // if pos intirior is on the right side if neg it is on the right side
    let mut orientation = 0;
    // should equal direction of the last instruction
    let mut last_dir = Direction::North;

    let mut left_corners = vec![];
    let mut right_corners = vec![];

    for line in input.lines() {
        let hex = line.split(" ").skip(2).next().unwrap();
        let value = i64::from_str_radix(&hex[2..7], 16).unwrap();
        let dir = match hex.chars().skip(7).next().unwrap() {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => panic!(),
        };
        match dir {
            Direction::East => {
                let index = hlines.len();
                if last_dir == Direction::North {
                    right_corners.push((index, 0));
                    orientation += 1;
                } else {
                    left_corners.push((index, 0));
                    orientation -= 1;
                }

                let ox = x;
                x += value;
                hlines.push((y, (ox, x)));
            }
            Direction::West => {
                let index = hlines.len();
                if last_dir == Direction::North {
                    left_corners.push((index, 1));
                    orientation -= 1;
                } else {
                    right_corners.push((index, 1));
                    orientation += 1;
                }

                let ox = x;
                x -= value;
                hlines.push((y, (ox, x)));
            }
            Direction::North => {
                let index = hlines.len().saturating_sub(1);
                if last_dir == Direction::East {
                    left_corners.push((index, 1));
                    orientation -= 1;
                } else {
                    right_corners.push((index, 0));
                    orientation += 1;
                }

                y -= value;
                ymin = ymin.min(y);
            }
            Direction::South => {
                let index = hlines.len().saturating_sub(1);
                if last_dir == Direction::East {
                    right_corners.push((index, 1));
                    orientation += 1;
                } else {
                    left_corners.push((index, 0));
                    orientation -= 1;
                }

                y += value;
                ymax = ymax.max(y);
            }
        }
        last_dir = dir;
    }
    debug_assert!(orientation == -4 || orientation == 4);

    // remove inner corners based on orientation
    let inner_corners = if orientation == 4 {
        left_corners
    } else {
        right_corners
    };
    for (index, p) in inner_corners {
        let r = &mut hlines[index].1;
        // increase lower bound
        if p == 0 {
            if r.0 < r.1 {
                r.0 += 1;
            } else {
                r.1 += 1;
            }
        // decreases upper bound
        } else {
            if r.0 > r.1 {
                r.0 -= 1;
            } else {
                r.1 -= 1;
            }
        }
    }

    let mut count = 0;
    // modified Trapezoid formula (https://en.wikipedia.org/wiki/Shoelace_formula)
    for (mut y, (x, ax)) in hlines {
        y -= ymin;
        if x < ax {
            let xdiff = ax - x + 1;
            count += xdiff * (ymax - y + 1);
        } else {
            let xdiff = x - ax + 1;
            count -= xdiff * (ymax - y);
        }
    }

    count
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("one_hlines", part1_hlines);
    aoc.part("two", part2);
    aoc.run();
}
