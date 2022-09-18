#[allow(dead_code)]
pub fn run() {
    println!("Day 10");
    let content = String::from("1113222113");
    part1(&content);
    part2(&content);
}

fn look_and_say(input: String) -> String {

    let mut chars: Vec<Vec<char>> = vec![];
    let mut cur_char = ' ';
    for c in input.chars() {
        if c == cur_char {
            chars.last_mut().unwrap().push(c);
        } else {
            chars.push(vec![c]);
        }
        cur_char = c;
    }
    // println!("{:?}", chars);
    let mut x = String::from("");
    for b in chars {
        x.insert_str(x.len(), format!("{}{}", b.len(), b.first().unwrap()).as_str());
    }
    x
}

#[allow(dead_code)]
fn part1(input: &String) {
    let mut temp: String = String::from(input);
    for _i in 0..40 {
        temp = look_and_say(temp);
    }
    // println!("{}", look_and_say(temp));
    println!("Length of out is {}", temp.len());
}

#[allow(dead_code)]
fn part2(input: &String) {
    let mut temp: String = String::from(input);
    for _i in 0..50 {
        temp = look_and_say(temp);
    }
    // println!("{}", look_and_say(temp));
    println!("Length of out is {}", temp.len());
}