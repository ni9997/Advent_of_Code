use std::collections::HashMap;
use std::fs;

use std::cell::RefCell;
use std::rc::Rc;

const DAY: usize = 7;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Default, Debug)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    childs: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .childs
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn parse(input: &str) -> Rc<Dir> {
    let root = Rc::new(Dir::default());
    let mut cwd = Rc::clone(&root);
    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                dirname => {
                    let newdir = cwd.childs.borrow().get(dirname).unwrap().clone();
                    cwd = newdir;
                }
            },
            ("dir", dirname) => {
                cwd.childs.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        childs: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }
    root
}

pub fn part1(input: &str) -> usize {
    let mut to_visit = vec![Rc::clone(&parse(input))];
    let mut total_size = 0;
    while let Some(dir) = to_visit.pop() {
        for d in dir.childs.borrow().values() {
            to_visit.push(d.clone());
        }
        let size = dir.get_size();
        if size <= 100000 {
            total_size += size;
        }
    }
    total_size
}

pub fn part2(input: &str) -> usize {
    let total_space: usize = 70000000;
    let required: usize = 30000000;
    let root = parse(input);
    let minimum_dir_size = required - (total_space - root.get_size());

    let mut possible_delete_dir = vec![];
    let mut to_visit = vec![Rc::clone(&root)];
    while let Some(dir) = to_visit.pop() {
        for d in dir.childs.borrow().values() {
            to_visit.push(d.clone());
        }
        let size = dir.get_size();
        if size >= minimum_dir_size {
            possible_delete_dir.push(size);
        }
    }
    *possible_delete_dir.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 95437);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 24933642);
    }
}
