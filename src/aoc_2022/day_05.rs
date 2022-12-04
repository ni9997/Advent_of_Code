use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 05");
    let input = fs::read_to_string("input/2022/day_05.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    panic!();
}

pub fn part2(input: &str) -> i32 {
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 2);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 4);
    }
}
