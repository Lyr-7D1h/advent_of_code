use std::{
    cmp::{self, Ordering},
    fmt::{Display, Formatter},
    i128,
    num::ParseIntError,
    ops::{Add, Div, Mul, MulAssign, Neg, Sub},
    str::FromStr,
};

use advent_of_code_2023::Aoc;

// Average Duration: 573.4µs
fn part1(input: String) -> u32 {
    let hailstones: Vec<[(f32, f32, f32); 2]> = input
        .lines()
        .map(|l| {
            match &l
                .split(" @ ")
                .map(|p| p.split(", ").map(|p| p.trim().parse().unwrap()).collect())
                .collect::<Vec<Vec<f32>>>()[..]
            {
                [p, v] => [(p[0], p[1], p[2]), (v[0], v[1], v[2])],
                _ => panic!(),
            }
        })
        .collect();

    // y = ax + b
    let functions: Vec<(f32, f32)> = hailstones
        .iter()
        .map(|[(x, y, _), (vx, vy, _)]| {
            // Fy / Fx
            let a = vy / vx;
            // b = y - ax
            let b = y - a * x;
            (a, b)
        })
        .collect();

    // let lower = 7.0;
    // let upper = 27.0;
    let lower = 200000000000000.0;
    let upper = 400000000000000.0;
    let mut count = 0;
    for i in 0..functions.len() {
        for j in i + 1..functions.len() {
            let (a, b) = functions[i];
            let (c, d) = functions[j];
            if a == c {
                continue;
            }
            // y = y = (a - c)x = d - b
            let x = (d - b) / (a - c);
            // y = ax + b
            let y = a * x + b;

            // check wheter the sign of the vector between intersection and starting point is the
            // same as velocity vector
            let [p, v] = hailstones[i];
            let [jp, jv] = hailstones[j];
            if (x - p.0).is_sign_positive() != v.0.is_sign_positive()
                || (y - p.1).is_sign_positive() != v.1.is_sign_positive()
                || (x - jp.0).is_sign_positive() != jv.0.is_sign_positive()
                || (y - jp.1).is_sign_positive() != jv.1.is_sign_positive()
            {
                continue;
            }

            if lower <= x && x <= upper && lower <= y && y <= upper {
                count += 1;
            }
        }
    }

    count
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// precise fraction type
#[derive(Copy, Clone, Debug)]
struct Fraction {
    numerator: i128,
    denominator: i128,
}
impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            f.write_fmt(format_args!("{}", self.numerator))
        } else {
            f.write_fmt(format_args!("{}/{}", self.numerator, self.denominator))
        }
    }
}
impl Fraction {
    fn new(numerator: i128, denominator: i128) -> Fraction {
        assert!(denominator != 0);
        let gcd = gcd(numerator.abs(), denominator.abs());
        let sign = numerator.signum() * denominator.signum();

        let numerator = sign * (numerator.abs() / gcd);
        let denominator = denominator.abs() / gcd;

        Fraction {
            numerator,
            denominator,
        }
    }
    fn abs(&self) -> Fraction {
        Fraction {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
    fn round(self) -> i128 {
        self.numerator.div(self.denominator)
        // Fraction {
        //     numerator: ,
        //     denominator: 1,
        // }
    }
    /// returns how much the number will differ if approximated
    fn accuracy(&self) -> i128 {
        let a = gcd(self.numerator.abs(), self.denominator.abs());
        // println!("AAA {a}");
        // (self.numerator.to_string().len() * self.denominator.to_string().len()) as i128
        let a = self.numerator.to_string().len();
        let b = self.denominator.to_string().len();
        (a + b - a.abs_diff(b)) as i128
        // self.round()
    }
    fn approximate(&self) -> Fraction {
        let s = 2;
        Fraction::new(self.numerator / s, self.denominator / s)
    }
}
impl FromStr for Fraction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fraction {
            numerator: i128::from_str(s)?,
            denominator: 1,
        })
    }
}
impl From<i128> for Fraction {
    fn from(value: i128) -> Self {
        Fraction {
            numerator: value,
            denominator: 1,
        }
    }
}
impl PartialEq<Fraction> for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}
impl PartialEq<i128> for Fraction {
    fn eq(&self, other: &i128) -> bool {
        // never equal if it isn't a whole number
        if self.denominator > 1 {
            return false;
        }
        self.numerator == *other
    }
}
impl Neg for Fraction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fraction {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}
impl Sub for Fraction {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let mut s = self;
        let mut other = other;
        loop {
            let new_numerator = s.numerator.checked_mul(other.denominator).and_then(|a| {
                other
                    .numerator
                    .checked_mul(s.denominator)
                    .and_then(|b| a.checked_sub(b))
            });
            let new_denominator = s.denominator.checked_mul(other.denominator);

            if new_denominator.is_some() && new_numerator.is_some() {
                return Fraction::new(new_numerator.unwrap(), new_denominator.unwrap());
            }

            if s.accuracy() > other.accuracy() {
                s = s.approximate();
            } else {
                other = other.approximate();
            }
        }
    }
}
impl Add for Fraction {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut s = self;
        let mut other = other;
        loop {
            let new_numerator = s.numerator.checked_mul(other.denominator).and_then(|a| {
                other
                    .numerator
                    .checked_mul(s.denominator)
                    .and_then(|b| a.checked_add(b))
            });
            let new_denominator = s.denominator.checked_mul(other.denominator);
            if new_denominator.is_some() && new_numerator.is_some() {
                return Fraction::new(new_numerator.unwrap(), new_denominator.unwrap());
            }

            if s.accuracy() > other.accuracy() {
                s = s.approximate();
            } else {
                other = other.approximate();
            }
        }
    }
}
impl Mul for Fraction {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut s = self;
        let mut other = other;
        loop {
            let new_numerator = s.numerator.checked_mul(other.numerator);
            let new_denominator = s.denominator.checked_mul(other.denominator);

            if new_denominator.is_some() && new_numerator.is_some() {
                return Fraction::new(new_numerator.unwrap(), new_denominator.unwrap());
            }

            if s.accuracy() > other.accuracy() {
                s = s.approximate();
            } else {
                other = other.approximate();
            }
        }
    }
}
impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.mul(Fraction {
            numerator: rhs.denominator,
            denominator: rhs.numerator,
        })
    }
}
impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

type Matrix = Vec<Vec<Fraction>>;
fn print(amatrix: &Matrix) {
    let a = amatrix
        .iter()
        .map(|r| {
            r.iter()
                .map(|i| {
                    let i = format!("{i}");
                    format!("{:30}", &i[0..i.len().min(30)])
                })
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("\n{a}",);
}
/// Add to one row a scalar multiple of another.
fn add(amatrix: &mut Matrix, to: usize, sto: Fraction, from: usize, sfrom: Fraction) {
    println!("adding {from} scaled by {sfrom} to {to} scaled by {sto}");
    for i in 0..amatrix.len() + 1 {
        let a = amatrix[to][i] * sto;
        let b = sfrom * amatrix[from][i];
        amatrix[to][i] = a + b;
    }
}
/// Multiply a row by a non-zero scalar.
fn scale(amatrix: &mut Matrix, row: usize, s: Fraction) {
    println!("Scaling {row} by {s}");
    amatrix[row].iter_mut().for_each(|i| *i *= s);
}
/// Swap the positions of two rows.
fn swap(amatrix: &mut Matrix, a: usize, b: usize) {
    if a == b {
        return;
    }
    println!("Swapping {a} {b}");
    amatrix.swap(a, b);
}
/// Gaussian Elimination on an augmented matrix to solve Ax=b returns x
fn gaussian_elimination(mut amatrix: Matrix) -> Option<Vec<Fraction>> {
    let n = amatrix.len();
    // n x n + 1
    assert!(n + 1 == amatrix[0].len(), "augmented matrix is not n x n+1");
    print(&amatrix);

    for col in 0..n {
        println!();
        print(&amatrix);
        // use the smallest row as pivot
        let r = (col..n).filter(|r| amatrix[*r][col] != 0).min_by(|a, b| {
            match (amatrix[*a][col].abs() - amatrix[*b][col].abs()).round() {
                0 => Ordering::Equal,
                d if d > 0 => Ordering::Less,
                _ => Ordering::Greater,
            }
        })?;
        swap(&mut amatrix, col, r);

        for row in 0..n {
            let v = amatrix[row][col];
            if v == 0 {
                continue;
            }
            match row {
                // scale pivot to 1
                row if row == col => {
                    scale(&mut amatrix, row, Fraction::from(1) / v);
                    // always round the pivot to 1
                    amatrix[col][col] = 1.into();
                }
                // remove pivot row from above
                row if row < col => {
                    let pivot = amatrix[col][col];
                    add(&mut amatrix, row, 1.into(), col, -v / pivot);
                    amatrix[row][col] = 0.into();
                }
                // remove pivot row below
                row => {
                    add(&mut amatrix, row, 1.into(), col, -v);
                    amatrix[row][col] = 0.into();
                }
            }
        }
    }
    print(&amatrix);

    // return last row
    Some(amatrix.into_iter().map(|r| r[n]).collect())
}

fn build_constraints<'a>(hailstones: &'a Vec<[(i128, i128, i128); 2]>) -> Matrix {
    let mut a = vec![];
    for i in 0..3 {
        let [(xi, yi, zi), (vxi, vyi, vzi)] = hailstones[i];
        let [(xj, yj, zj), (vxj, vyj, vzj)] = hailstones[i + 1];

        a.push(vec![
            vyj - vyi,
            vxi - vxj,
            0,
            yi - yj,
            xj - xi,
            0,
            yi * vxi - xi * vyi + xj * vyj - yj * vxj,
        ]);
        a.push(vec![
            vzj - vzi,
            0,
            vxi - vxj,
            zi - zj,
            0,
            xj - xi,
            zi * vxi - xi * vzi + xj * vzj - zj * vxj,
        ]);
    }
    return a
        .into_iter()
        .map(|r| r.into_iter().map(|i| i.into()).collect())
        .collect();
}

///
/// - Could be solved using cross products (https://github.com/electronsandstuff/AdventofCode2023/blob/162cb0fca7d73ca0ba4377e012d074149351bc06/src/day24.jl)
/// - Or by calculating x,y,vx,vy first (https://github.com/ash42/adventofcode/blob/main/adventofcode2023/src/nl/michielgraat/adventofcode2023/day24/Day24.java)
/// solving 2 2x2 matrices
fn part2(input: String) -> i128 {
    assert_eq!(
        gaussian_elimination(vec![
            vec![2.into(), 4.into(), 4.into()],
            vec![3.into(), 5.into(), 5.into()]
        ])
        .unwrap(),
        vec![Fraction::from(0), Fraction::from(1)]
    );
    let hailstones: Vec<[(i128, i128, i128); 2]> = input
        .lines()
        .map(|l| {
            match &l
                .split(" @ ")
                .map(|p| p.split(", ").map(|p| p.trim().parse().unwrap()).collect())
                .collect::<Vec<Vec<_>>>()[..]
            {
                [p, v] => [(p[0], p[1], p[2]), (v[0], v[1], v[2])],
                _ => panic!(),
            }
        })
        .collect();

    let a = build_constraints(&hailstones);
    let x = gaussian_elimination(a).expect("non singular");
    print(&vec![x.clone()]);
    return (x[0] + x[1] + x[2]).round();
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
