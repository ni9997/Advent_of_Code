use std::{collections::HashSet, fs};

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

#[derive(Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    
    fn update(&mut self, pervious: &Knot) {
        if (self.x - pervious.x).abs() + (self.y - pervious.y).abs() <= 2 {
            if (pervious.x - self.x) > 1 {
                self.x = self.x + 1;
            } else if (pervious.x - self.x) < -1 {
                self.x = self.x - 1;
            } else if (pervious.y - self.y) > 1 {
                self.y = self.y + 1;
            } else if (pervious.y - self.y) < -1 {
                self.y = self.y - 1;
            }
        } else {
            self.x = self.x + (pervious.x - self.x).signum();
            self.y = self.y + (pervious.y - self.y).signum();
        }
    }

    fn make_move(&mut self, dir: &Direction) {
        match dir {
            Direction::LEFT => self.x = self.x - 1,
            Direction::RIGHT => self.x = self.x + 1,
            Direction::UP => self.y = self.y + 1,
            Direction::DOWN => self.y = self.y - 1,
        }
    }
}

struct Rope {
    xH: i32,
    yH: i32,
    xT: i32,
    yT: i32,
    knots: Vec<Knot>,
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn make_move(&mut self, dir: Direction, distance: u32) -> Result<(), ()> {
        for _ in 0..distance {
            self.knots[0].make_move(&dir);
            self.update_tail();
            self.tail_visited
                .insert((self.knots.last().unwrap().x, self.knots.last().unwrap().y));
        }
        Ok(())
    }

    fn update_tail(&mut self) {
        for i in 1..self.knots.len() {
            let prev = self.knots[i-1];
            self.knots[i].update(&prev);
        }
        
        self.tail_visited.insert((self.xT, self.yT));
    }

    fn new(knots: usize) -> Rope {
        let mut k = Vec::with_capacity(knots + 1);
        for _ in 0..knots + 1 {
            k.push(Knot { x: 0, y: 0 })
        }
        let mut temp = Rope {
            xH: 0,
            yH: 0,
            xT: 0,
            yT: 0,
            knots: k,
            tail_visited: HashSet::new(),
        };
        temp.tail_visited.insert((temp.xT, temp.yT));
        temp
    }
}

pub fn part1(input: &str) -> usize {
    let mut rope = Rope::new(1);
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
    // println!("{:?}", rope.tail_visited);
    rope.tail_visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut rope = Rope::new(9);
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
    // println!("{:?}", rope.tail_visited);
    rope.tail_visited.len()
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
        assert_eq!(t, 1);
    }

    #[test]
    fn part2_test2() {
        let path = format!("input/2022/day_{:02}_test_02.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 36);
    }
}
