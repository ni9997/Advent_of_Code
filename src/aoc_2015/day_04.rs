use md5::{self, Digest};

#[allow(dead_code)]
pub fn run() {
    println!("Day 4");
    let input = String::from("bgvyzdsv");
    part1(&input);
    part2(&input);
}

#[allow(dead_code)]
fn md5hash(x: &str) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(x);
    let s = format!("{:x}", hasher.finalize());
    s
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn part1(input: &String) {
    // let mut number = 0;
    // let mut s = format!("{input}{number}");
    // while !md5hash(s.as_str()).starts_with("00000") {
    //     number += 1;
    //     s = format!("{input}{number}");
    // }
    let number = 254575;
    println!("The number starting with five zeros: {number}");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn part2(input: &String) {
    // let mut number = 0;
    // let mut s = format!("{input}{number}");
    // while !md5hash(s.as_str()).starts_with("000000") {
    //     number += 1;
    //     s = format!("{input}{number}");
    // }
    let number = 1038736;
    println!("The number starting with six zeros: {number}");
}