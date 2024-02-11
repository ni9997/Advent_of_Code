use std::collections::HashMap;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 7;

#[derive(Eq, Debug, PartialOrd, Ord, PartialEq, Hash, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    As,
}

impl Card {
    fn from_char(c: char, joker_mode: bool) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => {
                if joker_mode {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::As,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Card::Joker => 'I',
                Card::Two => '2',
                Card::Three => '3',
                Card::Four => '4',
                Card::Five => '5',
                Card::Six => '6',
                Card::Seven => '7',
                Card::Eight => '8',
                Card::Nine => '9',
                Card::Ten => 'T',
                Card::Jack => 'J',
                Card::Queen => 'Q',
                Card::King => 'K',
                Card::As => 'A',
            }
        )?;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    card_type: HandType,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize, joker_mode: bool) -> Self {
        let mut cards_map: HashMap<Card, usize> = HashMap::new();

        for card in &cards {
            match cards_map.get(card) {
                Some(count) => cards_map.insert(*card, count + 1),
                None => cards_map.insert(*card, 1),
            };
        }
        let joker_count = match cards_map.get(&Card::Joker) {
            Some(j) => {
                if joker_mode {
                    *j
                } else {
                    0
                }
            }
            None => 0,
        };
        cards_map.remove(&Card::Joker);
        let mut val: Vec<&usize> = cards_map.values().collect();
        val.sort();
        val.reverse();
        let card_type = match val.first() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => {
                if joker_count == 1 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            Some(3) => {
                if joker_count == 2 {
                    HandType::FiveOfAKind
                } else if joker_count == 1 {
                    HandType::FourOfAKind
                } else if *val[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            Some(2) => {
                if joker_count == 3 {
                    HandType::FiveOfAKind
                } else if joker_count == 2 {
                    HandType::FourOfAKind
                } else if joker_count == 1 {
                    if *val[1] == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if *val[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            Some(1) => {
                if joker_count == 4 {
                    HandType::FiveOfAKind
                } else if joker_count == 3 {
                    HandType::FourOfAKind
                } else if joker_count == 2 {
                    HandType::ThreeOfAKind
                } else if joker_count == 1 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            None => HandType::FiveOfAKind,
            _ => panic!(),
        };

        Self {
            cards,
            bid,
            card_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.card_type.cmp(&other.card_type) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    match a.cmp(b) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {
                            continue;
                        }
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                panic!()
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hand: ")?;
        for c in &self.cards {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

fn parse(input: &str, joker_mode: bool) -> Vec<Hand> {
    let mut hands = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut s = line.split(' ');
        let hand_str = s.next().unwrap();
        let mut cards = Vec::with_capacity(5);
        for c in hand_str.chars() {
            cards.push(Card::from_char(c, joker_mode))
        }
        let bid = s.next().unwrap().parse().unwrap();
        hands.push(Hand::new(cards, bid, joker_mode))
    }
    hands
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let mut hands = parse(input, false);
    // for h in &hands {
    //     println!("{}", h);
    // }
    hands.sort();
    // println!("#################");
    // for h in &hands {
    //     println!("{}", h);
    // }
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
}

pub fn part2(input: &str) -> usize {
    let mut hands = parse(input, true);
    // for h in &hands {
    //     println!("{}", h);
    // }
    hands.sort();
    // println!("#################");
    // for h in &hands {
    //     println!("{}", h);
    // }
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
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
        assert_eq!(t, 6440);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 5905);
    }
}
