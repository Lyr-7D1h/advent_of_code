use advent_of_code_2022::{Aoc, Input};
use std::{
    fmt::{Debug, Display},
    io::BufRead,
};

#[derive(Debug)]
struct Node<V> {
    prev: usize,
    next: usize,
    value: V,
    moved: bool,
}

#[derive(Debug)]
struct LinkedList<V> {
    nodes: Vec<Node<V>>,
    cursor: usize,
}

impl<V: Debug> Display for LinkedList<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec = vec![];
        let mut current = 0;
        for _ in self.nodes.iter() {
            let c = &self.nodes[current];
            vec.push(&c.value);
            current = c.next;
        }
        write!(f, "{:?}", vec)
    }
}

impl<V> LinkedList<V> {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn current(&self) -> &Node<V> {
        &self.nodes[self.cursor]
    }

    /// Get the index of where the cursor is pointing to
    fn cursor(&self) -> usize {
        self.cursor
    }

    /// Set the index of where the cursor is pointing to
    fn set_cursor(&mut self, index: usize) {
        self.cursor = index;
    }

    fn get(&self, index: usize) -> &Node<V> {
        &self.nodes[index]
    }

    /// Set the cursor to the next location
    fn next(&mut self) {
        self.cursor = self.nodes[self.cursor].next;
    }

    /// Move cursor to the previous
    fn prev(&mut self) {
        self.cursor = self.nodes[self.cursor].prev;
    }

    /// move given index to after the cursor
    fn move_after(&mut self, index: usize) {
        let from = index;
        let to = self.cursor;

        // set from as moved
        self.nodes[from].moved = true;

        if from == to {
            return;
        }

        // remove from
        let from_prev = self.nodes[from].prev;
        let from_next = self.nodes[from].next;
        self.nodes[from_prev].next = from_next;
        self.nodes[from_next].prev = from_prev;

        // insert after to
        let to_next = self.nodes[to].next;
        self.nodes[to].next = from;
        self.nodes[from].prev = to;
        self.nodes[from].next = to_next;
        self.nodes[to_next].prev = from;
    }
}

impl<V> FromIterator<V> for LinkedList<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut nodes = vec![];
        for value in iter {
            let prev = {
                if nodes.len() == 0 {
                    0
                } else {
                    nodes.len() - 1
                }
            };
            nodes.push(Node {
                prev,
                next: nodes.len() + 1,
                value,
                moved: false,
            })
        }

        nodes.first_mut().unwrap().prev = nodes.len() - 1;
        nodes.last_mut().unwrap().next = 0;

        LinkedList { nodes, cursor: 0 }
    }
}

fn part1(input: Input) -> isize {
    let mut list: LinkedList<isize> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // for all elements in the list
    for _ in 0..list.len() {
        // get the next one that hasn't been moved
        let mut current = list.current();
        while current.moved == true {
            list.next();
            current = list.current();
        }

        let offset = current.value % list.len() as isize;
        let prev_index = current.prev;
        let current_index = list.cursor();

        // move cursor to offset
        if offset.is_positive() {
            for _ in 0..offset {
                list.next();
            }
        } else {
            for _ in offset..1 {
                list.prev();
            }
        }

        // move the current to before the offset
        list.move_after(current_index);

        // move on to the next in the list
        list.set_cursor(prev_index);
        list.next();
    }

    // get the cursor the the 0 node
    while list.current().value != 0 {
        list.next();
    }

    let mut sum = 0;
    for _ in 0..1000 {
        list.next();
    }
    sum += list.current().value;

    for _ in 0..1000 {
        list.next();
    }
    sum += list.current().value;

    for _ in 0..1000 {
        list.next();
    }
    sum += list.current().value;

    return sum;
}

struct Entry {
    value: isize,
    moved: bool,
}
fn part1_vec(input: Input) {
    let values: Vec<Entry> = input
        .lines()
        .map(|l| Entry {
            value: l.unwrap().parse().unwrap(),
            moved: false,
        })
        .collect();

    let mut i = 0;
    for _ in 0..values.len() {
        let mut current = &values[i % values.len()];
        while current.moved == true {
            i += 1;
            current = &values[i % values.len()];
        }
        todo!()
    }
}

fn part2(input: Input) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
