use advent_of_code_2022::{Aoc, Input};
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone, Copy)]
enum Operant {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

impl From<&str> for Operant {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operant::Addition,
            "-" => Operant::Subtraction,
            "/" => Operant::Division,
            "*" => Operant::Multiplication,
            _ => panic!("Invalid operant {value}"),
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(usize),
    Operation {
        a: String,
        b: String,
        operant: Operant,
    },
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        match value.parse::<usize>() {
            Ok(number) => Value::Number(number),
            Err(_) => {
                let mut split = value.split(" ");
                Value::Operation {
                    a: split.next().unwrap().to_owned(),
                    operant: Operant::from(split.next().unwrap()),
                    b: split.next().unwrap().to_owned(),
                }
            }
        }
    }
}

fn recursive_solve(value: &Value, map: &HashMap<String, Value>) -> isize {
    match value {
        Value::Number(i) => *i as isize,
        Value::Operation { a, b, operant } => {
            let a = recursive_solve(map.get(a).unwrap(), map);
            let b = recursive_solve(map.get(b).unwrap(), map);
            match operant {
                Operant::Addition => a + b,
                Operant::Subtraction => a - b,
                Operant::Division => a / b,
                Operant::Multiplication => a * b,
            }
        }
    }
}

// 700ns
fn part1(input: Input) -> isize {
    let mut map = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();
        let name: String = line[..4].to_owned();
        let value = Value::from(&line[6..line.len()]);
        map.insert(name, value);
    }

    return recursive_solve(map.get("root").unwrap(), &map);
}

#[derive(Debug)]
enum Node {
    Number {
        value: usize,
    },
    Operation {
        operant: Operant,
        // index left child + humn flag
        left: (usize, bool),
        // index rigth child + humn flag
        rigth: (usize, bool),
    },
}

#[derive(Debug)]
struct CalculationTree {
    nodes: Vec<Node>,
}

impl CalculationTree {
    fn new() -> CalculationTree {
        CalculationTree { nodes: vec![] }
    }

    fn get(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    fn add_node(&mut self, node: Node) -> usize {
        let i = self.nodes.len();
        self.nodes.push(node);
        i
    }
}

fn fill_tree(
    name: &str,
    map: &HashMap<String, Value>,
    tree: &mut CalculationTree,
) -> (usize, bool) {
    match map.get(name).unwrap() {
        Value::Number(value) => {
            let node = Node::Number { value: *value };
            let index = tree.add_node(node);
            if name == "humn" {
                (index, true)
            } else {
                (index, false)
            }
        }
        Value::Operation { a, b, operant } => {
            let left = fill_tree(a, map, tree);
            let rigth = fill_tree(b, map, tree);
            let index = tree.add_node(Node::Operation {
                operant: *operant,
                left,
                rigth,
            });
            (index, left.1 || rigth.1)
        }
    }
}

// get the value of a node without taking into account any flags
fn get_value(tree: &CalculationTree, index: usize) -> isize {
    match tree.get(index) {
        Node::Number { value, .. } => *value as isize,
        Node::Operation {
            left,
            rigth,
            operant,
            ..
        } => {
            let left = get_value(tree, left.0);
            let rigth = get_value(tree, rigth.0);
            match operant {
                Operant::Addition => left + rigth,
                Operant::Subtraction => left - rigth,
                Operant::Division => left / rigth,
                Operant::Multiplication => left * rigth,
            }
        }
    }
}

fn get_humn_value(tree: &CalculationTree, index: usize, is_root: bool, mut sum: isize) -> isize {
    let node = tree.get(index);

    match node {
        Node::Number { .. } => {
            return sum;
        }
        Node::Operation {
            operant,
            left,
            rigth,
            ..
        } => {
            if left.1 {
                let rigth = get_value(tree, rigth.0);
                sum = if is_root {
                    rigth
                } else {
                    match operant {
                        Operant::Addition => sum - rigth,
                        Operant::Subtraction => sum + rigth,
                        Operant::Division => sum * rigth,
                        Operant::Multiplication => sum / rigth,
                    }
                };
                return get_humn_value(tree, left.0, false, sum);
            } else {
                let left = get_value(tree, left.0);
                sum = if is_root {
                    left
                } else {
                    match operant {
                        Operant::Addition => sum - left,
                        Operant::Subtraction => left - sum,
                        Operant::Division => left / sum,
                        Operant::Multiplication => sum / left,
                    }
                };
                // println!("{left} {sum}");
                return get_humn_value(tree, rigth.0, false, sum);
            }
        }
    }
}

// 690ns
fn part2(input: Input) -> isize {
    let mut map = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();
        let name: String = line[..4].to_owned();
        let value = Value::from(&line[6..line.len()]);
        map.insert(name, value);
    }

    let mut tree = CalculationTree::new();

    let root = fill_tree("root", &map, &mut tree);

    return get_humn_value(&tree, root.0, true, 0);
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
