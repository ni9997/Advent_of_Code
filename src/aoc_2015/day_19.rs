use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn run() {
    println!("Day 19");
    let content = fs::read_to_string("input/2015/day19.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut molecules: HashSet<String> = HashSet::new();
    let mut temp = input.split("\n\n");
    let  replacements = temp.next().unwrap();
    let start_molecule = temp.next().unwrap();

    for rep in replacements.split('\n') {
        
        molecules.insert(String::from(start_molecule));
        molecules.insert(String::from(rep));
    }
    molecules.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    panic!("Not impl.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let test_string = fs::read_to_string("input/2015/day19_part1_test1.txt").unwrap();
        let r = part1(&test_string);
        assert_eq!(r, 4);
    }

    #[test]
    fn part1_test2() {
        let test_string = fs::read_to_string("input/2015/day19_part1_test1.txt").unwrap();
        let r = part1(&test_string);
        assert_eq!(r, 7);
    }
}