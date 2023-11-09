use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::utils::get_input;

const YEAR: usize = 2022;
const DAY: usize = 12;

struct Grid {
    row_vec: Vec<Vec<Rc<RefCell<Node>>>>,
    x_max: usize,
    y_max: usize,
}

impl Grid {
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<Rc<RefCell<Node>>> {
        use std::cmp::{min, max};
        let mut neighbours = Vec::with_capacity(8);
        for x_i in max(x.saturating_sub(1), 0)..min(x+1, self.x_max) {
            for y_i in max(y.saturating_sub(1), 0)..min(y+1, self.y_max) {
                if x_i == x && y_i == y {
                    continue;
                }
                let node = self.row_vec[y_i][x_i].clone();
                neighbours.push(node);
            }
        }
        neighbours
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.row_vec {
            for n in row {
                write!(f, "{}", n.borrow())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum NodeType {
    Start,
    End,
    Normal,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    added: bool,
    next_possible: Vec<Node>,
    value: u8,
    x: usize,
    y:usize,
}

impl Node {
    fn new(node_type: NodeType, value: char, x:usize, y:usize) -> Node {
        Node {
            node_type,
            added: false,
            next_possible: vec![],
            value: value as u8,
            x,
            y,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.node_type {
                NodeType::Start => 'S',
                NodeType::End => 'E',
                NodeType::Normal => self.value as char,
            }
        )
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let lines = input.lines();
    let y_count = input.chars().filter(|x| *x == '\n').count() + 1;
    let mut row_vec = Vec::with_capacity(y_count);

    let mut start_x;
    let mut start_y;

    let mut x_max = 0;
    let y_max;

    for (y, line) in lines.enumerate() {
        let mut line_nodes = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            line_nodes.push(Rc::new(RefCell::new(match c {
                'S' => {
                    start_x = x;
                    start_y = y;
                    Node::new(NodeType::Start, 'a', x, y)
                }
                'E' => Node::new(NodeType::End, 'z', x, y),
                elevation => Node::new(NodeType::Normal, elevation, x, y),
            })));
        }
        x_max = line_nodes.len();
        row_vec.push(line_nodes);
    }
    y_max = row_vec.len();

    let grid = Grid { row_vec, x_max, y_max };

    println!("{}", grid);

    println!("{:?}", grid.get_neighbours(0, 0));

    todo!();
}

pub fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 31);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 2713310158);
    }
}
