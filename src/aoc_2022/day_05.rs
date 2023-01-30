use std::{fs, vec};

#[allow(dead_code)]
pub fn run() {
    println!("Day 05");
    let input = fs::read_to_string("input/2022/day_05.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> String {
    let mut temp = input.split("\n\n");
    let init = temp.next().unwrap();
    let moves = temp.next().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![];

    let mut layers = init.lines().rev();
    let  fst_layer = layers.next().unwrap();

    let col = ((fst_layer.len()-3)/4)+1;
    for _ in 0..col {
        stacks.push(vec![]);
    }

    for layer in layers {
        // println!("{:?}", layer);
        // println!("{:?}", layer.len());
        for i in 0..col {
            let c = layer.chars().nth(1+i*4).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    // println!("{:?}", stacks);

    
    for m in moves.lines() {
        let mut cmd = m.split(' ');
        let amount = cmd.nth(1).unwrap().parse::<usize>().unwrap();
        let origin = cmd.nth(1).unwrap().parse::<usize>().unwrap()-1;
        let target = cmd.nth(1).unwrap().parse::<usize>().unwrap()-1;
        // println!("move {} from {} to {}", amount, origin, target);
        for _ in 0..amount {
            let c = stacks[origin].pop().unwrap();
            stacks[target].push(c);
        }
    }


    let mut top_crates = String::new();
    for mut s in stacks {
        top_crates.push(s.pop().unwrap());
    }
    top_crates
}

pub fn part2(input: &str) -> String {
    let mut temp = input.split("\n\n");
    let init = temp.next().unwrap();
    let moves = temp.next().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![];

    let mut layers = init.lines().rev();
    let  fst_layer = layers.next().unwrap();

    let col = ((fst_layer.len()-3)/4)+1;
    for _ in 0..col {
        stacks.push(vec![]);
    }

    for layer in layers {
        // println!("{:?}", layer);
        // println!("{:?}", layer.len());
        for i in 0..col {
            let c = layer.chars().nth(1+i*4).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    // println!("{:?}", stacks);

    
    for m in moves.lines() {
        let mut cmd = m.split(' ');
        let amount = cmd.nth(1).unwrap().parse::<usize>().unwrap();
        let origin = cmd.nth(1).unwrap().parse::<usize>().unwrap()-1;
        let target = cmd.nth(1).unwrap().parse::<usize>().unwrap()-1;
        // println!("move {} from {} to {}", amount, origin, target);
        let mut storage = vec![];
        for _ in 0..amount {
            let c = stacks[origin].pop().unwrap();
            storage.push(c);
        }
        for c in storage.iter().rev() {
            stacks[target].push(*c);
        }
    }


    let mut top_crates = String::new();
    for mut s in stacks {
        top_crates.push(s.pop().unwrap());
    }
    top_crates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, "CMZ");
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_05_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, "MCD");
    }
}
