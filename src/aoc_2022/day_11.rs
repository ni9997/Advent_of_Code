use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::utils::get_input;

const YEAR: usize = 2022;
const DAY: usize = 11;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

struct MonkeyGang {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
}

impl MonkeyGang {
    fn new(input: &str) -> MonkeyGang {
        let mut monkeys = vec![];
        let monkey_strings = input.split("\n\n");
        for monkey_string in monkey_strings {
            let mut lines = monkey_string.split('\n');
            // Monkey #:
            let _ = lines.next().expect("Not properly formattet input");
            // Starting items
            let items = lines
                .next()
                .expect("Not properly formattet input")
                .replace("  Starting items: ", "")
                .split(", ")
                .map(|x| Item::new(x.parse::<i32>().unwrap()))
                .collect::<Vec<Item>>();
            println!("{:?}", items);
            let op_line = lines
                .next()
                .expect("Not properly formattet input")
                .replace("  Operation: new = old ", "");
            let mut temp = op_line.split(' ');
            let op = (
                temp.next().expect("Wrong format"),
                temp.next().expect("Wrong format"),
            );
            let operation = match op {
                ("*", "old") => Operation::MultiplyOld,
                ("*", num) => Operation::MultiplyNumber(num.parse().unwrap()),
                ("+", num) => Operation::Add(num.parse().unwrap()),
                _ => {
                    panic!("Wrong format")
                }
            };
            println!("{:?}", operation);
            let test_divider = lines
                .next()
                .expect("Wrong format")
                .replace("  Test: divisible by ", "")
                .parse::<usize>()
                .unwrap();
            let true_monkey_target = lines
                .next()
                .expect("Wrong format")
                .replace("    If true: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();
            let false_monkey_target = lines
                .next()
                .expect("Wrong format")
                .replace("    If false: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();
            println!(
                "Div: {}, true target: {}, false target: {}",
                test_divider, true_monkey_target, false_monkey_target
            );
            monkeys.push(Rc::new(RefCell::new(Monkey::new(
                items,
                test_divider,
                operation,
            ))));
        }
        let gang = MonkeyGang { monkeys };
        gang
    }

    fn make_round(&mut self) {
        for m in &self.monkeys {
            let mut monkey = m.as_ref().borrow_mut();
            monkey.turn();
        }
        todo!()
    }

    fn run(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.make_round();
        }
    }

    fn get_monkey_business(&self, amount: usize) -> usize {
        let mut test = self.monkeys.clone();
        test.sort_by(|a, b| {
            a.as_ref()
                .borrow()
                .inspection_count
                .cmp(&b.as_ref().borrow().inspection_count)
        });
        let test2 = test
            .iter()
            .take(amount)
            .map(|x| x.as_ref().borrow().inspection_count)
            .product();
        test2
    }

    fn print_inspections(&self) {
        for (i, m) in self.monkeys.iter().enumerate() {
            println!(
                "Monkey {:02} inspected {:04} items",
                i,
                m.as_ref().borrow().inspection_count
            );
        }
    }

    fn update_monkeys(gang: Rc<RefCell<MonkeyGang>>) {
        let monkey_gang = gang.borrow_mut();
        for monkey in &monkey_gang.monkeys {
            let mut m = monkey.borrow_mut();
            m.monkey_gang = Some(gang.clone());
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    MultiplyNumber(i32),
    MultiplyOld,
    Add(i32),
}

enum DivisibleResult {
    True,
    False,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    inspection_count: usize,
    test_divider: usize,
    operation: Operation,
    monkey_gang: Option<Rc<RefCell<MonkeyGang>>>,
}

impl Monkey {
    fn new(items: Vec<Item>, test_divider: usize, operation: Operation) -> Monkey {
        Monkey {
            items,
            inspection_count: 0,
            test_divider,
            operation,
            monkey_gang: None,
        }
    }

    fn turn(&mut self) {
        while !self.items.is_empty() {
            let mut current_item = self.items.pop().unwrap();
            current_item.process_inspection(&self.operation);
        }
    }

    fn throw(item: Item, monkey: Monkey) {
        todo!()
    }

    fn inspect(&self, item: &mut Item) {
        item.process_inspection(&self.operation);
        todo!();
    }

    fn test(&self, item: Item) -> DivisibleResult {
        match item.worry_level % (self.test_divider as i32) == 0 {
            true => DivisibleResult::True,
            false => DivisibleResult::False,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Item {
    worry_level: i32,
}

impl Item {
    fn new(worry_level: i32) -> Item {
        Item { worry_level }
    }

    fn process_inspection(&mut self, operation: &Operation) {
        match operation {
            Operation::MultiplyNumber(num) => {
                self.worry_level *= num;
            }
            Operation::MultiplyOld => {
                self.worry_level *= self.worry_level;
            }
            Operation::Add(num) => {
                self.worry_level += num;
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let gang = Rc::new(RefCell::new(MonkeyGang::new(input)));
    MonkeyGang::update_monkeys(gang.clone());
    let mut temp = gang.borrow_mut();
    temp.run(20);
    let x = temp.get_monkey_business(2);
    x
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
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 10605);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 0);
    }
}
