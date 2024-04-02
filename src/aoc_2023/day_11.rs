use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 11;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input, 1000000));
}

struct Universe {
    initial_map: Vec<Vec<bool>>,
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn expand(&mut self, size: usize) {
        let mut empty_rows = vec![];
        for r in 0..self.initial_map.len() {
            if self.get_row(r).iter().all(|x| !x) {
                empty_rows.push(r);
            }
        }

        let mut empty_cols = vec![];
        for c in 0..self.initial_map[0].len() {
            if self.get_col(c).iter().all(|x| !x) {
                empty_cols.push(c);
            }
        }

        for g in self.galaxies.iter_mut() {
            g.0 += empty_rows.iter().filter(|x| **x < g.0).count() * size;
            g.1 += empty_cols.iter().filter(|x| **x < g.1).count() * size;
        }
    }

    fn new(initial_map: Vec<Vec<bool>>) -> Self {
        let mut galaxies = vec![];

        for (r, col) in initial_map.iter().enumerate() {
            for (c, galaxy) in col.iter().enumerate() {
                if *galaxy {
                    galaxies.push((r, c));
                }
            }
        }

        Self {
            initial_map,
            galaxies,
        }
    }

    fn get_col(&self, index: usize) -> Vec<bool> {
        let mut v = vec![];
        for c in &self.initial_map {
            v.push(c[index]);
        }
        v
    }

    fn get_row(&self, index: usize) -> Vec<bool> {
        self.initial_map[index].clone()
    }
}

#[derive(Debug)]
struct UniverseParseError;

impl FromStr for Universe {
    type Err = UniverseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = vec![];
        for line in s.lines() {
            let mut l = vec![];
            for c in line.chars() {
                match c {
                    '.' => l.push(false),
                    '#' => l.push(true),
                    _ => panic!(),
                }
            }
            map.push(l);
        }
        Ok(Universe::new(map))
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.initial_map {
            for r in l {
                match r {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn get_distance(u1: (usize, usize), u2: (usize, usize)) -> usize {
    u1.0.abs_diff(u2.0) + u1.1.abs_diff(u2.1)
}

pub fn part1(input: &str) -> usize {
    let mut universe: Universe = input.parse().unwrap();

    universe.expand(1);

    let d_twice: usize = universe
        .galaxies
        .iter()
        .permutations(2)
        .map(|x| get_distance(*x[0], *x[1]))
        .sum();
    d_twice / 2
}

pub fn part2(input: &str, size: usize) -> usize {
    let mut universe: Universe = input.parse().unwrap();

    universe.expand(size);

    let d_twice: usize = universe
        .galaxies
        .iter()
        .permutations(2)
        .map(|x| get_distance(*x[0], *x[1]))
        .sum();
    d_twice / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 374);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input, 10);
        assert_eq!(t, 1030);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input, 100);
        assert_eq!(t, 8410);
    }
}
