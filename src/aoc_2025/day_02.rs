use crate::utils::get_input;

const YEAR: usize = 2025;
const DAY: usize = 2;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Debug)]
struct Range {
    low: usize,
    upper: usize,
}

impl Range {
    fn get_valid_ids_part1(&self) -> Vec<usize> {
        (self.low..self.upper + 1)
            .filter(|id| !is_valid_part1(*id))
            .collect()
    }

    fn get_valid_ids_part2(&self) -> Vec<usize> {
        (self.low..self.upper + 1)
            .filter(|id| !is_valid_part2(*id))
            .collect()
    }
}

fn get_ranges(input: &str) -> Vec<Range> {
    let mut ranges = vec![];

    for r in input.split(',') {
        let mut rs = r.split("-");
        ranges.push(Range {
            low: rs.next().unwrap().parse().unwrap(),
            upper: rs.next().unwrap().parse().unwrap(),
        });
    }

    ranges
}

fn is_valid_part1(id: usize) -> bool {
    let id_str = id.to_string();
    let (left, right) = id_str.split_at(id_str.len() / 2);
    left != right
}

fn is_valid_part2(id: usize) -> bool {
    let id_str = id.to_string();
    // println!(
    //     "Checking ID: {}, STR: {}, STR_LEN: {}",
    //     id,
    //     id_str,
    //     id_str.len()
    // );
    for i in (1..(id_str.len() / 2) + 1).rev() {
        let subs = id_str
            .as_bytes()
            .chunks(i)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        // println!("ID: {:10}, Chunksize: {}, Chunks: {:?}", id, i, subs);
        let mut equal = true;
        let first = subs[0];
        for s in &subs[1..] {
            // println!("FIRST: {}, SECOND: {}", first, *s);
            equal = *s == first && equal;
        }
        if equal {
            return false;
        }

    }
    // println!("VALID");
    true
}

pub fn part1(input: &str) -> usize {
    let test = get_ranges(input);
    let test2 = test
        .iter()
        .map(|r| r.get_valid_ids_part1().iter().sum::<usize>());
    test2.into_iter().sum()
}

pub fn part2(input: &str) -> usize {
    let test = get_ranges(input);
    let test2 = test
        .iter()
        .map(|r| r.get_valid_ids_part2().iter().sum::<usize>());
    test2.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 1227775554);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 4174379265);
    }
}
