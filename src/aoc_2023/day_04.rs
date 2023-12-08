use std::collections::{HashSet, HashMap};

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 4;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|c| {
            let mut numbers = c.split(": ").nth(1).unwrap().split(" | ");
            let own_num: HashSet<usize> = HashSet::from_iter(
                numbers
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap()),
            );
            let win_num: HashSet<usize> = HashSet::from_iter(
                numbers
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap()),
            );
            match win_num.intersection(&own_num).count() {
                0 => 0,
                i => 2_usize.pow(i as u32 - 1),
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cards: HashMap<usize, usize> = HashMap::new();
    input
        .lines()
        .map(|c| {
            let mut numbers = c.split(": ").nth(1).unwrap().split(" | ");
            let own_num: HashSet<usize> = HashSet::from_iter(
                numbers
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap()),
            );
            let win_num: HashSet<usize> = HashSet::from_iter(
                numbers
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap()),
            );
            match win_num.intersection(&own_num).count() {
                0 => 0,
                i => 2_usize.pow(i as u32 - 1),
            }
        })
        .sum()
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
        assert_eq!(t, 13);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 30);
    }
}
