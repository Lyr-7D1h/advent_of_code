use advent_of_code_2022::{Aoc, Input};
use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
struct Sensor {
    position: (isize, isize),
    distance_beacon: usize,
}

#[derive(Debug)]
struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<(isize, isize)>,
}

impl Map {
    /// Count the amount of positions where the position is in the range of a sensor
    fn count_sensor_ranges_on_row(&self, row: isize) -> usize {
        // get all relevant sensors that can reach that row
        let sensors_in_range: Vec<&Sensor> = self
            .sensors
            .iter()
            .filter(|s| {
                s.position.1 - s.distance_beacon as isize <= row
                    && row <= s.position.1 + s.distance_beacon as isize
            })
            .collect();

        // sort by x coordinate
        // sensors_in_range.sort_by(|a, b| a.position.0.cmp(&b.position.0));

        let mut scanned = HashSet::new();

        for sensor in sensors_in_range.iter() {
            // distance = range distance - relative distance
            let d = sensor.distance_beacon - sensor.position.1.abs_diff(row);

            // insert midpoint of range
            scanned.insert(sensor.position.0);

            // insert blocks based on how close the row is to the range
            for i in 1..d + 1 {
                scanned.insert(sensor.position.0 - i as isize);
                scanned.insert(sensor.position.0 + i as isize);
            }
        }

        // remove existing beacons
        for beacon in self.beacons.iter() {
            if beacon.1 == row {
                scanned.remove(&beacon.0);
            }
        }

        return scanned.len();
    }

    fn find_distress_position(&self, lowerbound: isize, upperbound: isize) -> (isize, isize) {
        for row in lowerbound..upperbound {}

        return (0, 0);
    }
}

impl From<Input> for Map {
    fn from(value: Input) -> Self {
        let mut beacons = HashSet::new();
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

                beacons.insert((bx, by));

                sensor
            })
            .collect();
        // println!("{sensors:?}");

        Map {
            sensors,
            beacons: beacons.into_iter().collect(),
        }
    }
}

fn part1(input: Input) -> usize {
    return Map::from(input).count_sensor_ranges_on_row(2000000);
}

fn part2(input: Input) -> isize {
    let (x, y) = Map::from(input).find_distress_position(0, 20);

    return x * 4000000 + y;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
