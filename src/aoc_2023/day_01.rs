use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 1;

fn get_first_digit(input: &str) -> char {
    for c in input.chars() {
        if c.is_ascii_digit() {
            return c;
        }
    }
    panic!("No digit found!");
}

fn get_last_digit(input: &str) -> char {
    for c in input.chars().rev() {
        if c.is_ascii_digit() {
            return c;
        }
    }
    panic!("No digit found!");
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let mut sum = 0;

    for l in input.lines() {
        sum += format!("{}{}", get_first_digit(l), get_last_digit(l))
            .parse::<usize>()
            .unwrap();
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let mut sum = 0;

    let repl_input = input
        .replace("zero", "zero0zero")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    for l in repl_input.lines() {
        sum += format!("{}{}", get_first_digit(l), get_last_digit(l))
            .parse::<usize>()
            .unwrap();
    }

    sum
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
        assert_eq!(t, 142);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/{}/day_{:02}_test_02.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 281);
    }
}
