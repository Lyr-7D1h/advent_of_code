use std::ops::RangeBounds;

use advent_of_code_2023::Aoc;

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
        map.push((source_start..source_start + range_length, destination_start));
    }
    // add last map
    maps.push(map);

    let mut min = u64::MAX;
    for c in seeds.chunks(2).into_iter() {
        let sranges = vec![(c[0], c[0] + c[1])];

        // println!("{c:?}");
        for i in 0..sranges.len() {
            let (smin, smax) = &mut sranges[i];

            'map: for map in maps.iter() {
                for (source_range, dest_start) in map.iter() {
                    if source_range.contains(&smin) {
                        let start = match source_range.start_bound() {
                            std::ops::Bound::Included(start) => start,
                            _ => panic!("no start bound"),
                        };

                        if source_range.contains(&smax) {
                            *smin -=  start + dest_start;
                            *smax -=  start + dest_start;
                            continue 'map;
                        }

                        // smax is bigger than range
                        let end = match source_range.start_bound() {
                            std::ops::Bound::Included(start) => *start,
                            _ => panic!("no start bound"),
                        };
                        sranges.push((end, *smax));
                        *smax = end;
                        continue 'map;

                        // range found go to next map
                    }

                    if source_range.contains(&smax) {
                        let start = match source_range.start_bound() {
                            std::ops::Bound::Included(start) => *start,
                            _ => panic!("no start bound"),
                        };
                        *smax = start;
                    }
                }
            }
        }

        let min_range = sranges
            .into_iter()
            .reduce(|a, b| if a.0 < b.0 { a } else { b })
            .unwrap()
            .0;
        min = min.min(min_range);
    }

    return min;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two_naive", part2_naive);
    aoc.part("two", part2);
    aoc.run();
}
