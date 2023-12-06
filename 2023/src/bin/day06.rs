use advent_of_code_2023::Aoc;

// 400ns
fn part1(input: String) -> f64 {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let times: Vec<f64> = times[11..times.len()]
        .split(" ")
        .filter_map(|t| {
            if t.len() == 0 {
                None
            } else {
                Some(t.parse().unwrap())
            }
        })
        .collect();
    let distances = lines.next().unwrap();
    let distances: Vec<f64> = distances[11..distances.len()]
        .split(" ")
        .filter_map(|t| {
            if t.len() == 0 {
                None
            } else {
                Some(t.parse().unwrap())
            }
        })
        .collect();

    let mut sum: f64 = 1.0;
    for i in 0..times.len() {
        let ttotal = times[i];
        let distance = distances[i];

        // solve: t^2 - {ttotal}t  + {distance} = 0
        // where t is time pressed on the button to accelerate

        let d = ((ttotal.powi(2) - 4.0 * distance) as f64).sqrt();
        let x_min = (((ttotal - d) / 2.0) + 1.).floor();
        let x_max = (((ttotal + d) / 2.0) - 1.).ceil();
        sum *= x_max - x_min + 1.0;
    }

    return sum;
}

// 380ns
fn part2(input: String) -> f64 {
    let mut lines = input.lines();

    let times_string = lines.next().unwrap();
    let time: String = times_string[11..times_string.len()]
        .split(" ")
        .filter_map(|t| if t.len() == 0 { None } else { Some(t) })
        .collect();
    let time: f64 = time.parse().unwrap();

    let distances_string = lines.next().unwrap();
    let distance: String = distances_string[11..distances_string.len()]
        .split(" ")
        .filter_map(|t| if t.len() == 0 { None } else { Some(t) })
        .collect();
    let distance: f64 = distance.parse().unwrap();

    // solve: t^2 - {ttotal}t  + {distance} = 0
    // where t is time pressed on the button to accelerate

    let d = ((time.powi(2) - 4.0 * distance) as f64).sqrt();
    let x_min = (((time - d) / 2.0) + 1.).floor();
    let x_max = (((time + d) / 2.0) - 1.).ceil();
    return x_max - x_min + 1.0;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
