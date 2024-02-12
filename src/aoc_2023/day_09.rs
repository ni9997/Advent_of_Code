use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 9;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut ret = Vec::new();
    for l in input.lines() {
        let row = l.split(' ').map(|x| x.parse().unwrap()).collect();
        ret.push(row);
    }
    ret
}

fn extrapolate_back(input: &[i64]) -> i64 {
    if input.iter().all(|x| *x == 0) {
        0
    } else {
        let diff: Vec<i64> = input
            .iter()
            .zip(input.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        input.last().unwrap() + extrapolate_back(&diff)
    }
}

fn extrapolate_front(input: &[i64]) -> i64 {
    if input.iter().all(|x| *x == 0) {
        0
    } else {
        let diff: Vec<i64> = input
            .iter()
            .zip(input.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        input.first().unwrap() - extrapolate_front(&diff)
    }
}

pub fn part1(input: &str) -> i64 {
    let input = parse(input);
    input.iter().map(|x| extrapolate_back(x)).sum()
}

pub fn part2(input: &str) -> i64 {
    let input = parse(input);
    input.iter().map(|x| extrapolate_front(x)).sum()
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
        assert_eq!(t, 114);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 2);
    }
}
