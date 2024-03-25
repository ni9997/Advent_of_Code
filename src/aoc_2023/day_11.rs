use std::{fmt::Display, str::FromStr};

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

struct Galaxy {
    map: Vec<Vec<bool>>
}

impl Galaxy {
    fn expand(&mut self) {
        
    }

    fn insert_row(&mut self, index: usize) {

    }

    fn insert_col(&mut self, index: usize) {

    }
}

#[derive(Debug)]
struct GalaxyParseError;

impl FromStr for Galaxy {
    type Err = GalaxyParseError;

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

impl Display for Galaxy {
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
    let mut galaxy: Galaxy = input.parse().unwrap();

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
