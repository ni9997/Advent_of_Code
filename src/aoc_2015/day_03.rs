use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() {
    println!("Day 3");
    let content = fs::read_to_string("input/2015/day03.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut count_visited = 0;
    let mut visited_homes = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    visited_homes.insert((x, y), 1);
    count_visited += 1;

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Error: {c} is not supported")
        }
        if visited_homes.contains_key(&(x, y)) {
            continue;
        }
        else {
            visited_homes.insert((x, y), 1);
            count_visited += 1;
        }
    }
    println!("Visited {count_visited} homes")
}

#[allow(dead_code)]
fn part2(input: &String) {
    let mut count_visited = 0;
    let mut visited_homes = HashMap::new();
    let mut x_santa = 0;
    let mut y_santa = 0;
    let mut x_robot = 0;
    let mut y_robot = 0;

    visited_homes.insert((0, 0), 1);
    count_visited += 1;

    for (i, c) in input.chars().enumerate() {
        let mut x: i32;
        let mut y: i32;

        if i % 2 == 0 {
            x = x_santa;
            y = y_santa;
        }
        else {
            x = x_robot;
            y = y_robot;
        }

        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Error: {c} is not supported")
        }
        if !visited_homes.contains_key(&(x, y)) {
            visited_homes.insert((x, y), 1);
            count_visited += 1;
        }
        if i % 2 == 0 {
            x_santa = x;
            y_santa = y;
        }
        else {
            x_robot = x;
            y_robot = y;
        }
    }
    println!("Visited {count_visited} homes")
}