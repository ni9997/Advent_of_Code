use std::fs;
use std::collections::HashMap;

pub fn run() {
    println!("Day 7");
    let content = fs::read_to_string("input/2015/day06.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

fn part1(input: &String) {
    let mut state: HashMap<&str, u16> = HashMap::new();

    for cmd in input.split("\n") {

        let inst = cmd.split("->");


    }
}

fn part2(input: &String) {

}