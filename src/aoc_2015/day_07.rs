use std::fs;
use std::collections::HashMap;

pub fn run() {
    println!("Day 7");
    let content = fs::read_to_string("input/2015/day07.txt").expect("Wo Datei?");
    let test = String::from(
        "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i");
    part1(&content);
    part2(&content);
}

fn is_numeric(t: &str) -> bool {
    match t.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,        
    }
}

fn part1(input: &String) {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut state: HashMap<&str, u16> = HashMap::new();

    while !state.contains_key("a") {
        for line in lines.clone() {
            let mut x = line.split("->");
            let left = x.nth(0).unwrap().trim();
            let right = x.nth(0).unwrap().trim();
            if state.contains_key(right) {
                continue;
            } else {
                if left.contains("AND") {
                    let mut temp = left.split("AND");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv & rv);
                    } else {
                        continue;
                    }
                } else if left.contains("OR") {
                    let mut temp = left.split("OR");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv | rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("LSHIFT") {
                    let mut temp = left.split("LSHIFT");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv << rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("RSHIFT") {
                    let mut temp = left.split("RSHIFT");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv >> rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("NOT") {
                    let mut temp = left.split(" ");
                    let ll = temp.nth(1).unwrap().trim();
                    if state.contains_key(ll) || is_numeric(ll){
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        state.insert(right, !lv);
                    } else {
                        continue;
                    }                    
                } else {
                    if state.contains_key(left) || is_numeric(left){
                        let lv = match state.get(left) {
                            Some(i) => *i,
                            None => left.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv);
                    } else {
                        continue;
                    }                   
                }
            }
        }
    }
    println!("a={}", state.get("a").unwrap());
}

fn part2(input: &String) {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut state: HashMap<&str, u16> = HashMap::new();
    state.insert("b", 3176);

    while !state.contains_key("a") {
        for line in lines.clone() {
            let mut x = line.split("->");
            let left = x.nth(0).unwrap().trim();
            let right = x.nth(0).unwrap().trim();
            if state.contains_key(right) {
                continue;
            } else {
                if left.contains("AND") {
                    let mut temp = left.split("AND");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv & rv);
                    } else {
                        continue;
                    }
                } else if left.contains("OR") {
                    let mut temp = left.split("OR");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv | rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("LSHIFT") {
                    let mut temp = left.split("LSHIFT");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv << rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("RSHIFT") {
                    let mut temp = left.split("RSHIFT");
                    let ll = temp.nth(0).unwrap().trim();
                    let lr = temp.nth(0).unwrap().trim();
                    if (state.contains_key(ll) || is_numeric(ll)) &&  (state.contains_key(lr) || is_numeric(lr)) {
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        let rv = match state.get(lr) {
                            Some(i) => *i,
                            None => lr.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv >> rv);
                    } else {
                        continue;
                    }                    
                } else if left.contains("NOT") {
                    let mut temp = left.split(" ");
                    let ll = temp.nth(1).unwrap().trim();
                    if state.contains_key(ll) || is_numeric(ll){
                        let lv = match state.get(ll) {
                            Some(i) => *i,
                            None => ll.parse::<u16>().unwrap(),
                        };
                        state.insert(right, !lv);
                    } else {
                        continue;
                    }                    
                } else {
                    if state.contains_key(left) || is_numeric(left){
                        let lv = match state.get(left) {
                            Some(i) => *i,
                            None => left.parse::<u16>().unwrap(),
                        };
                        state.insert(right, lv);
                    } else {
                        continue;
                    }                   
                }
            }
        }
    }
    println!("a={}", state.get("a").unwrap());
}