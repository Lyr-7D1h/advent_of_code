use std::{cell::RefCell, collections::HashMap};

use advent_of_code_2023::Aoc;

thread_local! {
    pub static SEED_U16: RefCell<usize> = RefCell::new(500);
}
pub fn xorshift() -> usize {
    SEED_U16.with(|x| {
        let mut x = x.borrow_mut();
        *x ^= *x << 7;
        *x ^= *x >> 9;
        *x ^= *x << 8;

        x.clone()
    })
}

/// could be optimized by using only vectors for indexing instead of &str
/// Average Duration: 117.591048ms
fn part1(input: String) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in input.lines() {
        let mut p = l.split(": ");
        let node = p.next().unwrap();
        let mut edges: Vec<&str> = p.next().unwrap().split(" ").collect();
        for e in edges.iter() {
            if let Some(e) = map.get_mut(e) {
                e.push(node);
            } else {
                map.insert(e, vec![node]);
            }
        }
        if let Some(e) = map.get_mut(&node) {
            e.append(&mut edges);
        } else {
            map.insert(node, edges);
        }
    }

    loop {
        // add entry for contrating nodes to
        let mut cmap: HashMap<&str, (Vec<&str>, Vec<&str>)> = map
            .clone()
            .into_iter()
            .map(|(k, edges)| (k, (vec![k], edges)))
            .collect();

        while cmap.len() > 2 {
            // get "random" edge
            let keys: Vec<&str> = cmap.keys().copied().collect();
            let a = keys[xorshift() % keys.len()];
            // remove node a
            let (mut a_nodes, mut a_edges) = cmap.remove(a).unwrap();
            // remove b edge from a
            let b = a_edges.swap_remove(xorshift() % a_edges.len());

            // remove a edge from b
            let e = &mut cmap.get_mut(b).unwrap().1;
            e.retain_mut(|n| *n != a);

            // rename every edge from a to b
            for dn in a_edges.iter() {
                // println!("{dn:?}");
                let e = &mut cmap.get_mut(dn).unwrap().1;

                e.iter_mut().for_each(|n| {
                    if *n == a {
                        *n = b
                    }
                })
            }
            a_edges.retain_mut(|n| *n != b);

            // add edges from a to b
            let (b_nodes, b_edges) = cmap.get_mut(b).unwrap();
            b_nodes.append(&mut a_nodes);
            b_edges.append(&mut a_edges);
        }

        let (nodes, edges) = cmap.values().next().unwrap();
        if edges.len() == 3 {
            let a = nodes.len();
            let b = map.len() - a;
            return a * b;
        }
    }
}

fn part2(input: String) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
