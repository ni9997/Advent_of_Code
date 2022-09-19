use std::{fs};

#[allow(dead_code)]
pub fn run() {
    println!("Day 16");
    let content = fs::read_to_string("input/2015/day16.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[allow(dead_code)]
fn part1(input: &String) {
    
    for (i, line) in input.split("\n").enumerate() {
        let compounds = String::from(line).replace(format!("Sue {}:", i+1).as_str(), "");
        // println!("{}", line);
        // println!("{}", compounds);
        let mut found = true;
        for compound in compounds.split(",") {
            let key = compound.split(":").nth(0).unwrap().trim();
            let value = compound.split(":").nth(1).unwrap().trim().parse::<u32>().unwrap();
            // println!("Key={key}. value={value}");
            match key {
                "children" => {
                    found = found && value == 3;
                },
                "cats" => {
                    found = found && value == 7;
                },
                "samoyeds" => {
                    found = found && value == 2;
                },
                "pomeranians" => {
                    found = found && value == 3;
                },
                "akitas" => {
                    found = found && value == 0;
                },
                "vizslas" => {
                    found = found && value == 0;
                },
                "goldfish" => {
                    found = found && value == 5;
                },
                "trees" => {
                    found = found && value == 3;
                },
                "cars" => {
                    found = found && value == 2;
                },
                "perfumes" => {
                    found = found && value == 1;
                },
                _ => panic!()
            }
        }
        if found {
            println!("Its Sue {}", i+1);
        }
    }
    
}

#[allow(dead_code)]
fn part2(input: &String) {

    for (i, line) in input.split("\n").enumerate() {
        let compounds = String::from(line).replace(format!("Sue {}:", i+1).as_str(), "");
        // println!("{}", line);
        // println!("{}", compounds);
        let mut found = true;
        for compound in compounds.split(",") {
            let key = compound.split(":").nth(0).unwrap().trim();
            let value = compound.split(":").nth(1).unwrap().trim().parse::<u32>().unwrap();
            // println!("Key={key}. value={value}");
            match key {
                "children" => {
                    found = found && value == 3;
                },
                "cats" => {
                    found = found && value > 7;
                },
                "samoyeds" => {
                    found = found && value == 2;
                },
                "pomeranians" => {
                    found = found && value < 3;
                },
                "akitas" => {
                    found = found && value == 0;
                },
                "vizslas" => {
                    found = found && value == 0;
                },
                "goldfish" => {
                    found = found && value < 5;
                },
                "trees" => {
                    found = found && value > 3;
                },
                "cars" => {
                    found = found && value == 2;
                },
                "perfumes" => {
                    found = found && value == 1;
                },
                _ => panic!()
            }
        }
        if found {
            println!("Its Sue {}", i+1);
        }
    }
    
}