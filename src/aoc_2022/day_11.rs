use std::fs;

const DAY: usize = 11;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

struct MonkeyGang {
    monkeys: Vec<Monkey>,
}

impl MonkeyGang {
    fn new(input: &str) -> MonkeyGang {
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
                .map(|x| Item {
                    worry_level: x.parse::<i32>().unwrap(),
                })
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
        }
        todo!()
    }

    fn make_round(&mut self) {
        for mut m in self.monkeys {
            m.turn();
        }
        todo!()
    }

    fn get_monkey_business(&self, amount: usize) -> usize {
        let mut test = self.monkeys.clone();
        test.sort_by(|a, b| a.inspection_count.cmp(&b.inspection_count));
        let test2 = test
            .iter()
            .take(amount)
            .map(|x| x.inspection_count)
            .product();
        test2
    }

    fn print_inspections(&self) {
        for (i, m) in self.monkeys.iter().enumerate() {
            println!("Monkey {:02} inspected {:04} items", i, m.inspection_count);
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
}

impl Monkey {
    fn new(items: Vec<Item>, test_divider: usize, operation: Operation) -> Monkey {
        Monkey {
            items,
            inspection_count: 0,
            test_divider,
            operation,
        }
    }

    fn turn(&mut self) {}

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
    let gang = MonkeyGang::new(input);
    gang.get_monkey_business(2)
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
