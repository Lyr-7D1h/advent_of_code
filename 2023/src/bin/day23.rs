use std::collections::{BinaryHeap, HashMap, VecDeque};

use advent_of_code_2023::Aoc;

#[derive(Eq, Ord, PartialEq)]
struct State {
    x: isize,
    y: isize,
    prev_dir: (isize, isize),
    on_hill: bool,
    value: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// by default map does not really intersect so no need to keep history intersect
// Average Duration: 5.866322ms
fn part1(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut queue = BinaryHeap::new();
    queue.push(State {
        x: 1,
        y: 0,
        // Direction where you came from
        prev_dir: (0, -1),
        on_hill: false,
        value: 0,
    });

    let mut max = 0;
    while let Some(State {
        x,
        y,
        prev_dir: prev,
        on_hill,
        value,
    }) = queue.pop()
    {
        if x as usize == map[0].len() - 2 && y as usize == map.len() - 1 {
            max = max.max(value);
            continue;
        }
        for d in DIRECTIONS.iter() {
            if *d == prev {
                continue;
            }
            let (x, y) = (x + d.0, y + d.1);
            if x < 0 || y < 0 || x as usize > map[0].len() - 1 || y as usize > map.len() - 1 {
                continue;
            }
            match map[y as usize][x as usize] {
                '#' => continue,
                '.' => queue.push(State {
                    x,
                    y,
                    on_hill,
                    prev_dir: (-d.0, -d.1),
                    value: value + 1,
                }),

                // walking down when on hill
                '>' if on_hill && *d == (1, 0) => queue.push(State {
                    x,
                    y,
                    prev_dir: (-d.0, -d.1),
                    on_hill: false,
                    value: value + 1,
                }),
                '^' if on_hill && *d == (0, -1) => queue.push(State {
                    x,
                    y,
                    prev_dir: (-d.0, -d.1),
                    on_hill: false,
                    value: value + 1,
                }),
                '<' if on_hill && *d == (-1, 0) => queue.push(State {
                    x,
                    y,
                    prev_dir: (-d.0, -d.1),
                    on_hill: false,
                    value: value + 1,
                }),
                'v' if on_hill && *d == (0, 1) => queue.push(State {
                    x,
                    y,
                    prev_dir: (-d.0, -d.1),
                    on_hill: false,
                    value: value + 1,
                }),
                // skip if can't walk down
                '>' | '^' | '<' | 'v' if on_hill => continue,
                // going on hill
                '>' | '^' | '<' | 'v' => queue.push(State {
                    x,
                    y,
                    prev_dir: (-d.0, -d.1),
                    on_hill: true,
                    value: value + 1,
                }),

                _ => panic!(),
            }
        }
    }

    return max;
}

#[derive(Debug, Eq, Ord, PartialEq)]
struct HState {
    node: (isize, isize),
    history: Vec<(isize, isize)>,
    steps: usize,
    bound: isize,
}

impl PartialOrd for HState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bound.partial_cmp(&other.bound)
    }
}

// Very slow probably due to bad bounding and using hashmap for indexing nodes couldn't be bothered
// to improve
// Average Duration: 12.589740351s
fn part2(input: String) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // create a weighted graph
    let mut nodes: HashMap<(isize, isize), Vec<((isize, isize), usize)>> = HashMap::new();
    nodes.insert((1, 0), vec![]);
    let end = ((map[0].len() - 2) as isize, (map.len() - 1) as isize);
    nodes.insert(end, vec![]);
    let mut queue = VecDeque::new();
    queue.push_front(((1, 0), (1, 0), 0, (0, -1)));
    while let Some(((x, y), node, mut weight, prev_dir)) = queue.pop_front() {
        // update weigth
        weight += 1;

        if x as usize == map[0].len() - 2 && y as usize == map.len() - 1 {
            nodes.get_mut(&(x, y)).unwrap().push((node, weight));
            nodes.get_mut(&node).unwrap().push(((x, y), weight));
            continue;
        }

        let directions: Vec<(&(isize, isize), (isize, isize))> = DIRECTIONS
            .iter()
            .filter_map(|d| {
                if *d == prev_dir {
                    return None;
                }
                let (x, y) = (x + d.0, y + d.1);
                if x < 0 || y < 0 || x as usize > map[0].len() - 1 || y as usize > map.len() - 1 {
                    panic!();
                    // return false;
                }
                match map[y as usize][x as usize] {
                    '#' => None,
                    '>' | '^' | '<' | 'v' | '.' => Some((d, (x, y))),
                    _ => panic!(),
                }
            })
            .collect();

        match directions.len() {
            0 => {}
            1 => {
                let (d, (x, y)) = directions[0];
                queue.push_back(((x, y), node, weight, (-d.0, -d.1)));
            }
            _ => {
                let new_node = (x, y);

                // push edge to prev node and new node
                let edges = nodes.get_mut(&node).unwrap();
                if !edges.iter().any(|(n, _)| n == &new_node) {
                    edges.push((new_node, weight));
                }

                // if new node already exists just add this path to edges
                if let Some(edges) = nodes.get_mut(&new_node) {
                    if !edges.iter().any(|(n, _)| *n == node) {
                        edges.push((node, weight));
                    }
                    continue;
                }

                nodes.insert((x, y), vec![(node, weight)]);

                for (d, new_pos) in directions {
                    queue.push_back((new_pos, (x, y), 0, (-d.0, -d.1)));
                }
            }
        }
    }

    let mut queue = BinaryHeap::new();
    let bind = |history: &Vec<(isize, isize)>| -> isize {
        nodes.iter().fold(0, |a, (node, edges)| {
            if history.contains(node) {
                return a;
            }
            return a + edges.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 as isize;
        })
    };

    // if below lower bound it isn't promising
    let mut lower_bound = nodes
        .iter()
        .fold(0, |a, (_, e)| e.iter().fold(0, |a, (_, w)| a + w))
        as isize;
    queue.push(HState {
        node: (1, 0),
        steps: 0,
        history: vec![(1, 0)],
        bound: lower_bound,
    });

    let mut max = 0;
    while let Some(HState {
        node,
        steps,
        history,
        bound,
    }) = queue.pop()
    {
        if bound < lower_bound {
            continue;
        }

        if node == end {
            if steps > max {
                max = steps;
                lower_bound = max as isize;
            }
            continue;
        }

        for (n, weight) in nodes[&node].iter() {
            if history.contains(n) {
                continue;
            }
            let node = *n;
            let steps = steps + weight;
            let mut history = history.clone();
            history.push(node);
            let bound = steps as isize + bind(&history);
            if bound < lower_bound {
                continue;
            }
            queue.push(HState {
                node,
                steps,
                history,
                bound,
            })
        }
    }

    return max - 1;
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
