use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 2;

enum Game {
    Valid(usize),
    Power(usize),
    Invalid,
}

struct Qubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn ppower_game(game: &str) -> Game {
    let mut game = game.split(':');
    // let game_id = game.next().unwrap().replace("Game ", "").parse().unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for game in game.nth(1).unwrap().split(';') {
        for x in game.split(',') {
            let mut info = x.trim().split(' ');
            match (
                info.next().unwrap().trim().parse::<usize>().unwrap(),
                info.next().unwrap(),
            ) {
                (num, "blue") => {
                    if num > max_blue {
                        max_blue = num
                    }
                }
                (num, "red") => {
                    if num > max_red {
                        max_red = num
                    }
                }
                (num, "green") => {
                    if num > max_green {
                        max_green = num
                    }
                }
                _ => panic!(),
            }
        }
    }
    Game::Power(max_blue * max_green * max_red)
}

fn possible_game(game: &str, qubes: &Qubes) -> Game {
    let mut game = game.split(':');
    let game_id = game.next().unwrap().replace("Game ", "").parse().unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for game in game.next().unwrap().split(';') {
        for x in game.split(',') {
            let mut info = x.trim().split(' ');
            match (
                info.next().unwrap().trim().parse::<usize>().unwrap(),
                info.next().unwrap(),
            ) {
                (num, "blue") => {
                    if num > max_blue {
                        max_blue = num
                    }
                }
                (num, "red") => {
                    if num > max_red {
                        max_red = num
                    }
                }
                (num, "green") => {
                    if num > max_green {
                        max_green = num
                    }
                }
                _ => panic!(),
            }
        }
    }
    if max_red <= qubes.red && max_blue <= qubes.blue && max_green <= qubes.green {
        Game::Valid(game_id)
    } else {
        Game::Invalid
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
    let qubes = Qubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: usize = input
        .lines()
        .map(|x| match possible_game(x, &qubes) {
            Game::Invalid => 0,
            Game::Valid(id) => id,
            _ => panic!(),
        })
        .sum();
    sum
}

pub fn part2(input: &str) -> usize {
    let sum: usize = input
        .lines()
        .map(|x| match ppower_game(x) {
            Game::Invalid => panic!(),
            Game::Valid(_) => panic!(),
            Game::Power(power) => power,
        })
        .sum();
    sum
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
        assert_eq!(t, 8);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 2286);
    }
}
