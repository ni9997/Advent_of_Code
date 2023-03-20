use std::{collections::HashSet, fs, vec};

const DAY: usize = 9;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct Rope {
    xH: i32,
    yH: i32,
    xT: i32,
    yT: i32,
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn make_move(&mut self, dir: Direction, distance: u32) -> Result<(), ()> {
        match dir {
            Direction::LEFT => {
                for _ in 0..distance {

                }
            },
            Direction::RIGHT => {},
            Direction::UP => {},
            Direction::DOWN => {},
            
        }
        Ok(())
    }

    fn update_tail(&mut self) {
        
    }

    fn new() -> Rope {
        let mut temp = Rope {
            xH: 0,
            yH: 0,
            xT: 0,
            yT: 0,
            tail_visited: HashSet::new(),
        };
        temp.tail_visited.insert((temp.xT, temp.yT));
        temp
    }
}

pub fn part1(input: &str) -> usize {
    let mut rope = Rope::new();
    for line in input.split('\n') {
        let cmd = line.split(' ').collect::<Vec<&str>>();
        match (cmd[0], cmd[1].parse::<u32>().unwrap()) {
            ("R", distance) => {
                rope.make_move(Direction::RIGHT, distance).unwrap();
            }
            ("L", distance) => {
                rope.make_move(Direction::LEFT, distance).unwrap();
            }
            ("U", distance) => {
                rope.make_move(Direction::UP, distance).unwrap();
            }
            ("D", distance) => {
                rope.make_move(Direction::DOWN, distance).unwrap();
            }
            _ => {
                panic!()
            }
        }
    }
    rope.tail_visited.len()
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 13);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 8);
    }
}
