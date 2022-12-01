use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 01");
    let input = fs::read_to_string("input/2022/day_01.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> u32 {
    let cal_elfs_strings = input.split("\n\n");
    let mut highest: u32 = 0;
    for calories_strings in cal_elfs_strings {
        let mut sum = 0;
        for cal in calories_strings.split('\n') {
            sum += cal.parse::<u32>().unwrap();
        }
        if sum > highest {
            highest = sum;
        }
    }
    highest
}

pub fn part2(input: &str) -> u32 {
    let cal_elfs_strings = input.split("\n\n");
    let mut calories: Vec<u32> = vec![];
    for calories_strings in cal_elfs_strings {
        let mut sum: u32 = 0;
        for cal in calories_strings.split('\n') {
            sum += cal.parse::<u32>().unwrap();
        }
        calories.push(sum);
    }
    calories.sort();
    calories[calories.len() - 3..].iter().sum()
}

#[cfg(feature = "2022_01")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_01_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 24000);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_01_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 45000);
    }
}
