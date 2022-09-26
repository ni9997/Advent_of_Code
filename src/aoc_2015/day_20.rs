#[allow(dead_code)]
pub fn run() {
    println!("Day 19");
    let input = 33100000u32;
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &u32) -> usize {
    for i in 0usize.. {
        let mut presents = 0;
        for elf in 1..i+1 {
            if i % elf == 0 {
                presents += elf * 10;
            }
        }
        if presents >= *input as usize {
            return i;
        }
        if i % 10000 == 0 {
            println!("House {i} gets {presents}");
        }
    }
    panic!("No solution found and infinite loop exited lol");
}

pub fn part2(input: &u32) -> usize {
    panic!("Not implemented")
}

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
