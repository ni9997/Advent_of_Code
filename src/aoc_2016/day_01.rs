use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn run() {
    println!("Day 01");
    let input = fs::read_to_string("input/2016/day_01.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    let mut direction = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in input.split(", ") {
        // println!("{}", dir);
        // println!("{}", &dir[0..1]);
        match &dir[0..1] {
            "L" => {
                if direction == 0 {
                    direction = 3;
                } else {
                    direction = direction - 1;
                }
            }
            "R" => {
                direction = (direction + 1) % 4;
            }
            _ => panic!(),
        }
        let distance = dir[1..].parse::<i32>().unwrap();
        match direction {
            0 => {
                y = y + distance;
            }
            1 => {
                x = x + distance;
            }
            2 => {
                y = y - distance;
            }
            3 => {
                x = x - distance;
            }
            _ => panic!(),
        }
        // println!("{} {}", x, y);
    }
    return x.abs() + y.abs();
}

pub fn part2(input: &str) -> i32 {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut direction = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in input.split(", ") {
        match &dir[0..1] {
            "L" => {
                if direction == 0 {
                    direction = 3;
                } else {
                    direction = direction - 1;
                }
            }
            "R" => {
                direction = (direction + 1) % 4;
            }
            _ => panic!(),
        }
        let distance = dir[1..].parse::<i32>().unwrap();
        match direction {
            0 => {
                for _ in 0..distance {
                    y += 1;
                    // println!("{} {}", x, y);
                    if !visited.insert((x, y)) {
                        return x.abs() + y.abs();
                    }
                }
            }
            1 => {
                for _ in 0..distance {
                    x += 1;
                    // println!("{} {}", x, y);
                    if !visited.insert((x, y)) {
                        return x.abs() + y.abs();
                    }
                }
            }
            2 => {
                for _ in 0..distance {
                    y -= 1;
                    // println!("{} {}", x, y);
                    if !visited.insert((x, y)) {
                        return x.abs() + y.abs();
                    }
                }
            }
            3 => {
                for _ in 0..distance {
                    x -= 1;
                    // println!("{} {}", x, y);
                    if !visited.insert((x, y)) {
                        return x.abs() + y.abs();
                    }
                }
            }
            _ => panic!(),
        }
        // println!("{} {}", x, y);
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2016/day_01_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 5);
    }

    #[test]
    fn part1_test2() {
        let input = fs::read_to_string("input/2016/day_01_test_02.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 2);
    }

    #[test]
    fn part1_test3() {
        let input = fs::read_to_string("input/2016/day_01_test_03.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 12);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2016/day_01_test_04.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 4);
    }
}
