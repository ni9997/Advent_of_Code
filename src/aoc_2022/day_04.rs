use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 04");
    let input = fs::read_to_string("input/2022/day_04.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    for pairs in input.split('\n') {
        let mut pair_split = pairs.split(',');
        let mut left = pair_split.next().unwrap().split('-');
        let left_min = left.next().unwrap().parse::<i32>().unwrap();
        let left_max = left.next().unwrap().parse::<i32>().unwrap();
        let mut right = pair_split.next().unwrap().split('-');
        let right_min = right.next().unwrap().parse::<i32>().unwrap();
        let right_max = right.next().unwrap().parse::<i32>().unwrap();

        if (left_min <= right_min && left_max >= right_max)
            || (right_min <= left_min && right_max >= left_max)
        {
            total += 1;
        }
    }
    total
}

pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    for pairs in input.split('\n') {
        let mut pair_split = pairs.split(',');
        let mut left = pair_split.next().unwrap().split('-');
        let left_min = left.next().unwrap().parse::<i32>().unwrap();
        let left_max = left.next().unwrap().parse::<i32>().unwrap();
        let mut right = pair_split.next().unwrap().split('-');
        let right_min = right.next().unwrap().parse::<i32>().unwrap();
        let right_max = right.next().unwrap().parse::<i32>().unwrap();

        if (left_min <= right_max && left_max >= right_min)
            || (right_min <= left_max && right_max >= left_min)
        {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_04_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 2);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_04_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 4);
    }
}
