use std::cmp;
use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 6");
    let content = fs::read_to_string("input/2015/day06.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
#[allow(clippy::bool_to_int_with_if)]
fn part1(input: &str) {
    let mut lights = [[0u8; 1000]; 1000];

    for cmd in input.split('\n') {
        let mut s = cmd.split(' ');
        let mode = match s.next().unwrap() {
            "toggle" => 2,
            "turn" => {
                if s.next().unwrap() == "on" {
                    1
                } else {
                    0
                }
            }
            _ => -1,
        };
        let c1 = s.next().unwrap();
        let c1_x = c1.split(',').next().unwrap().parse::<i32>().unwrap();
        let c1_y = c1.split(',').nth(1).unwrap().parse::<i32>().unwrap();
        s.next();
        let c2 = s.next().unwrap();
        let c2_x = c2.split(',').next().unwrap().parse::<i32>().unwrap();
        let c2_y = c2.split(',').nth(1).unwrap().parse::<i32>().unwrap();

        for i in c1_x..c2_x + 1 {
            for j in c1_y..c2_y + 1 {
                lights[i as usize][j as usize] = match mode {
                    0 => 0,
                    1 => 1,
                    2 => match lights[i as usize][j as usize] {
                        0 => 1,
                        1 => 0,
                        _ => panic!(""),
                    },
                    _ => panic!(""),
                }
            }
        }
    }
    let mut lit_lights = 0;
    for i in lights {
        for j in i {
            if j == 1 {
                lit_lights += 1;
            }
        }
    }
    println!("{lit_lights} lights lit");
}

#[allow(dead_code)]
#[allow(clippy::bool_to_int_with_if)]
fn part2(input: &str) {
    let mut lights = [[0i64; 1000]; 1000];

    for cmd in input.split('\n') {
        let mut s = cmd.split(' ');
        let mode = match s.next().unwrap() {
            "toggle" => 2,
            "turn" => {
                if s.next().unwrap() == "on" {
                    1
                } else {
                    0
                }
            }
            _ => -1,
        };
        let c1 = s.next().unwrap();
        let c1_x = c1.split(',').next().unwrap().parse::<i32>().unwrap();
        let c1_y = c1.split(',').nth(1).unwrap().parse::<i32>().unwrap();
        s.next();
        let c2 = s.next().unwrap();
        let c2_x = c2.split(',').next().unwrap().parse::<i32>().unwrap();
        let c2_y = c2.split(',').nth(1).unwrap().parse::<i32>().unwrap();

        for i in c1_x..c2_x + 1 {
            for j in c1_y..c2_y + 1 {
                lights[i as usize][j as usize] = match mode {
                    0 => cmp::max(0, lights[i as usize][j as usize] - 1),
                    1 => cmp::max(0, lights[i as usize][j as usize] + 1),
                    2 => cmp::max(0, lights[i as usize][j as usize] + 2),
                    _ => panic!(""),
                }
            }
        }
    }
    let mut brightness = 0;
    for i in lights {
        for j in i {
            brightness += j;
        }
    }
    println!("{brightness} brightness");
}
