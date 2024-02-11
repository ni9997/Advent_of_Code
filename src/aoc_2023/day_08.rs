use std::collections::HashMap;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 8;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for l in lines {
        let mut split = l.split(" = ");
        let key = split.next().unwrap();
        let mut val_split = split.next().unwrap().split(", ");
        let left_val = val_split.next().unwrap()[1..].to_string();
        let mut right_val = val_split.next().unwrap().to_string();
        right_val.pop();
        left.insert(key, left_val);
        right.insert(key, right_val);
    }

    let mut current = "AAA";
    let mut steps = 0;
    for i in instructions.chars().cycle() {
        match i {
            'L' => {
                current = left.get(current).unwrap();
            }
            'R' => {
                current = right.get(current).unwrap();
            }
            _ => panic!(),
        }
        steps += 1;
        if current == "ZZZ" {
            return steps;
        }
    }
    panic!()
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = lcm(first, second);
    for &next in iter {
        ans = lcm(ans, next);
    }
    ans
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    let mut starts = Vec::new();

    for l in lines {
        let mut split = l.split(" = ");
        let key = split.next().unwrap().to_string();
        let mut val_split = split.next().unwrap().split(", ");
        let left_val = val_split.next().unwrap()[1..].to_string();
        let mut right_val = val_split.next().unwrap().to_string();
        right_val.pop();
        left.insert(key.clone(), left_val);
        right.insert(key.clone(), right_val);
        if key.ends_with('A') {
            starts.push(key.clone());
        }
    }
    println!("Setup done");
    println!("{} start positions", starts.len());
    println!("{:?}", starts);
    let mut ans = Vec::new();
    for start in starts {
        let mut current = start;
        let mut steps = 0;
        for i in instructions.chars().cycle() {
            match i {
                'L' => {
                    current = left.get(&current).unwrap().clone();
                }
                'R' => {
                    current = right.get(&current).unwrap().clone();
                }
                _ => panic!(),
            }
            steps += 1;
            if current.ends_with('Z') {
                ans.push(steps);
                break;
            }
        }
    }

    lcm_of(&ans)
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
        assert_eq!(t, 2);
    }

    #[test]
    fn part1_test2() {
        let path = format!("input/{}/day_{:02}_test_02.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 6);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_03.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 6);
    }
}
