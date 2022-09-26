use std::{fs};

#[allow(dead_code)]
pub fn run() {
    println!("Day 18");
    let content = fs::read_to_string("input/2015/day18.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

fn animate(start_state: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // println!("{:?}", start_state);
    let mut end_state: Vec<Vec<bool>> = vec![];
    let height = start_state.len() as i32;
    if height > 0 {
        let width = start_state[0].len() as i32;
        // println!("heigth={}, width={}", height, width);
        for i in 0..(height as i32) {
            end_state.push(vec![]);
            for j in 0..(width as i32) {
                let mut light_on_neig = 0;
                for ii in -1..2 {
                    for jj in -1..2 {
                        let test_i = i+ii;
                        let test_j = j+jj;
                        if test_i >= height || test_i < 0 || test_j < 0 || test_j >= width || (ii == 0 && jj == 0)  {
                            continue;
                        } else {
                            match start_state[test_i as usize][test_j as usize] {
                                true => {
                                    light_on_neig += 1;
                                },
                                false => {
                                }
                            }
                        }
                    }
                }
                // println!("{}:{} has {} on neighbours", i, j, light_on_neig);
                if start_state[i as usize][j as usize] {
                    if light_on_neig == 2 || light_on_neig == 3 {
                        end_state[i as usize].push(true);
                    } else {
                        end_state[i as usize].push(false);
                    }
                }
                else if light_on_neig == 3{
                    end_state[i as usize].push(true);
                } else {
                    end_state[i as usize].push(false);
                }
            }
        }
    } else {
        panic!("Not a valid input state")
    }
    end_state
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut state: Vec<Vec<bool>> = vec![];

    for line in input.split('\n') {
        let mut line_state: Vec<bool> = vec![];
        for c in line.chars() {
            line_state.push(match c {
                '.' => false,
                '#' => true,
                _ => panic!("Not a valid state")
            });
        }
        state.push(line_state);

    }
    for _i in 0..100 {
        state = animate(state);
        // println!("{:?}", state);
    }
    let mut lights_on: u32 = 0;
    let height: usize = state.len();
    if height > 0 {
        for i in state {
            for j in i {
                match j {
                    true => {
                        lights_on += 1;
                    },
                    false => {}
                }
            }
        }
    }
    println!("Lights on: {}", lights_on);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut state: Vec<Vec<bool>> = vec![];

    for line in input.split('\n') {
        let mut line_state: Vec<bool> = vec![];
        for c in line.chars() {
            line_state.push(match c {
                '.' => false,
                '#' => true,
                _ => panic!("Not a valid state")
            });
        }
        state.push(line_state);

    }
    let height = state.len();
    let width = state[0].len();
    state[0][0] = true;
    state[0][width-1] = true;
    state[height-1][0] = true;
    state[height-1][width-1] = true;
    for _i in 0..100 {
        state = animate(state);
        state[0][0] = true;
        state[0][width-1] = true;
        state[height-1][0] = true;
        state[height-1][width-1] = true;
        // println!("{:?}", state);
    }
    let mut lights_on: u32 = 0;
    let height: usize = state.len();
    if height > 0 {
        for i in state {
            for j in i {
                match j {
                    true => {
                        lights_on += 1;
                    },
                    false => {}
                }
            }
        }
    }
    println!("Lights on: {}", lights_on);

}