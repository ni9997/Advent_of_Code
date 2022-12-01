#[allow(dead_code)]
pub fn run() {
    println!("Day 20");
    let input = 33100000u32;
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &u32) -> usize {
    let n = input / 10;

    let mut presents = vec![0u32; n as usize + 1];

    for i in 1..(n + 1) {
        for j in (i..n + 1).step_by(i as usize) {
            presents[j as usize] += i * 10
        }
    }

    // println!("{:?}", presents);

    for (i, p) in presents.iter().enumerate() {
        if p >= input {
            return i;
        }
    }
    panic!("No solution found.")
}

pub fn part2(input: &u32) -> usize {
    let n = input / 10;

    let mut presents = vec![0u32; n as usize + 100];

    for i in 1..(n + 1) {
        let mut temp = 0;
        for j in (i..n + 1).step_by(i as usize) {
            presents[j as usize] += i * 11;
            temp += 1;
            if temp >= 50 {
                break;
            }
        }
    }

    // println!("{:?}", presents);

    for (i, p) in presents.iter().enumerate() {
        if p >= input {
            return i;
        }
    }
    panic!("No solution found.")
}

#[cfg(feature = "2015_01")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let t = part1(&70);
        assert_eq!(t, 4);
    }

    #[test]
    fn part1_test2() {
        let t = part1(&80);
        assert_eq!(t, 6);
    }

    #[test]
    fn part2_test1() {}

    #[test]
    fn part2_test2() {}
}
