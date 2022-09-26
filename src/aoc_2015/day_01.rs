use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 1");
    part1();
    part2();
}

#[allow(dead_code)]
pub fn part1() {
    let contents = fs::read_to_string("input/2015/day01.txt").expect("Wo Datei?");
    let mut floor = 0;
    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    println!("Final floor: {floor}");
}

#[allow(dead_code)]
pub fn part2() {
    let contents = fs::read_to_string("input/2015/day01.txt").expect("Wo Datei?");
    let mut floor = 0;
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor < 0 {
            let pos = i + 1;
            println!("Position of entering basement: {pos}");
            break;
        }
    }
}
