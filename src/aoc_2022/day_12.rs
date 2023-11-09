use core::fmt;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
        use std::cmp::{max, min};
        let mut neighbours = Vec::with_capacity(4);
        for x_i in max(x.saturating_sub(1), 0)..=min(x + 1, self.x_max) {
            if x_i == x {
                continue;
            }
            let node = self.row_vec[y][x_i].clone();
            neighbours.push(node);
        }
        for y_i in max(y.saturating_sub(1), 0)..=min(y + 1, self.y_max) {
            if y_i == y {
                continue;
            }
            let node = self.row_vec[y_i][x].clone();
            neighbours.push(node);
        }
        neighbours
    }

    fn solve(&self, x_start: usize, y_start: usize) -> usize {
        let mut to_solve = VecDeque::with_capacity(self.x_max * self.y_max);
        let start_node = self.row_vec[y_start][x_start].clone();
        to_solve.push_back(start_node);

        while let Some(test_node) = to_solve.pop_front() {
            let neig = self.get_neighbours(test_node.borrow().x, test_node.borrow().y);
            let valid_neig = neig.iter().filter(|x| {
                x.borrow().value - 1 <= test_node.borrow().value
                    && x.borrow().distance.borrow().is_none()
            });
            for n in valid_neig {
                if n.borrow().distance.borrow().is_none() {
                    *n.borrow().distance.borrow_mut() =
                        Some(test_node.borrow().distance.borrow().unwrap() + 1);
                    if n.borrow().node_type == NodeType::End {
                        return n.borrow().distance.borrow().unwrap();
                    } else {
                        to_solve.push_back(n.clone());
                    }
                }
            }
        }
        usize::MAX
    }

    fn reset(&self) {
        for row in &self.row_vec {
            for node in row {
                let n = node.borrow();
                match n.node_type {
                    NodeType::Start => {}
                    _ => *n.distance.borrow_mut() = None,
                }
            }
        }
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

#[derive(Debug, PartialEq)]
enum NodeType {
    Start,
    End,
    Normal,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    value: u8,
    x: usize,
    y: usize,
    distance: Rc<RefCell<Option<usize>>>,
}

impl Node {
    fn new(node_type: NodeType, value: char, x: usize, y: usize) -> Node {
        let distance = match node_type {
            NodeType::Start => Rc::new(RefCell::new(Some(0))),
            _ => Rc::new(RefCell::new(None)),
        };
        Node {
            node_type,
            value: value as u8,
            x,
            y,
            distance,
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

    let mut x_max = 0;
    let mut start_node = None;

    for (y, line) in lines.enumerate() {
        let mut line_nodes = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            line_nodes.push(match c {
                'S' => {
                    start_node = Some(Rc::new(RefCell::new(Node::new(NodeType::Start, 'a', x, y))));
                    start_node.clone().unwrap()
                }
                'E' => Rc::new(RefCell::new(Node::new(NodeType::End, 'z', x, y))),
                elevation => Rc::new(RefCell::new(Node::new(NodeType::Normal, elevation, x, y))),
            });
        }
        x_max = line_nodes.len();
        row_vec.push(line_nodes);
    }
    let y_max = row_vec.len();

    let grid = Grid {
        row_vec,
        x_max: x_max - 1,
        y_max: y_max - 1,
    };
    grid.solve(
        start_node.clone().unwrap().borrow().x,
        start_node.clone().unwrap().borrow().y,
    )
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines();
    let y_count = input.chars().filter(|x| *x == '\n').count() + 1;
    let mut row_vec = Vec::with_capacity(y_count);

    let mut x_max = 0;
    let mut start_nodes = vec![];

    for (y, line) in lines.enumerate() {
        let mut line_nodes = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            line_nodes.push(match c {
                'S' | 'a' => {
                    let start_node =
                        Some(Rc::new(RefCell::new(Node::new(NodeType::Start, 'a', x, y))));
                    start_nodes.push(start_node.clone().unwrap());
                    start_node.clone().unwrap()
                }
                'E' => Rc::new(RefCell::new(Node::new(NodeType::End, 'z', x, y))),
                elevation => Rc::new(RefCell::new(Node::new(NodeType::Normal, elevation, x, y))),
            });
        }
        x_max = line_nodes.len();
        row_vec.push(line_nodes);
    }
    let y_max = row_vec.len();

    let grid = Grid {
        row_vec,
        x_max: x_max - 1,
        y_max: y_max - 1,
    };
    start_nodes
        .iter()
        .map(|x| {
            let test_node = x.borrow();
            let result = grid.solve(test_node.x, test_node.y);
            grid.reset();
            result
        })
        .min()
        .unwrap()
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
        assert_eq!(t, 29);
    }
}
