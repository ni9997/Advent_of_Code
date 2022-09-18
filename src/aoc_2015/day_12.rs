use std::fs;
use regex::Regex;
use serde_json::{Value};

#[allow(dead_code)]
pub fn run() {
    println!("Day 12");
    let content = fs::read_to_string("input/2015/day12.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut sum = 0;
    let test = Regex::new(r"\d+|-\d+").unwrap();
    for num in test.find_iter(input) {
        sum += num.as_str().parse::<i32>().unwrap();
    }
    println!("Total sum is: {}", sum);
}

fn sum_all(x: &Value) -> i64 {
    let mut sum = 0;
    if x.is_array() {
        for temp in x.as_array().unwrap() {
            // println!("{:?}", temp);
            if temp.is_number() {
                sum += temp.as_i64().unwrap();
            } else if temp.is_object() {
                sum += sum_all(temp);
            } else if temp.is_array() {
                sum += sum_all(temp);
            }
        }
    } else if x.is_object() {
        let mut red:bool = false;
        for (_k, v) in x.as_object().unwrap() {
            if v.is_number() {
                sum += v.as_i64().unwrap();
            } else if v.is_string() {
                if v.as_str().unwrap() == "red" {
                    red = true;
                }
            } else if x.is_object() {
                sum += sum_all(v);
            } else if x.is_array() {
                sum += sum_all(v);
            } else if v.is_string() {
                if v.as_str().unwrap() == "red" {
                    red = true;
                }
            } else {
                sum += 0;
            }
        }
        if red {
            sum = 0;
        }
    }
    sum
}

#[allow(dead_code)]
fn part2(input: &String) {
    let test: Value = serde_json::from_str(input).unwrap();  
    println!("Without red: {:?}", sum_all(&test));  
}