use core::panic;

use advent_of_code_2023::Aoc;

fn is_palindrome(input: &[char]) -> bool {
    let mut i = 0;
    while i < input.len() / 2 {
        if input[i] != input[input.len() - 1 - i] {
            return false;
        }
        i += 1;
    }
    return true;
}

fn is_reflective(map: &Vec<Vec<char>>) -> usize {
    // check if rows are palindrome
    let mut contenders: Vec<usize> = (1..map[0].len()).collect();
    // fast check to see possible rows by checking only two chars
    for y in 0..map.len() {
        contenders.retain(|x| {
            if map[y][*x] == map[y][x - 1] {
                true
            } else {
                false
            }
        });
    }
    'outer: for x in contenders {
        // skip if columns aren't a palindrome
        for y in 0..map.len() {
            let diff = (map[0].len() - x).min(x);
            let pali = &map[y][x - diff..x + diff];
            if !is_palindrome(pali) {
                continue 'outer;
            }
        }
        return x;
    }

    let mut contenders: Vec<usize> = (1..map.len()).collect();
    for x in 0..map[0].len() {
        contenders.retain(|y| {
            if map[*y][x] == map[y - 1][x] {
                true
            } else {
                false
            }
        });
    }
    'outer: for y in contenders {
        for x in 0..map[0].len() {
            let diff = (map.len() - y).min(y);
            let pali: &Vec<char> = &map[y - diff..y + diff].into_iter().map(|l| l[x]).collect();
            if !is_palindrome(pali) {
                continue 'outer;
            }
        }
        return y * 100;
    }

    panic!()
}

/// 233.052µs
fn part1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|map| {
            let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
            is_reflective(&map)
        })
        .sum()
}

/// count the amount of mistakes each palindrome has, if more or less than 1 it doesn't contain a
/// smudge
fn is_reflective_with_smudge(map: &Vec<Vec<char>>) -> usize {
    // check if rows are palindrome
    let mut contenders: Vec<usize> = (1..map[0].len()).collect();
    let mut mistakes = vec![0_u32; map[0].len()];
    for y in 0..map.len() {
        contenders.retain(|x| {
            let mistake = &mut mistakes[*x];

            let diff = (map[0].len() - x).min(*x);
            let pali = &map[y][x - diff..x + diff];

            let mut i = 0;
            while i < pali.len() / 2 {
                if pali[i] != pali[pali.len() - 1 - i] {
                    *mistake += 1;
                    if *mistake >= 2 {
                        return false;
                    }
                }
                i += 1;
            }

            return true;
        });
    }
    if let Some((i, _)) = mistakes.into_iter().enumerate().find(|(_, m)| *m == 1) {
        return i;
    }

    let mut contenders: Vec<usize> = (1..map.len()).collect();
    let mut mistakes = vec![0; map.len()];
    for x in 0..map[0].len() {
        contenders.retain(|y| {
            let mistake = &mut mistakes[*y];

            let diff = (map.len() - y).min(*y);
            let pali: &Vec<char> = &map[y - diff..y + diff].into_iter().map(|l| l[x]).collect();

            let mut i = 0;
            while i < pali.len() / 2 {
                if pali[i] != pali[pali.len() - 1 - i] {
                    *mistake += 1;
                    if *mistake >= 2 {
                        return false;
                    }
                }
                i += 1;
            }

            return true;
        });
    }
    if let Some((i, _)) = mistakes.into_iter().enumerate().find(|(_, m)| *m == 1) {
        return i * 100;
    }

    panic!()
}

/// 287.331µs
fn part2(input: String) -> usize {
    input
        .split("\n\n")
        .map(|map| {
            let map: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
            is_reflective_with_smudge(&map)
        })
        .sum()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
