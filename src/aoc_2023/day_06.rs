use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 6;

enum Part {
    One,
    Two,
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_beatable_ways_number(&self) -> usize {
        let mut win_count = 0;
        for charge_time in 1..=self.time {
            let dist = (self.time - charge_time) * charge_time;
            if dist > self.distance {
                win_count += 1;
            }
        }
        win_count
    }
}

fn parse(input: &str, part: Part) -> Vec<Race> {
    match part {
        Part::One => {
            let mut races = Vec::new();
            let mut input = input.split('\n');
            let time_line = input.next().unwrap();
            let dist_line = input.next().unwrap();
            let times = time_line.split_whitespace().skip(1);
            let dists = dist_line.split_whitespace().skip(1);

            for (t, d) in times.zip(dists) {
                println!("Race: time={}ms dist={}mm", t, d);
                races.push(Race {
                    time: t.parse().unwrap(),
                    distance: d.parse().unwrap(),
                })
            }

            races
        }
        Part::Two => {
            let mut input = input.split('\n');
            let time_line = input.next().unwrap().replace("Time:", "").replace(' ', "");
            let dist_line = input
                .next()
                .unwrap()
                .replace("Distance:", "")
                .replace(' ', "");
            vec![Race {
                time: time_line.parse().unwrap(),
                distance: dist_line.parse().unwrap(),
            }]
        }
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
    let races = parse(input, Part::One);
    races.iter().map(|r| r.get_beatable_ways_number()).product()
}

pub fn part2(input: &str) -> usize {
    let races = parse(input, Part::Two);
    races.iter().map(|r| r.get_beatable_ways_number()).product()
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
        assert_eq!(t, 288);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 71503);
    }
}
