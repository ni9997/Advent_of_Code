use std::{fs};

#[allow(dead_code)]
pub fn run() {
    println!("Day 14");
    let content = fs::read_to_string("input/2015/day14.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}


#[derive(Debug)]
enum State {
    Running(u32),
    Rest(u32)
}


#[derive(Debug)]
struct Reindeer {
    state: State,
    distance: u32,
    rest_dur: u32,
    run_dur: u32,
    speed: u32,
    points: u32,
}

impl Reindeer {
    fn update(&mut self) {
        self. state = match self.state {
            State::Running(i) => {
                if i > 0 {
                    self.distance += self.speed;
                    State::Running(i-1)
                } else {
                    State::Rest(self.rest_dur-1)
                }
            },
            State::Rest(i) => {
                if i > 0 {
                    State::Rest(i-1)
                } else {
                    self.distance += self.speed;
                    State::Running(self.run_dur-1)
                }
            }
        };        
    }

    fn new(rest_dur: u32, run_dur: u32, speed: u32) -> Reindeer {
        let state = State::Running(run_dur);
        Reindeer { state, distance: 0, rest_dur, run_dur, speed, points: 0 }
    }
}

#[allow(dead_code)]
fn part1(input: &String) {

    let mut reindeers = vec![];

    for line in input.split("\n") {
        let mut temp = line.split(" ");
        let speed = temp.nth(3).unwrap().trim().parse::<u32>().unwrap();
        let run_dur = temp.nth(2).unwrap().trim().parse::<u32>().unwrap();
        let rest_dur = temp.nth(6).unwrap().trim().parse::<u32>().unwrap();
        reindeers.push(Reindeer::new(rest_dur, run_dur, speed));
    }

    for _i in 0..2503 {
        // if _i <= 200 {
        //     println!("{}: {:?}", _i, reindeers);
        // }
        for r in reindeers.iter_mut() {
            r.update();
        }
    }
    let mut longest:u32 = 0;
    for r in reindeers {
        println!("{:?}", r);
        if r.distance > longest {
            longest = r.distance
        }
    }
    println!("Longest distance: {}", longest);
    
}

#[allow(dead_code)]
fn part2(input: &String) {
    let mut reindeers = vec![];

    for line in input.split("\n") {
        let mut temp = line.split(" ");
        let speed = temp.nth(3).unwrap().trim().parse::<u32>().unwrap();
        let run_dur = temp.nth(2).unwrap().trim().parse::<u32>().unwrap();
        let rest_dur = temp.nth(6).unwrap().trim().parse::<u32>().unwrap();
        reindeers.push(Reindeer::new(rest_dur, run_dur, speed));
    }

    let len = reindeers.len();

    for _i in 0..2503 {
        // if _i <= 200 {
        //     println!("{}: {:?}", _i, reindeers);
        // }
        for i in 0..len {
            reindeers[i].update();
        }
        let mut longest:u32 = 0;
        for i in 0..len {
            if reindeers[i].distance > longest {
                longest = reindeers[i].distance
            }
        }
        for i in 0..len {
            if reindeers[i].distance == longest {
                reindeers[i].points += 1;
            }
        }
    }
    let mut most_points:u32 = 0;
    for r in reindeers {
        println!("{:?}", r);
        if r.points > most_points {
            most_points = r.points;
        }
    }
    println!("Winning reindeer has {} points!!", most_points);
}