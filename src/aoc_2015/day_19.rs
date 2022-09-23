use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn run() {
    println!("Day 19");
    let content = fs::read_to_string("input/2015/day18.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut molecules: HashSet<String> = HashSet::new();
    let mut temp = input.split("\n\n");
    let  replacements = temp.nth(0).unwrap();
    let start_molecule = temp.nth(0).unwrap();

    for rep in replacements.split("\n") {

    }
}

#[allow(dead_code)]
fn part2(input: &String) {

}