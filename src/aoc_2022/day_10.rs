use std::{collections::HashMap, fs};

const DAY: usize = 10;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is:\n{}", part2(&input));
}

struct Cpu {
    register: HashMap<String, i64>,
    cycle: usize,
    instructions: Vec<Box<dyn Instruction>>,
    val: i64,
}

impl Cpu {
    fn new(instructions: Vec<Box<dyn Instruction>>) -> Cpu {
        let mut register = HashMap::new();
        register.insert("x".to_string(), 1);
        Cpu {
            register,
            cycle: 0,
            instructions,
            val: 0,
        }
    }

    fn process(&mut self) -> i64 {
        while !self.instructions.is_empty() {
            self.cycle += 1;
            // self.instructions[0].process(&mut self.register);
            // println!("{} {:?}", self.cycle, self.instructions[0].to_string());
            if (self.cycle + 20) % 40 == 0 {
                self.val += self.cycle as i64 * self.register["x"];
                // println!(
                //     "Cycle: {} Register: {}, Val: {}, Inc: {}",
                //     self.cycle,
                //     self.register["x"],
                //     self.val,
                //     self.cycle as i64 * self.register["x"]
                // );
            }
            self.instructions[0].process(&mut self.register);
            if self.instructions[0].get_cycle_duration() == 0 {
                self.instructions.remove(0);
            }
        }
        self.val
    }

    fn draw(&mut self) -> String {
        // let mut output = String::new();
        let mut r = String::new();
        let mut line = String::new();
        while !self.instructions.is_empty() {
            self.cycle += 1;
            // println!("{}", self.register["x"]);
            if (self.register["x"] - ((self.cycle as i64 - 1) % 40)).abs() < 2 {
                line.push('#');
            } else {
                line.push('.');
            }
            self.instructions[0].process(&mut self.register);
            if self.instructions[0].get_cycle_duration() == 0 {
                self.instructions.remove(0);
            }
            if self.cycle % 40 == 0 {
                // output.push_str(
                //     format!(
                //         "Cycle {:03} -> {} <- Cycle {:03}\n",
                //         self.cycle - 39,
                //         line,
                //         self.cycle
                //     )
                //     .as_str(),
                // );
                r.push_str(format!("{}\n", line).as_str());
                line.clear();
            }
        }
        // println!("{}", output);
        r.trim().to_string()
    }
}

trait Instruction {
    fn get_cycle_duration(&self) -> usize;
    fn process(&mut self, register: &mut HashMap<String, i64>);
    fn to_string(&self) -> String;
}

struct Noop {
    cycles: usize,
}
struct Addx {
    cycles: usize,
    val: i64,
}

impl Instruction for Noop {
    fn get_cycle_duration(&self) -> usize {
        self.cycles
    }

    fn process(&mut self, _register: &mut HashMap<String, i64>) {
        self.cycles -= 1;
    }

    fn to_string(&self) -> String {
        "NOOP".to_string()
    }
}

impl Instruction for Addx {
    fn get_cycle_duration(&self) -> usize {
        self.cycles
    }

    fn process(&mut self, register: &mut HashMap<String, i64>) {
        if self.cycles == 1 {
            register.insert("x".to_string(), register["x"] + self.val);
        }
        self.cycles -= 1;
    }

    fn to_string(&self) -> String {
        format!("ADDX {} remaining cycles: {}", self.val, self.cycles)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut instructions: Vec<Box<dyn Instruction>> =
        Vec::with_capacity(input.chars().filter(|x| *x == '\n').count() + 1);
    for l in input.split('\n') {
        // println!("{}", l);
        let cmd = l.split(' ').collect::<Vec<&str>>();
        // println!("{:?}", cmd);
        match cmd[0] {
            "addx" => {
                instructions.push(Box::new(Addx {
                    val: cmd[1].parse().unwrap(),
                    cycles: 2,
                }));
            }
            "noop" => {
                instructions.push(Box::new(Noop { cycles: 1 }));
            }
            _ => {
                panic!();
            }
        }
    }
    let mut cpu = Cpu::new(instructions);
    cpu.process()
}

pub fn part2(input: &str) -> String {
    let mut instructions: Vec<Box<dyn Instruction>> =
        Vec::with_capacity(input.chars().filter(|x| *x == '\n').count() + 1);
    for l in input.split('\n') {
        // println!("{}", l);
        let cmd = l.split(' ').collect::<Vec<&str>>();
        // println!("{:?}", cmd);
        match cmd[0] {
            "addx" => {
                instructions.push(Box::new(Addx {
                    val: cmd[1].parse().unwrap(),
                    cycles: 2,
                }));
            }
            "noop" => {
                instructions.push(Box::new(Noop { cycles: 1 }));
            }
            _ => {
                panic!();
            }
        }
    }
    let mut cpu = Cpu::new(instructions);
    cpu.draw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 13140);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(
            t,
            String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            )
        );
    }
}
