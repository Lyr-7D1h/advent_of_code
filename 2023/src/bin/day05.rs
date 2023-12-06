use std::ops::RangeBounds;

use advent_of_code_2023::Aoc;

/// 21.605µs
fn part1(input: String) -> u64 {
    let mut lines = input.lines();

    let seed_string = lines.next().unwrap();
    let seeds: Vec<u64> = seed_string[7..seed_string.len()]
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps = vec![];
    let mut map = vec![];
    let mut lines = lines.skip(2);
    while let Some(l) = lines.next() {
        if l.len() == 0 {
            maps.push(map);
            map = vec![];
            lines.next();
            continue;
        }

        let mut parts = l.split(" ");
        let destination_start: u64 = parts.next().unwrap().parse().unwrap();
        let source_start: u64 = parts.next().unwrap().parse().unwrap();
        let range_length: u64 = parts.next().unwrap().parse().unwrap();
        map.push((source_start..source_start + range_length, destination_start));
    }
    // add last map
    maps.push(map);

    let mut min = u64::MAX;
    for mut s in seeds {
        'map: for map in maps.iter() {
            for (source_range, dest_start) in map.iter() {
                if source_range.contains(&s) {
                    let start = match source_range.start_bound() {
                        std::ops::Bound::Included(start) => start,
                        _ => panic!("no start bound"),
                    };
                    s = dest_start + s - start;
                    // range found go to next map
                    continue 'map;
                }
            }
        }

        min = min.min(s);
    }

    return min;
}

/// 130s
/// brute force solution
fn part2_naive(input: String) -> u64 {
    let mut lines = input.lines();

    let seed_string = lines.next().unwrap();
    let seeds: Vec<u64> = seed_string[7..seed_string.len()]
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps = vec![];
    let mut map = vec![];
    let mut lines = lines.skip(2);
    while let Some(l) = lines.next() {
        if l.len() == 0 {
            maps.push(map);
            map = vec![];
            lines.next();
            continue;
        }

        let mut parts = l.split(" ");
        let destination_start: u64 = parts.next().unwrap().parse().unwrap();
        let source_start: u64 = parts.next().unwrap().parse().unwrap();
        let range_length: u64 = parts.next().unwrap().parse().unwrap();
        map.push((source_start..source_start + range_length, destination_start));
    }
    // add last map
    maps.push(map);

    let mut min = u64::MAX;
    for c in seeds.chunks(2).into_iter() {
        // println!("{c:?}");
        for mut s in c[0]..c[0] + c[1] {
            'map: for map in maps.iter() {
                for (source_range, dest_start) in map.iter() {
                    if source_range.contains(&s) {
                        let start = match source_range.start_bound() {
                            std::ops::Bound::Included(start) => start,
                            _ => panic!("no start bound"),
                        };
                        s = dest_start + s - start;
                        // range found go to next map
                        continue 'map;
                    }
                }
            }
            // println!("{s}");
            min = min.min(s);
        }
    }

    return min;
}

/// 31.841µs
/// mapping boundries and splitting ranges
fn part2(input: String) -> u64 {
    let mut lines = input.lines();

    let seed_string = lines.next().unwrap();
    let seeds: Vec<u64> = seed_string[7..seed_string.len()]
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut maps = vec![];
    let mut map = vec![];
    let mut lines = lines.skip(2);
    while let Some(l) = lines.next() {
        if l.len() == 0 {
            maps.push(map);
            map = vec![];
            lines.next();
            continue;
        }

        let mut parts = l.split(" ");
        let destination_start: u64 = parts.next().unwrap().parse().unwrap();
        let source_start: u64 = parts.next().unwrap().parse().unwrap();
        let range_length: u64 = parts.next().unwrap().parse().unwrap();
        map.push((
            source_start,
            source_start + range_length - 1,
            destination_start,
        ));
    }
    // add last map
    maps.push(map);

    let mut ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|c| (c[0], c[0] + c[1] - 1)).collect();
    for map in maps.into_iter() {
        let mut queue: Vec<usize> = (0..ranges.len()).collect();

        'range: while let Some(i) = queue.pop() {
            for (start, end, dest_start) in map.iter().cloned() {
                let (smin, smax) = ranges[i];
                if start <= smin && smin <= end {
                    // change whole range by offset
                    if smax <= end {
                        ranges[i].0 = smin - start + dest_start;
                        ranges[i].1 = smax - start + dest_start;
                        // assert!(ranges[i].0 < ranges[i].1);
                        continue 'range;
                    }

                    // smax is bigger than range so split into two ranges
                    ranges[i].0 = smin - start + dest_start;
                    ranges[i].1 = end - start + dest_start;
                    // assert!(ranges[i].0 < ranges[i].1);

                    // add new range
                    ranges.push((end + 1, smax));
                    queue.push(ranges.len() - 1);
                    // assert!(ranges[ranges.len() - 1].0 < ranges[ranges.len() - 1].1);

                    continue 'range;
                }

                if start <= smax && smax <= end {
                    ranges[i].0 = dest_start;
                    ranges[i].1 = smax - start + dest_start;
                    // assert!(ranges[i].0 < ranges[i].1);

                    ranges.push((smin, start - 1));
                    queue.push(ranges.len() - 1);
                    // assert!(ranges[ranges.len() - 1].0 < ranges[ranges.len() - 1].1);
                    continue 'range;
                }
            }
        }
    }

    let min_range = ranges
        .into_iter()
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .unwrap()
        .0;

    return min_range;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two_naive", part2_naive);
    aoc.part("two", part2);
    aoc.run();
}
