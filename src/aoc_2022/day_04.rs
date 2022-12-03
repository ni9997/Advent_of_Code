use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 04");
    let input = fs::read_to_string("input/2022/day_04.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    let mut total_sum: i32 = 0;
    for bp in input.split('\n') {
        let left_bp = &bp[..bp.len() / 2];
        let right_bp = &bp[bp.len() / 2..];
        for lc in left_bp.chars() {
            let mut found = false;
            for rc in right_bp.chars() {
                if lc == rc {
                    // println!("{}", lc);
                    match lc {
                        'a'..='z' => total_sum += lc as i32 - 'a' as i32 + 1,
                        'A'..='Z' => total_sum += lc as i32 - 'A' as i32 + 27,
                        _ => panic!(),
                    }
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    total_sum
}

pub fn part2(input: &str) -> i32 {
    let mut total_sum = 0;
    let mut bps = input.split('\n');

    for i in 0..input.split('\n').count() / 3 {
        let mut found = false;
        // println!("{:?}", i);
        let f: Vec<char> = bps.next().unwrap().chars().collect();
        let s: Vec<char> = bps.next().unwrap().chars().collect();
        let t: Vec<char> = bps.next().unwrap().chars().collect();
        // break;
        for a in f {
            for b in &s {
                if a == *b {
                    for c in &t {
                        if *c == *b {
                            found = true;
                            match c {
                                'a'..='z' => total_sum += *c as i32 - 'a' as i32 + 1,
                                'A'..='Z' => total_sum += *c as i32 - 'A' as i32 + 27,
                                _ => panic!(),
                            }
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_04_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 157);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_04_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 70);
    }
}
