use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 11;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

struct Universe {
    map: Vec<Vec<bool>>,
}

impl Universe {
    fn expand(&mut self) {
        for r in (0..self.map.len()).rev() {
            if self.get_row(r).iter().all(|x|!x) {
                self.insert_row(r);
            }
        }
        for c in (0..self.map[0].len()).rev() {
            if self.get_col(c).iter().all(|x|!x) {
                self.insert_col(c);
            }
        }
    }

    fn insert_row(&mut self, index: usize) {
        self.map.insert(index, vec![false; self.map[0].len()]);
    }

    fn insert_col(&mut self, index: usize) {
        for r in self.map.iter_mut() {
            r.insert(index, false);
        }
    }

    fn get_col(&self, index: usize) -> Vec<bool> {
        let mut v = vec![];
        for c in &self.map {
            v.push(c[index]);
        }
        v
    }

    fn get_row(&self, index: usize) -> Vec<bool> {
        self.map[index].clone()
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
                    _ => panic!()
                }
            }
            map.push(l);
        }
        Ok(Self { map })
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.map {
            for r in l {
                match r {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> usize {
    let mut galaxy: Universe = input.parse().unwrap();

    galaxy.expand();

    println!("{}", galaxy);

    todo!()
}

pub fn part2(input: &str) -> usize {
    todo!()
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
        let t = part2(&input);
        assert_eq!(t, 2);
    }
}
