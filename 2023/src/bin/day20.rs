use std::collections::{HashMap, VecDeque};

use advent_of_code_2023::Aoc;

fn lcm(a: u64, b: u64) -> u64 {
    let max = a.max(b);
    let min = a.min(b);
    let mut i = max;
    loop {
        if i % min == 0 {
            return i;
        }
        i += max;
    }
}

#[derive(Debug, Clone)]
enum ModuleKind {
    Broadcaster,
    Receiver,
    /// starts everything low pulse
    Conjunction {
        cache: Vec<bool>,
    },
    /// Starts off
    FlipFlop {
        state: bool,
    },
}

#[derive(Debug, Clone)]
struct Module {
    kind: ModuleKind,
    dest: Vec<usize>,
    from: Vec<usize>,
}

impl Module {
    // initialize module
    fn init(&mut self) {
        if let ModuleKind::Conjunction { cache } = &mut self.kind {
            for _ in 0..self.from.len() {
                cache.push(false)
            }
        }
    }

    fn process(&mut self, pulse: bool, from: usize) -> Option<bool> {
        match &mut self.kind {
            ModuleKind::Broadcaster => Some(pulse),
            ModuleKind::Receiver => None,
            ModuleKind::Conjunction { cache } => {
                // update cache entry
                let from_i = self.from.iter().position(|i| *i == from).unwrap();
                cache[from_i] = pulse;

                // if all high then send false
                if cache.iter().all(|p| *p) {
                    Some(false)
                } else {
                    // otherwise send true
                    Some(true)
                }
            }
            ModuleKind::FlipFlop { state } => {
                // ignore high pulse
                if pulse {
                    return None;
                }

                // switch state
                *state = !*state;
                if *state {
                    return Some(true);
                }
                return Some(false);
            }
        }
    }
}

#[derive(Debug)]
struct Graph {
    modules: Vec<Module>,
    broadcaster: usize,
    rx: usize,
    low_pulses: u32,
    high_pulses: u32,
}

impl Graph {
    fn count_pulse(&mut self, pulse: bool) {
        // println!("{pulse}");
        if pulse {
            self.high_pulses += 1;
        } else {
            self.low_pulses += 1;
        }
    }

    fn score(&self) -> u32 {
        // println!("{} {}", self.low_pulses, self.high_pulses);
        self.low_pulses * self.high_pulses
    }

    fn press_button(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_front((0, self.broadcaster, false));
        while let Some((from, to, pulse)) = queue.pop_front() {
            // println!("{queue:?}");
            // println!("send {pulse} to {to}");
            // process pulse
            self.count_pulse(pulse);

            // send to destinations
            if let Some(pulse) = self.modules[to].process(pulse, from) {
                for d in self.modules[to].dest.clone() {
                    // count when sending to output
                    if d == self.rx {
                        self.count_pulse(pulse);
                        continue;
                    }
                    queue.push_back((to, d, pulse));
                }
            }
        }
    }

    fn find_activation_cycle(&mut self) -> u64 {
        assert!(lcm(10, 5) == 10);
        assert!(lcm(4, 6) == 12);
        let mut queue = VecDeque::new();
        let last_conjunction = self.modules[self.rx].from[0];
        let mut cycles: Vec<Option<u64>> = self.modules[last_conjunction]
            .from
            .clone()
            .into_iter()
            .map(|_| None)
            .collect();

        let mut pressed = 1;
        loop {
            queue.push_front((0, self.broadcaster, false));
            // assume only conjunction points to this
            while let Some((from, to, pulse)) = queue.pop_front() {
                // send to destinations
                if let Some(pulse) = self.modules[to].process(pulse, from) {
                    for d in self.modules[to].dest.clone() {
                        if d == last_conjunction {
                            if pulse == true {
                                let i = self.modules[d].from.iter().position(|i| *i == to).unwrap();
                                cycles[i] = Some(pressed);
                                if cycles.iter().all(|c| c.is_some()) {
                                    return cycles.into_iter().fold(1, |a, c| lcm(a, c.unwrap()));
                                }
                            }
                            continue;
                        }
                        // count when sending to output
                        queue.push_back((to, d, pulse));
                    }
                }
            }
            pressed += 1;
        }
    }
}

impl From<String> for Graph {
    fn from(input: String) -> Self {
        let mut broadcaster = 0;
        let named_map: HashMap<&str, (usize, ModuleKind, Vec<&str>)> = input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(i, l)| {
                let mut parts = l.split(" -> ");
                let name = parts.next().unwrap();
                let to: Vec<&str> = parts.next().unwrap().split(", ").collect();
                if name == "broadcaster" {
                    broadcaster = i;
                    return (name, (i, ModuleKind::Broadcaster, to));
                }
                match name.chars().next().unwrap() {
                    '&' => (
                        &name[1..name.len()],
                        (i, ModuleKind::Conjunction { cache: vec![] }, to),
                    ),
                    '%' => (
                        &name[1..name.len()],
                        (i, ModuleKind::FlipFlop { state: false }, to),
                    ),
                    _ => panic!(),
                }
            })
            .collect();
        // rx will always be last
        let mut modules = vec![
            Module {
                kind: ModuleKind::Broadcaster,
                dest: vec![],
                from: vec![]
            };
            named_map.len()
        ];
        // always have a receiver
        modules.push(Module {
            kind: ModuleKind::Receiver,
            dest: vec![],
            from: vec![],
        });
        for (i, kind, to) in named_map.values().cloned() {
            modules[i] = Module {
                kind: kind.clone(),
                dest: to
                    .into_iter()
                    .map(|name| {
                        named_map
                            .get(name)
                            .map(|(index, _, _)| *index)
                            // if not found it is a receiver
                            .unwrap_or(modules.len() - 1)
                    })
                    .collect(),
                from: vec![],
            };
        }
        // fill from field
        for i in 0..modules.len() {
            for d in modules[i].dest.clone() {
                modules[d].from.push(i);
            }
        }
        // initialize modules
        for m in modules.iter_mut() {
            m.init()
        }
        return Self {
            rx: modules.len() - 1,
            modules,
            broadcaster,
            high_pulses: 0,
            low_pulses: 0,
        };
    }
}

/// Average Duration: 643.254µs
fn part1(input: String) -> u32 {
    let mut graph = Graph::from(input);
    for _ in 0..1000 {
        graph.press_button();
    }
    graph.score()
}

/// Average Duration: 1.732614ms
fn part2(input: String) -> u64 {
    let mut graph = Graph::from(input);
    return graph.find_activation_cycle();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
