use std::fs;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 9");
    let content = fs::read_to_string("input/2015/day09.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut stations: HashSet<&str> = HashSet::new();
    let mut graph: HashMap<(&str, &str), u32> = HashMap::new();
    let mut shortest_dist = u32::MAX;

    for line in input.split("\n") {
        let left = line.split("=").nth(1).unwrap().trim().parse::<u32>().unwrap();
        let mut right = line.split("=").nth(0).unwrap().trim().split("to");
        let one = right.nth(0).unwrap().trim();
        let two = right.nth(0).unwrap().trim();
        graph.insert((one, two), left);
        graph.insert((two, one), left);
        // graph.insert((two, two), 0);
        // graph.insert((one, one), 0);
        stations.insert(one);
        stations.insert(two);
    }
    for (i, x) in stations.iter().permutations(stations.len()).enumerate() {
        // println!("{:?}", x);
        let mut dist = 0;
        for i in 0..x.len()-1 {
            let station1 = **x.get(i).unwrap();
            let station2 = **x.get(i+1).unwrap();
            // println!("{} to {}", station1, station2);
            dist += graph.get(&(station1, station2)).unwrap();
        }
        if dist < shortest_dist {
            shortest_dist = dist;
        }
        println!("Dist[{i}]= {}", shortest_dist);
    }
    println!("{shortest_dist}");
}

#[allow(dead_code)]
fn part2(input: &String) {
    let mut stations: HashSet<&str> = HashSet::new();
    let mut graph: HashMap<(&str, &str), u32> = HashMap::new();
    let mut longest_dist = u32::MIN;

    for line in input.split("\n") {
        let left = line.split("=").nth(1).unwrap().trim().parse::<u32>().unwrap();
        let mut right = line.split("=").nth(0).unwrap().trim().split("to");
        let one = right.nth(0).unwrap().trim();
        let two = right.nth(0).unwrap().trim();
        graph.insert((one, two), left);
        graph.insert((two, one), left);
        // graph.insert((two, two), 0);
        // graph.insert((one, one), 0);
        stations.insert(one);
        stations.insert(two);
    }
    for (i, x) in stations.iter().permutations(stations.len()).enumerate() {
        // println!("{:?}", x);
        let mut dist = 0;
        for i in 0..x.len()-1 {
            let station1 = **x.get(i).unwrap();
            let station2 = **x.get(i+1).unwrap();
            // println!("{} to {}", station1, station2);
            dist += graph.get(&(station1, station2)).unwrap();
        }
        if dist > longest_dist {
            longest_dist = dist;
        }
        println!("Dist[{i}]= {}", longest_dist);
    }
    println!("{longest_dist}");
}