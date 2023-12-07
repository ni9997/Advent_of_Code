use std::collections::HashSet;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 3;

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let mut points = HashSet::new();
        let value = (ch as u8 - b'0') as i64;
        points.extend([
            (row + 1, col - 1),
            (row, col - 1),
            (row - 1, col - 1),
            (row + 1, col),
            (row, col),
            (row - 1, col),
            (row + 1, col + 1),
            (row, col + 1),
            (row - 1, col + 1),
        ]);
        Self { value, points }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value *= 10;
        self.value += (ch as u8 - b'0') as i64;
        self.points
            .extend([(row, col + 1), (row - 1, col + 1), (row + 1, col + 1)]);
    }

    fn next_to_symbol(&self, syms: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(syms).next().is_some()
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i64 {
    let mut nums: Vec<PartNumber> = vec![];
    let mut syms: HashSet<(i64, i64)> = HashSet::new();

    let mut cur_part: Option<PartNumber> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_part {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_part = Some(PartNumber::new(row as i64, col as i64, ch))
                }
            } else {
                if let Some(num) = cur_part.take() {
                    nums.push(num);
                }
                if ch != '.' {
                    syms.insert((row as i64, col as i64));
                }
            }
        }
        if let Some(num) = cur_part.take() {
            nums.push(num);
        }
    }

    nums.iter()
        .filter(|num| num.next_to_symbol(&syms))
        .map(|x| x.value)
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    let mut nums: Vec<PartNumber> = vec![];
    let mut syms: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let mut cur_part: Option<PartNumber> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_part {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_part = Some(PartNumber::new(row as i64, col as i64, ch))
                }
            } else {
                if let Some(num) = cur_part.take() {
                    nums.push(num);
                }
                if ch != '.' {
                    syms.insert((row as i64, col as i64));
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
        if let Some(num) = cur_part.take() {
            nums.push(num);
        }
    }

    let mut total = 0;
    for gear in &gears {
        let mut matches = vec![];
        for num in &nums {
            if num.points.contains(gear) {
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }
    total
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
        assert_eq!(t, 4361);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 467835);
    }
}
