use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

#[derive(Debug)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    /// Update the boundries of current range if they are overlapping or bordering
    fn update_boundries(&mut self, other: &Range) -> bool {
        let mut updated = false;
        // left side overlaps
        if self.start - 1 <= other.end && self.start >= other.start {
            self.start = other.start;
            updated = true;
        }
        // right side overlaps
        if self.end + 1 >= other.start && self.end <= other.end {
            self.end = other.end;
            updated = true;
        }
        return updated;
    }

    // Check if other is contained within self
    fn contains(&self, other: &Range) -> bool {
        if self.start <= other.start && other.end <= self.end {
            return true;
        }
        return false;
    }
}

/// A set containing only unique ranges
#[derive(Debug)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: vec![] }
    }

    /// Get the length of all intervals togheter
    fn len(&self) -> usize {
        let mut len = 0;
        for r in self.ranges.iter() {
            len += r.start.abs_diff(r.end);
        }
        return len;
    }

    /// Insert a new range into the set or update an existing range if overlapping or bordering
    fn insert(&mut self, range: Range) {
        for i in 0..self.ranges.len() {
            if self.ranges[i].contains(&range) {
                return;
            }
            if self.ranges[i].update_boundries(&range) {
                let updated_interval = self.ranges.remove(i);
                return self.insert(updated_interval);
            }
        }
        self.ranges.push(range);
    }
}

#[derive(Debug)]
struct Sensor {
    position: (isize, isize),
    distance_beacon: usize,
}

#[derive(Debug)]
struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn sensor_ranges_on_row(&self, row: isize) -> RangeSet {
        let mut set = RangeSet::new();

        for sensor in self.sensors.iter() {
            // distance = range distance - relative distance
            let d = sensor.distance_beacon as isize - sensor.position.1.abs_diff(row) as isize;

            if d < 0 {
                continue;
            }

            set.insert(Range {
                start: sensor.position.0 - d,
                end: sensor.position.0 + d,
            });
        }

        return set;
    }

    // find the only position within a range that isn't covered
    fn find_distress_position(&self, lowerbound: isize, upperbound: isize) -> (isize, isize) {
        for row in lowerbound..upperbound {
            let mut set = RangeSet::new();

            for sensor in self.sensors.iter() {
                // distance = range distance - relative distance
                let d = sensor.distance_beacon as isize - sensor.position.1.abs_diff(row) as isize;

                // skip if sensor can't reach the row
                if d < 0 {
                    continue;
                }

                let mut range = Range {
                    start: sensor.position.0 - d,
                    end: sensor.position.0 + d,
                };

                // ensure range is within boundry
                if range.start < lowerbound {
                    range.start = lowerbound
                }
                if range.end > upperbound {
                    range.end = upperbound
                }
                set.insert(range);
            }

            if set.ranges.len() > 1 {
                if set.ranges.len() > 2 {
                    panic!("Multiple distress positions found on row {row}");
                }

                // range with biggest start - 1
                let x = if set.ranges[1].start > set.ranges[0].start {
                    set.ranges[1].start - 1
                } else {
                    set.ranges[0].start - 1
                };

                return (x, row);
            }
        }

        return (0, 0);
    }
}

impl From<Input> for Map {
    fn from(value: Input) -> Self {
        let sensors = value
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut split = l.split(" ").skip(2);
                let sx = split
                    .next()
                    .unwrap()
                    .replace("x=", "")
                    .replace(",", "")
                    .parse()
                    .unwrap();
                let sy = split
                    .next()
                    .unwrap()
                    .replace("y=", "")
                    .replace(":", "")
                    .parse()
                    .unwrap();
                let mut split = split.skip(4);
                let bx = split
                    .next()
                    .unwrap()
                    .replace("x=", "")
                    .replace(",", "")
                    .parse()
                    .unwrap();
                let by = split.next().unwrap().replace("y=", "").parse().unwrap();

                let sensor = Sensor {
                    position: (sx, sy),
                    distance_beacon: sx.abs_diff(bx) + sy.abs_diff(by),
                };

                sensor
            })
            .collect();

        Map { sensors }
    }
}

// 130ns
fn part1(input: Input) -> usize {
    return Map::from(input).sensor_ranges_on_row(2000000).len();
}

// 4.9s
fn part2(input: Input) -> isize {
    let (x, y) = Map::from(input).find_distress_position(0, 4000000);
    return x * 4000000 + y;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
