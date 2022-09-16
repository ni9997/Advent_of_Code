use std::fs;

pub fn run() {
    println!("Day 5");
    let content = fs::read_to_string("input/2015/day05.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

fn part1(input: &String) {
    let mut amount_of_nice_strings = 0;
    
    for test_string in input.split("\n") {
        // Test 1
        let mut vowels = 0;
        let mut last_char = '_';
        let mut contains_double_letter = false;
        for c in test_string.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => (),
            }
            if c == last_char {
                contains_double_letter = true;
            }
            last_char = c;
        }
        let mut contains_naughty_string: bool;
        contains_naughty_string = test_string.contains("ab");
        contains_naughty_string = contains_naughty_string || test_string.contains("pq");
        contains_naughty_string = contains_naughty_string || test_string.contains("cd");
        contains_naughty_string = contains_naughty_string || test_string.contains("xy");
        if vowels < 3 || !contains_double_letter || contains_naughty_string {
            continue;
        }
        amount_of_nice_strings += 1
    }
    println!("Count of nice rows: {amount_of_nice_strings}")
}

fn part2(input: &String) {
    let mut amount_of_nice_strings = 0;


    for test_string in input.split("\n") {

        let mut last_two: Vec<char> = vec!['_', '_'];
        let mut repeat = false;
        let mut last_char = '_';
        let mut rep_string: String;
        let mut test1 = false;

        // last_two.remove(0);
        for (i, c) in test_string.chars().enumerate() {

            if !test1 {
                rep_string = String::from(format!("{last_char}{c}"));
                test1 = match test_string[i+1..].find(rep_string.as_str()) {
                    Some(_) => true,
                    None => test1,
                };
            }

            if c == *last_two.first().unwrap() {
                repeat = true;
            }
            last_two.push(c);
            last_two.remove(0);
            last_char = c;
        }
        if test1 && repeat {
            amount_of_nice_strings += 1;
        }

    }
    println!("Count of nice rows: {amount_of_nice_strings}")
}