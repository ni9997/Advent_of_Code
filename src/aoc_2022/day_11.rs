use std::{cell::RefCell, rc::Rc};

use crate::utils::get_input;

const YEAR: usize = 2022;
const DAY: usize = 11;

type DividerType = i64;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Debug)]
struct MonkeyGang {
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    mod_enable: bool,
    mod_val: usize,
}

impl MonkeyGang {
    fn new(input: &str, divider: DividerType, mod_enable: bool) -> MonkeyGang {
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
                .map(|x| Item::new(x.parse::<DividerType>().unwrap(), divider))
                .collect::<Vec<Item>>();
            // println!("{:?}", items);
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
            // println!("{:?}", operation);
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
            // println!(
            //     "Div: {}, true target: {}, false target: {}",
            //     test_divider, true_monkey_target, false_monkey_target
            // );
            monkeys.push(Rc::new(RefCell::new(Monkey::new(
                items,
                test_divider,
                operation,
                true_monkey_target,
                false_monkey_target,
            ))));
        }
        let mod_val = match mod_enable {
            true => monkeys.iter().map(|x| x.borrow().test_divider).product(),
            false => 0,
        };
        // println!("Mod Val: {}", mod_val);
        MonkeyGang {
            monkeys,
            mod_val,
            mod_enable,
        }
    }

    fn make_round(&self) {
        for m in &self.monkeys {
            let mut monkey = m.as_ref().borrow_mut();
            monkey.turn();
        }
        // todo!()
    }

    fn run(&self, rounds: usize) {
        for _ in 0..rounds {
            self.make_round();
        }
        // for (i, m) in self.monkeys.iter().enumerate() {
        //     println!("Monkey {} inspected {} times", i, m.borrow().inspection_count);
        // }
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
            .rev()
            .take(amount)
            .map(|x| x.as_ref().borrow().inspection_count)
            .product();
        test2
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
    MultiplyNumber(DividerType),
    MultiplyOld,
    Add(DividerType),
}

enum DivisibleResult {
    True,
    False,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    inspection_count: usize,
    test_divider: usize,
    operation: Operation,
    monkey_gang: Option<Rc<RefCell<MonkeyGang>>>,
    true_monkey_target: usize,
    false_monkey_target: usize,
}

impl Monkey {
    fn new(
        items: Vec<Item>,
        test_divider: usize,
        operation: Operation,
        true_monkey_target: usize,
        false_monkey_target: usize,
    ) -> Monkey {
        Monkey {
            items,
            inspection_count: 0,
            test_divider,
            operation,
            monkey_gang: None,
            true_monkey_target,
            false_monkey_target,
        }
    }

    fn turn(&mut self) {
        while let Some(mut current_item) = self.items.pop() {
            self.inspection_count += 1;
            {
                let gang = self.monkey_gang.clone().unwrap();
                let gang = gang.borrow();
                current_item.process_inspection(&self.operation, &gang.mod_enable, &gang.mod_val);
            }
            match self.test(&current_item) {
                DivisibleResult::True => {
                    let monkeys = self.monkey_gang.clone().unwrap().clone();
                    let test = monkeys.borrow();
                    let test2 = &test.monkeys;
                    let test3 = test2[self.true_monkey_target].clone();
                    Self::throw(current_item, test3);
                }
                DivisibleResult::False => {
                    let monkeys = self.monkey_gang.clone().unwrap().clone();
                    let test = monkeys.borrow();
                    let test2 = &test.monkeys;
                    let test3: Rc<RefCell<Monkey>> = test2[self.false_monkey_target].clone();
                    Self::throw(current_item, test3);
                }
            }
        }
    }

    fn throw(item: Item, monkey: Rc<RefCell<Monkey>>) {
        monkey.borrow_mut().items.push(item);
    }

    fn test(&self, item: &Item) -> DivisibleResult {
        match item.worry_level % (self.test_divider as DividerType) == 0 {
            true => DivisibleResult::True,
            false => DivisibleResult::False,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Item {
    worry_level: DividerType,
    divider: DividerType,
}

impl Item {
    fn new(worry_level: DividerType, divider: DividerType) -> Item {
        Item {
            worry_level,
            divider,
        }
    }

    fn process_inspection(&mut self, operation: &Operation, mod_enable: &bool, mod_val: &usize) {
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
        self.worry_level /= self.divider;
        if *mod_enable {
            self.worry_level %= *mod_val as i64;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let gang = Rc::new(RefCell::new(MonkeyGang::new(input, 3, false)));
    MonkeyGang::update_monkeys(gang.clone());
    let temp = gang.borrow();
    temp.run(20);

    temp.get_monkey_business(2)
}

pub fn part2(input: &str) -> usize {
    let gang = Rc::new(RefCell::new(MonkeyGang::new(input, 1, true)));
    MonkeyGang::update_monkeys(gang.clone());
    let temp = gang.borrow();
    temp.run(10_000);

    temp.get_monkey_business(2)
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
        assert_eq!(t, 2713310158);
    }
}
