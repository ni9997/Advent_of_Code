use std::fs;

use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 06");
    let input = fs::read_to_string("input/2022/day_06.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

fn contains_douplicate(input: &[char]) -> bool {
    for c1 in input {
        let mut count = 0;
        for c2 in input {
            if c1 == c2 {
                count += 1;
            }
            if count > 1 {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> u32 {
    let temp = input.chars().collect_vec();
    let windows = temp.windows(4);
    for (i, w) in windows.enumerate() {
        if !contains_douplicate(w) {
            return i as u32 + 4;
        }
    }
    u32::MAX
}

pub fn part2(input: &str) -> u32 {
    let temp = input.chars().collect_vec();
    let windows = temp.windows(14);
    for (i, w) in windows.enumerate() {
        if !contains_douplicate(w) {
            return i as u32 + 14;
        }
    }
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(t, 5);
    }
    #[test]
    fn part1_test2() {
        // let input = fs::read_to_string("input/2022/day_06_test_02.txt").expect("Wo Datei?");
        let t = part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(t, 6);
    }
    #[test]
    fn part1_test3() {
        // let input = fs::read_to_string("input/2022/day_06_test_03.txt").expect("Wo Datei?");
        let t = part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(t, 10);
    }
    #[test]
    fn part1_test4() {
        // let input = fs::read_to_string("input/2022/day_06_test_04.txt").expect("Wo Datei?");
        let t = part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(t, 11);
    }
    #[test]
    fn part1_test5() {
        // let input = fs::read_to_string("input/2022/day_06_test_05.txt").expect("Wo Datei?");
        let t = part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(t, 7);
    }

    #[test]
    fn part2_test1() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(t, 19);
    }
    #[test]
    fn part2_test2() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(t, 23);
    }
    #[test]
    fn part2_test3() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(t, 23);
    }
    #[test]
    fn part2_test4() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(t, 29);
    }
    #[test]
    fn part2_test5() {
        // let input = fs::read_to_string("input/2022/day_06_test_01.txt").expect("Wo Datei?");
        let t = part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(t, 26);
    }
}
