use std::{cell, str::FromStr};

use crate::utils::get_input;

const YEAR: usize = 2025;
const DAY: usize = 3;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Debug)]
struct Bank {
    cells: Vec<u8>
}

impl Bank {
    fn get_largest_joltage_part1(&self) -> usize {
        let mut max = 0;
        for (i, first) in self.cells.iter().enumerate() {
            for (j, second) in self.cells.iter().enumerate() {
                if i < j {
                    if max < first*10 + second {
                        max = first*10 + second;
                    }
                }
            }
        }
        max.into()
    }

    fn get_largest_joltage_part2(&mut self) -> usize {
        let mut c = self.cells.clone();
        c.sort();
        c.reverse();
        let mut sum = 0;
        let mut index = 0;
        for x in 0..self.cells.len() {
            if index >= 12 {
                break;
            }
            if c[index] == self.cells[x] {
                println!("index: {}", index);
                index += 1;
                sum += c[index] as usize *10usize.pow(12u32-index as u32);
            }
            
        }
        println!("sum: {}", sum);
        sum
    }
}

#[derive(Debug)]
struct ParsingError;

impl FromStr for Bank {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::with_capacity(s.len());
        for c in s.chars() {
            cells.push((c as u8)-48);
        }
        Ok(Bank { cells })
    }
}

pub fn part1(input: &str) -> usize {
    input.split("\n").map(|l| Bank::from_str(l).unwrap().get_largest_joltage_part1()).sum()
}

pub fn part2(input: &str) -> usize {
    input.split("\n").map(|l| Bank::from_str(l).unwrap().get_largest_joltage_part2()).sum()
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
        assert_eq!(t, 357);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 3121910778619);
    }
}
