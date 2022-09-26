use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
pub fn run() {
    println!("Day 13");
    let content = fs::read_to_string("input/2015/day13.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut guests: HashSet<&str> = HashSet::new();
    let mut graph: HashMap<(&str, &str), i32> = HashMap::new();

    for x in input.split('\n') {
        let mut s = x.split(' ');
        let person1 = s.next().unwrap().trim();

        let value = match s.nth(1).unwrap() {
            "gain" => s.next().unwrap().parse::<i32>().unwrap(),
            "lose" => -s.next().unwrap().parse::<i32>().unwrap(),
            _ => 0,
        };

        let mut person2 = s.nth(6).unwrap().trim();
        person2 = &person2[..person2.len() - 1];
        guests.insert(person1);
        guests.insert(person2);
        graph.insert((person1, person2), value);
    }

    let mut max_happiness = i32::MIN;

    for comb in guests.iter().permutations(guests.len()) {
        // println!("{:?}", comb);
        let mut happiness = *graph
            .get(&(*comb.first().unwrap(), *comb.last().unwrap()))
            .unwrap();
        happiness += *graph
            .get(&(*comb.last().unwrap(), *comb.first().unwrap()))
            .unwrap();
        for i in 0..comb.len() - 1 {
            happiness += graph
                .get(&(*comb.get(i).unwrap(), *comb.get(i + 1).unwrap()))
                .unwrap();
            happiness += graph
                .get(&(*comb.get(i + 1).unwrap(), *comb.get(i).unwrap()))
                .unwrap();
        }
        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }
    println!("Max happiness: {}", max_happiness);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut guests: HashSet<&str> = HashSet::new();
    let mut graph: HashMap<(&str, &str), i32> = HashMap::new();

    guests.insert("Me");

    for x in input.split('\n') {
        let mut s = x.split(' ');
        let person1 = s.next().unwrap().trim();

        let value = match s.nth(1).unwrap() {
            "gain" => s.next().unwrap().parse::<i32>().unwrap(),
            "lose" => -s.next().unwrap().parse::<i32>().unwrap(),
            _ => 0,
        };

        let mut person2 = s.nth(6).unwrap().trim();
        person2 = &person2[..person2.len() - 1];
        guests.insert(person1);
        guests.insert(person2);
        graph.insert(("Me", person1), 0);
        graph.insert((person1, "Me"), 0);
        graph.insert(("Me", person2), 0);
        graph.insert((person2, "Me"), 0);
        graph.insert((person1, person2), value);
    }

    let mut max_happiness = i32::MIN;

    for comb in guests.iter().permutations(guests.len()) {
        // println!("{:?}", comb);
        let mut happiness = *graph
            .get(&(*comb.first().unwrap(), *comb.last().unwrap()))
            .unwrap();
        happiness += *graph
            .get(&(*comb.last().unwrap(), *comb.first().unwrap()))
            .unwrap();
        for i in 0..comb.len() - 1 {
            happiness += graph
                .get(&(*comb.get(i).unwrap(), *comb.get(i + 1).unwrap()))
                .unwrap();
            happiness += graph
                .get(&(*comb.get(i + 1).unwrap(), *comb.get(i).unwrap()))
                .unwrap();
        }
        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }
    println!("Max happiness: {}", max_happiness);
}
