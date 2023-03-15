use std::fs;

const DAY: usize = 8;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    todo!();
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 21);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 24933642);
    }
}
