use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 18");
    let content = fs::read_to_string("input/2015/day17.txt").expect("Wo Datei?");
    // let content = fs::read_to_string("input/2015/test.txt").expect("Wo Datei?");
    part1(&content);
    part2(&content);
}

#[derive(Clone)]
struct Container {
    size: u32,
}

impl Container {
    fn new(size: u32) -> Container {
        Container { size }
    }
}

struct Set {
    container: Vec<Container>,
    min: u32,
    combination: Vec<Vec<usize>>,
}

impl Set {
    fn possible_combinations(&mut self, volumen: u32) -> u32 {
        self.calc_possible_combinations(volumen, &mut vec![], 0)
    }

    fn calc_possible_combinations(
        &mut self,
        volumen: u32,
        used: &mut Vec<usize>,
        depth: u32,
    ) -> u32 {
        // println!("{:?} remaining: {}", used, volumen);
        if self.container.len() == used.len() {
            return 0;
        }
        let mut combinations: u32 = 0;
        for i in 0..self.container.len() {
            if used.contains(&i) {
                continue;
            }
            let c = &self.container[i];
            // println!("Container: {} volumen: {}", c.size, volumen);
            if c.size > volumen {
                continue;
            }
            if c.size == volumen {
                // println!("{i} vol:{volumen}");
                used.push(i);
                combinations += 1;
                if depth == 3 {
                    self.min = depth;
                    self.combination.push(used.to_vec());
                }
            } else {
                // print!("{i}->");
                let mut temp = used.to_vec();
                used.push(i);
                temp.push(i);
                combinations +=
                    self.calc_possible_combinations(volumen - c.size, &mut temp, depth + 1);
            }
        }
        combinations
    }
}

#[allow(dead_code)]
fn part1(input: &str) {
    let mut container: Vec<Container> = vec![];

    for line in input.split('\n') {
        container.push(Container::new(line.parse::<u32>().unwrap()));
    }
    let mut set: Set = Set {
        container,
        min: u32::MAX,
        combination: vec![],
    };
    println!("{} Combinations possible", set.possible_combinations(150));
}

#[allow(dead_code)]
fn part2(input: &str) {
    let mut container: Vec<Container> = vec![];

    for line in input.split('\n') {
        container.push(Container::new(line.parse::<u32>().unwrap()));
    }
    let mut set: Set = Set {
        container,
        min: u32::MAX,
        combination: vec![],
    };
    println!("{} Combinations possible", set.possible_combinations(150));
    println!(
        "Minimum number needed: {}. {} combinations possible",
        set.min,
        set.combination.len()
    );
}
