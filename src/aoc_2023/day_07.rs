use std::collections::HashMap;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 7;

#[derive(Eq, Debug, PartialOrd, Ord, PartialEq)]
struct Card {
    label: char,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
enum CardType {
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
    card_type: CardType,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        let card_type;

        let mut cards_map: HashMap<char, usize> = HashMap::new();

        for card in &cards {
            match cards_map.get(&card.label) {
                Some(count) => cards_map.insert(card.label, count + 1),
                None => cards_map.insert(card.label, 1),
            };
        }

        let mut val: Vec<&usize> = cards_map.values().collect();
        val.sort();
        val.reverse();
        card_type = match val.first() {
            Some(5) => CardType::FiveOfAKind,
            Some(4) => CardType::FourOfAKind,
            Some(3) => {
                if *val[1] == 2 {
                    CardType::FullHouse
                } else {
                    CardType::ThreeOfAKind
                }
            }
            Some(2) => {
                if *val[1] == 2 {
                    CardType::TwoPair
                } else {
                    CardType::OnePair
                }
            }
            Some(1) => CardType::HighCard,
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
                    match a.cmp(&b) {
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

fn parse(input: &str) -> Vec<Hand> {
    let mut hands = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut s = line.split(' ');
        let hand_str = s.next().unwrap();
        let mut cards = Vec::with_capacity(5);
        for c in hand_str.chars() {
            cards.push(Card { label: c })
        }
        let bid = s.next().unwrap().parse().unwrap();
        hands.push(Hand::new(cards, bid))
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
    let mut hands = parse(input);
    println!("{:?}", hands);
    hands.sort();
    hands.reverse();
    println!("#################");
    println!("{:?}", hands);
    hands.iter().enumerate().map(|(i, h)| h.bid * i).sum()
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
        assert_eq!(t, 6440);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 71503);
    }
}
