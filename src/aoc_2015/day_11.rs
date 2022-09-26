#[allow(dead_code)]
pub fn run() {
    println!("Day 11");
    let content = String::from("hxbxwxba");
    // let content = String::from("ghijklmn");
    part1(&content);
    part2(&content);
}

fn increment_pw(pw: String) -> String {
    let mut temp: Vec<char> = pw.chars().collect::<Vec<char>>();
    // let temp_len = temp.len();
    *temp.last_mut().unwrap() = char::from_u32((*temp.last().unwrap() as u32) +1).unwrap();
    for j in 0..temp.len() {
        let i = temp.len()-1 - j;
        // println!("{i}");
        if temp[i] > 'z' && i >= 1{
            temp[i] = 'a';
            temp[i-1] = char::from_u32((temp[i-1] as u32) +1).unwrap();
        } 
    }
    temp.into_iter().collect::<String>()
}

fn check_straight(pw: &str) -> bool {
    let mut current_straight: usize = 0;
    let mut last_char = '{';
    for c in pw.chars() {
        if c as u32 == last_char as u32 +1 {
            current_straight += 1;
        } else {
            current_straight = 0;
        }
        if current_straight == 2 {
            return true;
        }
        last_char = c;
    }
    false
}

fn check_chars(pw: &str) -> bool {
    !pw.contains('i') && !pw.contains('o') && !pw.contains('l')  
}

fn check_pairs(pw: &str) -> bool {
    let mut last_char = '{';
    let mut found_first = false;
    for c in pw.chars() {
        if c == last_char && !found_first {
            found_first = true;
            last_char = '{';
            continue;
        } else if c == last_char && found_first { // && c != first {
            return true
        }
        last_char = c;
    }
    false
}

fn new_pw(input: &String) -> String {
    let mut pw = String::from(input);
    let mut valid = false;
    while !valid {
        pw = increment_pw(pw);
        // println!("Testing {}", pw);
        valid = check_straight(&pw) && check_chars(&pw) && check_pairs(&pw);
        // if pw == String::from("ghjaabcc") {
        //     break;
        // }
    }
    pw
}

#[allow(dead_code)]
fn part1(input: &String) {
    let pw = new_pw(input);
    println!("{} {} {}", check_straight(&pw), check_chars(&pw), check_pairs(&pw));
    println!("Found the new password: {}", pw);
}

#[allow(dead_code)]
fn part2(input: &String) {
    let pw = new_pw(&new_pw(input));
    println!("{} {} {}", check_straight(&pw), check_chars(&pw), check_pairs(&pw));
    println!("Found the new password: {}", pw);    
}