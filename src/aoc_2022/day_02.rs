use std::fs;

#[allow(dead_code)]
pub fn run() {
    println!("Day 02");
    let input = fs::read_to_string("input/2022/day_02.txt").expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> i32 {
    let mut score = 0;
    for round in input.split('\n') {
        let mut player_choices = round.split(' ');
        let opponent_choice =
            player_choices.next().unwrap().chars().next().unwrap() as i32 - '@' as i32;
        let my_choice = player_choices.next().unwrap().chars().next().unwrap() as i32 - 'W' as i32;
        score += my_choice;
        match opponent_choice - my_choice {
            0 => score += 3,
            1 => {}
            -1 => score += 6,
            2 => score += 6,
            -2 => {}
            _ => panic!("Error"),
        }
    }
    score
}

pub fn part2(input: &str) -> i32 {
    let mut score = 0;
    for round in input.split('\n') {
        let mut player_choices = round.split(' ');
        let opponent_choice =
            player_choices.next().unwrap().chars().next().unwrap() as i32 - 'A' as i32;
        let target = player_choices.next().unwrap().chars().next().unwrap() as i32 - 'X' as i32;
        score += target * 3;
        let mut my_choice = 1;
        match target {
            0 => my_choice += (opponent_choice + 2) % 3,
            1 => my_choice += opponent_choice,
            2 => my_choice += (opponent_choice + 1) % 3,
            _ => panic!("Error"),
        }
        score += my_choice;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = fs::read_to_string("input/2022/day_02_test_01.txt").expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 15);
    }

    #[test]
    fn part2_test1() {
        let input = fs::read_to_string("input/2022/day_02_test_01.txt").expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 12);
    }
}
