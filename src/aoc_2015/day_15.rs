use std::{fs, cmp};

#[allow(dead_code)]
pub fn run() {
    println!("Day 15");
    let content = fs::read_to_string("input/2015/day15.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(_input: &String) {
    let mut max_score = 0;
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                for l in 0..100 {
                    if i+j+k+l == 100 {
                        let capacity = cmp::max(0,i*2+j*0+k*0+l*0);
                        let durability = cmp::max(0,i*0+j*5+k*0-l*1);
                        let flavour = cmp::max(0,-i*2 -j*3+k*5+l*0);
                        let texture = cmp::max(0,i*0+j*0+-k*1+l*5);
                        let score = cmp::max(0, capacity*durability*flavour*texture);
                        if score > max_score {
                            max_score = score;
                        }
                    }
                }
            }
        }
    }
    println!("Max score: {}", max_score);
}

#[allow(dead_code)]
fn part2(_input: &String) {
    let mut max_score = 0;
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                for l in 0..100 {
                    if i+j+k+l == 100 && i*3+j*3+k*8+l*8 == 500 {
                        let capacity = cmp::max(0,i*2+j*0+k*0+l*0);
                        let durability = cmp::max(0,i*0+j*5+k*0-l*1);
                        let flavour = cmp::max(0,-i*2 -j*3+k*5+l*0);
                        let texture = cmp::max(0,i*0+j*0+-k*1+l*5);
                        let score = cmp::max(0, capacity*durability*flavour*texture);
                        if score > max_score {
                            max_score = score;
                        }
                    }
                }
            }
        }
    }
    println!("Max score: {}", max_score);
}