use advent_of_code_2023::Aoc;

/// 140ns
fn part1(input: String) -> u32 {
    let map: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();

    let mut sum = 0;

    for (y, line) in map.iter().enumerate() {
        let mut digit: Option<String> = None;

        'line: for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if let Some(digit) = &mut digit {
                    digit.push(*c);
                } else {
                    digit = Some(c.to_string());
                }
                continue;
            }

            // check if digit has surrounding symbol
            if let Some(digit_string) = digit.clone() {
                for y in y.saturating_sub(1)..map.len().min(y + 2) {
                    for x in x.saturating_sub(digit_string.len() + 1)..line.len().min(x + 1) {
                        let c = map[y][x];
                        // is symbol?
                        if !c.is_ascii_digit() && c != '.' {
                            sum += digit_string.parse::<u32>().unwrap();
                            digit = None;
                            continue 'line;
                        }
                    }
                }
            }
            digit = None;
        }

        // check if digit has surrounding symbol
        if let Some(digit_string) = digit.clone() {
            let x = line.len() - 1;
            for y in y.saturating_sub(1)..map.len().min(y + 2) {
                for x in x.saturating_sub(digit_string.len() + 1)..line.len().min(x + 1) {
                    let c = map[y][x];
                    // is symbol?
                    if !c.is_ascii_digit() && c != '.' {
                        // println!("{:?}", digit_string.parse::<u32>().unwrap());
                        sum += digit_string.parse::<u32>().unwrap();
                    }
                }
            }
        }
    }

    return sum;
}

// 116ns
fn part2(input: String) -> u32 {
    let mut sum = 0;
    let mut map: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();
    // ends with empty vector when splitting
    map.pop();

    let y_max = map.len();
    let x_max = map[0].len();
    for y in 0..y_max {
        for x in 0..x_max {
            if map[y][x] == '*' {
                let mut positions = vec![];
                for y in y.saturating_sub(1)..(y + 2).min(y_max) {
                    for x in x.saturating_sub(1)..(x + 2).min(x_max) {
                        if map[y][x].is_ascii_digit() {
                            positions.push((x, y));
                        }
                    }
                }

                // reduce adjacent digits
                let mut remove = vec![];
                for i in 0..positions.len() {
                    for j in i..positions.len() {
                        if positions[i].1 == positions[j].1 && positions[j].0 == positions[i].0 + 1
                        {
                            remove.push(j)
                        }
                    }
                }
                for i in remove.into_iter().rev() {
                    positions.remove(i);
                }

                let mut digits: Vec<u32> = vec![];
                for (mut x, y) in positions {
                    loop {
                        if x == 0 {
                            break;
                        }
                        if !map[y][x - 1].is_ascii_digit() {
                            break;
                        }
                        x -= 1;
                    }

                    let mut digit = String::new();
                    while map[y][x].is_ascii_digit() {
                        digit.push(map[y][x]);
                        x += 1;
                        if x == x_max {
                            break;
                        }
                    }
                    let digit = digit.parse::<u32>().unwrap();
                    digits.push(digit);
                }

                if digits.len() == 2 {
                    sum += digits[0] * digits[1];
                }
            }
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
