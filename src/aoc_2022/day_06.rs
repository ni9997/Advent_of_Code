use std::{fs, vec};

#[allow(dead_code)]
pub fn run() {
    println!("Day 06");
    let input = fs::read_to_string("input/2022/day_06.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> u32 {
    todo!()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 5);
    }
    #[test]
    fn part1_test2() {
        let input = fs::read_to_string("input/2022/day_05_test_02.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 6);
    }
    #[test]
    fn part1_test3() {
        let input = fs::read_to_string("input/2022/day_05_test_03.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 10);
    }
    #[test]
    fn part1_test4() {
        let input = fs::read_to_string("input/2022/day_05_test_04.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 11);
    }
    #[test]
    fn part1_test5() {
        let input = fs::read_to_string("input/2022/day_05_test_05.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 7);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 0);
    }
}
