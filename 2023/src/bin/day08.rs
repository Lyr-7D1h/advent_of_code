use advent_of_code_2023::Aoc;

/// 2ms
fn part1(input: String) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let nodes: Vec<(&str, (&str, &str))> = lines
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" = ");
            let name = parts.next().unwrap();

            let dir = parts.next().unwrap();

            (name, (&dir[1..4], &dir[6..9]))
        })
        .collect();

    let mut last = None;
    let mut start = None;
    let mut map = vec![];
    for i in 0..nodes.len() {
        if nodes[i].0 == "ZZZ" {
            last = Some(i);
        }
        if nodes[i].0 == "AAA" {
            start = Some(i);
        }

        // convert names to indices
        let (l_name, r_name) = nodes[i].1;
        let mut dir = (None, None);
        for j in 0..nodes.len() {
            if nodes[j].0 == l_name {
                dir.0 = Some(j);
                if dir.0.is_some() && dir.1.is_some() {
                    break;
                }
            }
            if nodes[j].0 == r_name {
                dir.1 = Some(j);
                if dir.0.is_some() && dir.1.is_some() {
                    break;
                }
            }
        }
        map.push((dir.0.unwrap(), dir.1.unwrap()))
    }

    let last = last.unwrap();

    let mut steps = 0;
    let mut i = start.unwrap();
    loop {
        let ins = instructions[steps % instructions.len()];
        steps += 1;
        match ins {
            'L' => i = map[i].0,
            'R' => i = map[i].1,
            _ => panic!("invalid ins"),
        }
        if i == last {
            break;
        }
    }

    return steps;
}

fn part2(input: String) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let nodes: Vec<(&str, (&str, &str))> = lines
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" = ");
            let name = parts.next().unwrap();

            let dir = parts.next().unwrap();

            (name, (&dir[1..4], &dir[6..9]))
        })
        .collect();

    let mut map = vec![];
    let mut start = vec![];
    for i in 0..nodes.len() {
        // convert names to indices
        let (l_name, r_name) = nodes[i].1;
        let mut dir = (None, None);
        for j in 0..nodes.len() {
            if nodes[j].0 == l_name {
                dir.0 = Some(j);
                if dir.0.is_some() && dir.1.is_some() {
                    break;
                }
            }
            if nodes[j].0 == r_name {
                dir.1 = Some(j);
                if dir.0.is_some() && dir.1.is_some() {
                    break;
                }
            }
        }

        let name = nodes[i].0;
        if name.ends_with("A") {
            start.push(i);
        }

        let is_z = name.ends_with("Z");

        map.push((dir.0.unwrap(), dir.1.unwrap(), is_z))
    }

    let mut steps = 0;
    loop {
        let ins = instructions[steps % instructions.len()];
        let mut all_z = true;
        for i in start.iter_mut() {
            if !map[*i].2 {
                all_z = false;
            }
            match ins {
                'L' => *i = map[*i].0,
                'R' => *i = map[*i].1,
                _ => panic!("invalid ins"),
            }
        }
        if all_z {
            break;
        }
        steps += 1;
    }

    return steps;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
