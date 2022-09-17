use std::fs;
use std::cmp;

#[allow(dead_code)]
pub fn run() {
    println!("Day 2");
    let content = fs::read_to_string("input/2015/day02.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut amount = 0;

    for ps in input.split("\n") {
        if ps == "" {
            break;
        }
        let mut size = ps.split("x");
        let l = size.nth(0).unwrap().parse::<i32>().unwrap();
        let w = size.nth(0).unwrap().parse::<i32>().unwrap();
        let h = size.nth(0).unwrap().parse::<i32>().unwrap();
        let side1 = l*w;
        let side2 = w*h;
        let side3 = h*l;
        amount += 2*side1 + 2*side2 + 2*side3 + cmp::min(side1, cmp::min(side2, side3));
    }
    println!("Total amount of wrapping paper needed: {amount}");
}

#[allow(dead_code)]
fn part2(input: &String) {
    let mut amount = 0;

    for ps in input.split("\n") {
        if ps == "" {
            break;
        }
        let mut size = ps.split("x");
        let l = size.nth(0).unwrap().parse::<i32>().unwrap();
        let w = size.nth(0).unwrap().parse::<i32>().unwrap();
        let h = size.nth(0).unwrap().parse::<i32>().unwrap();
        let mut dim = vec![l, w, h];
        dim.sort();
        amount += 2*dim[0] + 2*dim[1] + l*h*w;
        
    }

    println!("Total amount of ribbon needed: {amount}");
}