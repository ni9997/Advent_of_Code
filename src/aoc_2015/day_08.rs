use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 8");
    let content = fs::read_to_string("input/2015/day08.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
#[allow(clippy::if_same_then_else)]
fn part1(input: &str) {
    // let processed = input.replace("\\\\", "a").replace("\\\"", "a").replace("\"", "");

    let mut total = 0;

    for line in input.split('\n') {
        let chars = line.trim().chars().collect::<Vec<char>>();

        let mut i = 0;

        while i < chars.len() {
            if *chars.get(i).unwrap() == '\\' && *chars.get(i + 1).unwrap() == '\\' {
                total += 1;
                i += 1;
            } else if *chars.get(i).unwrap() == '\\' && *chars.get(i + 1).unwrap() == '"' {
                total += 1;
                i += 1;
            } else if *chars.get(i).unwrap() == '\\' && *chars.get(i + 1).unwrap() == 'x' {
                i += 3;
                total += 3;
            } else if *chars.get(i).unwrap() == '"' {
                total += 1
            } else {
                // total -=1;
            }
            i += 1;
        }
    }

    println!("Total = {}", total);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut total = 0;
    for line in input.split('\n') {
        let mut old = 0;
        let mut new = 2;
        let new_line = line.replace('\\', "\\\\").replace('\"', "\\\"");
        for _i in new_line.chars() {
            new += 1;
        }
        for _i in line.chars() {
            old += 1;
        }
        total += new - old;
    }
    println!("{total}");
}
