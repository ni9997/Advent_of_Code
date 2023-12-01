use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use crate::utils::get_input;

const YEAR: usize = 2022;
const DAY: usize = 13;

enum PacketData {
    Integer(usize),
    List(Vec<PacketData>),
}

struct Packet {
    data: Vec<PacketData>,
}

impl Packet {
    fn from_str(input: &str) -> Packet {
        todo!()
    }
}

struct PacketPair {
    left: Packet,
    right: Packet,
}

fn compare(left: &Vec<PacketData>, right: &Vec<PacketData>) -> bool {
    todo!()
}

impl PacketPair {
    fn is_right_order(&self) -> bool {
        compare(&self.left.data, &self.right.data)
    }
}

fn parse(input: &str) -> Vec<PacketPair> {
    let pair_str = input.split("\n\n");
    let mut packet_pairs = Vec::with_capacity(pair_str.clone().count());

    for pair in pair_str {
        let mut strs = pair.split('\n');
        let left_pkt = Packet::from_str(strs.nth(0).unwrap());
        let right_pkt = Packet::from_str(strs.nth(0).unwrap());
        packet_pairs.push(PacketPair {
            left: left_pkt,
            right: right_pkt,
        });
    }
    packet_pairs
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let packet_pairs = parse(input);
    packet_pairs.iter().filter(|x| x.is_right_order()).count()
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
        assert_eq!(t, 31);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 29);
    }
}
