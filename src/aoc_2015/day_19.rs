use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn run() {
    println!("Day 19");
    let content = fs::read_to_string("input/2015/day19.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&content));
    println!("The result of part 2 is: {}", part2(&content));
}

fn get_variations(replace: &str, by: &str, molecule: &str) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    let test = molecule.match_indices(replace);

    for loc in test {
        let mut temp = String::new();
        if loc.0 != 0 {
            temp.push_str(&molecule[0..loc.0]);
        }
        temp.push_str(by);

        let old_idx = loc.0 + replace.len();
        if old_idx < molecule.len() {
            temp.push_str(&molecule[old_idx..]);
        }
        results.push(temp);
    }
    results
}

pub fn part1(input: &str) -> usize {
    let mut molecules: HashSet<String> = HashSet::new();
    let mut temp = input.split("\n\n");
    let replacements = temp.next().unwrap();
    let start_molecule = temp.next().unwrap();

    for rep in replacements.split('\n') {
        let mut temp = rep.split(" => ");
        let variations = get_variations(temp.next().unwrap(), temp.next().unwrap(), start_molecule);

        for temp in variations {
            molecules.insert(temp);
        }
    }
    molecules.len()
}

struct Replacement {
    replace: String,
    by: String,
}

pub fn part2(input: &str) -> usize {
    let mut temp = input.split("\n\n");
    let replacements = temp.next().unwrap();
    let mut molecule = String::from(temp.next().unwrap());
    let mut reps = vec![];

    for rep in replacements.split('\n') {
        let mut temp = rep.split(" => ");
        reps.push(Replacement {
            replace: String::from(temp.next().unwrap()),
            by: String::from(temp.next().unwrap()),
        });
    }

    let mut count: usize = 0;

    loop {
        let mut done = true;
        for r in &reps {
            if let Some(pos) = molecule.find(&r.by) {
                let (left, right) = molecule.split_at(pos);
                let right = right.to_string().split_off(r.by.len());
                molecule = format!("{}{}{}", left, r.replace, right);
                done = false;
                count += 1
            }
        }
        if done {
            break;
        }
    }
    count
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
        let test_string = fs::read_to_string("input/2015/day19_part1_test2.txt").unwrap();
        let r = part1(&test_string);
        assert_eq!(r, 7);
    }

    #[test]
    fn part2_test1() {
        let test_string = fs::read_to_string("input/2015/day19_part2_test1.txt").unwrap();
        let r = part2(&test_string);
        assert_eq!(r, 3);
    }

    #[test]
    fn part2_test2() {
        let test_string = fs::read_to_string("input/2015/day19_part2_test2.txt").unwrap();
        let r = part2(&test_string);
        assert_eq!(r, 6);
    }
}
