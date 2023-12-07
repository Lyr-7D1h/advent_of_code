use std::cmp::Ordering;

use advent_of_code_2023::Aoc;

#[derive(Debug, Ord, Eq)]
struct Hand {
    bid: u32,
    cards: Vec<char>,
    hand_type: usize,
    joker_enabled: bool,
}

fn hand_type(cards: &str) -> usize {
    let mut map = vec![];
    for c in cards.chars() {
        if let Some((_, count)) = map.iter_mut().find(|(ci, _)| *ci == c) {
            *count += 1;
        } else {
            map.push((c, 1));
        }
    }

    // by default distinct
    let mut hand_type = 1;
    for i in 0..map.len() {
        match map[i].1 {
            5 => return 7,
            4 => return 6,
            3 => {
                if hand_type == 2 {
                    return 5;
                }
                if let Some((_, count)) = map.get(i + 1) {
                    if *count == 2 {
                        return 5;
                    }
                }
                return 4;
            }
            2 if hand_type == 2 => return 3,
            2 => hand_type = 2,
            1 => {}
            _ => panic!("impossible card count"),
        }
    }

    return hand_type;
}

fn hand_type_joker(cards: &str) -> usize {
    let mut map = vec![];
    for c in cards.chars() {
        if let Some((_, count)) = map.iter_mut().find(|(ci, _)| *ci == c) {
            *count += 1;
        } else {
            map.push((c, 1));
        }
    }

    // add joker to max card
    let mut joker = 0;
    let mut map: Vec<u32> = map
        .into_iter()
        .filter_map(|(c, o)| {
            if c == 'J' {
                joker = o;
                None
            } else {
                Some(o)
            }
        })
        .collect();
    map.sort();
    map.reverse();
    match map.get_mut(0) {
        Some(o) => *o += joker,
        None => map.push(joker),
    }

    // by default distinct
    let mut hand_type = 1;

    for i in 0..map.len() {
        match map[i] {
            5 => return 7,
            4 => return 6,
            3 => {
                if hand_type == 2 {
                    return 5;
                }
                if let Some(count) = map.get(i + 1) {
                    if *count == 2 {
                        return 5;
                    }
                }
                return 4;
            }
            2 if hand_type == 2 => return 3,
            2 => hand_type = 2,
            1 => {}
            _ => panic!("impossible card count"),
        }
    }

    return hand_type;
}

impl Hand {
    fn from_line(line: &str, joker_enabled: bool) -> Hand {
        let mut parts = line.split(" ");
        let cards = parts.next().unwrap().to_string();
        let bid = parts.next().unwrap().parse().unwrap();
        let hand_type = if joker_enabled {
            hand_type_joker(&cards)
        } else {
            hand_type(&cards)
        };

        Hand {
            bid,
            cards: cards.chars().collect(),
            hand_type,
            joker_enabled,
        }
    }
    fn card_value(&self, index: usize) -> u32 {
        let card = self.cards[index];
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if self.joker_enabled {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = self.hand_type.cmp(&other.hand_type);

        if let Ordering::Equal = ord {
            for i in 0..self.cards.len() {
                let a = self.card_value(i);
                let b = other.card_value(i);
                match a.cmp(&b) {
                    Ordering::Equal => {}
                    ord => return Some(ord),
                }
            }
        }

        return Some(ord);
    }
}

/// 312.849µs
fn part1(input: String) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .into_iter()
        .map(|l| Hand::from_line(l, false))
        .collect();
    hands.sort();
    return hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (rank, hand)| sum + ((rank + 1) as u32 * hand.bid));
}

/// 329.685µs
fn part2(input: String) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .into_iter()
        .map(|l| Hand::from_line(l, true))
        .collect();
    hands.sort();
    return hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (rank, hand)| sum + ((rank + 1) as u32 * hand.bid));
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
