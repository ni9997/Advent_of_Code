use std::str::FromStr;

use crate::utils::get_input;

const YEAR: usize = 2025;
const DAY: usize = 1;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Debug)]
struct RotationParseError;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = RotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_at(1);
        match dir {
            "L" => Ok(Rotation::Left(amount.parse().unwrap())),
            "R" => Ok(Rotation::Right(amount.parse().unwrap())),
            _ => panic!(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let lines = input.lines().map(|l| Rotation::from_str(l).unwrap());
    let mut dial: i32 = 50;
    let mut zeros: usize = 0;

    for r in lines {
        match r {
            Rotation::Left(i) => dial -= i,
            Rotation::Right(i) => dial += i,
        };
        dial %= 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().map(|l| Rotation::from_str(l).unwrap());
    let mut dial: i32 = 50;
    let mut zeros: usize = 0;

    for r in lines {
        match r {
            Rotation::Left(d) => {
                for i in 0..d {
                    dial -= 1;
                    if dial == 0 {
                        zeros += 1;
                    } else if dial < 0 {
                        dial = 99;
                    }
                }
            }
            Rotation::Right(d) => {
                for i in 0..d {
                    dial += 1;
                    dial %= 100;
                    if dial == 0 {
                        zeros += 1;
                    }
                }
            }
        };
    }
    zeros
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
        assert_eq!(t, 3);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 6);
    }
}
