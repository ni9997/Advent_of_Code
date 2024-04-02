use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 10;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

fn get_next_pos(x: usize, y: usize, grid: &Vec<Vec<Option<Pipe>>>) -> (usize, usize) {
    todo!()
}

pub fn part1(input: &str) -> usize {
    let mut grid = Vec::with_capacity(input.lines().count());
    let mut start_x = None;
    let mut start_y = None;
    for (y, l) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(l.len());
        for (x, c) in l.chars().enumerate() {
            row.push(match c {
                '|' => Some(Pipe::Vertical),
                '-' => Some(Pipe::Horizontal),
                'L' => Some(Pipe::NorthEast),
                'J' => Some(Pipe::NorthWest),
                '7' => Some(Pipe::SouthWest),
                'F' => Some(Pipe::SouthEast),
                'S' => {
                    start_x = Some(x);
                    start_y = Some(y);
                    Some(Pipe::Start)
                }
                _ => None,
            })
        }
        grid.push(row);
    }
    let mut route = Vec::new();
    let mut current_x = start_x.unwrap();
    let mut current_y = start_y.unwrap();
    let mut last_x = None;
    let mut last_y = None;
    while let Some(pipe) = &grid[current_x][current_y] {
        route.push((current_x, current_y));
        match pipe {
            Pipe::Vertical => {
                if last_y.unwrap() < current_y {
                    current_y += 1;
                } else {
                    current_y -= 1;
                }
            }
            Pipe::Horizontal => {
                if last_x.unwrap() < current_x {
                    current_x += 1;
                } else {
                    current_x -= 1;
                }
            }
            Pipe::NorthEast => {
                if last_y.unwrap() > current_y {
                    current_x += 1;
                } else {
                    current_y -= 1;
                }
            }
            Pipe::NorthWest => {
                if last_y.unwrap() > current_y {
                    current_x -= 1;
                } else {
                    current_y -= 1;
                }
            }
            Pipe::SouthEast => {
                if last_y.unwrap() < current_y {
                    current_x += 1;
                } else {
                    current_y += 1;
                }
            }
            Pipe::SouthWest => {
                if last_y.unwrap() < current_y {
                    current_x += 1;
                } else {
                    current_y += 1;
                }
            }
            Pipe::Start => todo!(),
        };
        last_x = Some(current_x);
        last_y = Some(current_y);
        (current_x, current_y) = get_next_pos(current_x, current_y, &grid)
    }

    route.len() / 2
}

pub fn part2(input: &str) -> usize {
    todo!()
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
        assert_eq!(t, 4);
    }

    #[test]
    fn part1_test2() {
        let path = format!("input/{}/day_{:02}_test_02.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 8);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 2);
    }
}
