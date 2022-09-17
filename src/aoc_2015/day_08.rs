use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 8");
    let content = fs::read_to_string("input/2015/day08.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {

    let mut total = 0;
    let mut escaped = 0;

    for line in input.split("\n") {

        for i in 0..line.chars().count() {
            total += 1;

        }

    }

    println!("Total - escaped = {}", total-escaped);
    
}

#[allow(dead_code)]
fn part2(input: &String) {
    
}