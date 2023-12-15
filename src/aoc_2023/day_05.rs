use std::str::FromStr;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 5;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

#[derive(Debug)]
struct Map {
    source: usize,
    destination: usize,
    range: usize,
}

impl Map {
    fn map(&self, input: usize) -> Option<usize> {
        if self.source >= input && input < self.source+self.range {
            return Some(input-self.source+self.destination);
        }
        None
    }
}

#[derive(Debug)]
struct Garden {
    seeds: Vec<usize>,
    seed_to_soil_map: Map,
    soil_to_fetilizer_map: Map,
    fetilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

impl Garden {
    fn get_locations(&self) -> Vec<usize> {
        todo!()
    }
}

#[derive(Debug)]
struct GardenParseError;

impl FromStr for Garden {
    type Err = GardenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split("\n\n");
    let seeds: Vec<usize> = input
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut s2smap = vec![];
    for x in input.next().unwrap().split('\n').skip(1) {
        let mut temp = x.split(' ');
        let destination = temp.next().unwrap().parse().unwrap();
        let source = temp.next().unwrap().parse().unwrap();
        let range = temp.next().unwrap().parse().unwrap();
        let map = Map {
            source,
            destination,
            range,
        };
        s2smap.push(map);
    }
    println!("{:?}", seeds);
    todo!()
    }
}

fn seed_to_soil(seed: usize) -> usize {
    todo!()
}

fn soil_to_fetilizer(soil: usize) -> usize {
    todo!()
}

fn fetilizer_to_water(fertilizer: usize) -> usize {
    todo!()
}

fn water_to_light(water: usize) -> usize {
    todo!()
}

fn light_to_temperature(light: usize) -> usize {
    todo!()
}

fn temperature_to_humidity(temperature: usize) -> usize {
    todo!()
}

fn humidity_to_location(humidity: usize) -> usize {
    todo!()
}

pub fn part1(input: &str) -> usize {
    let garden = input.parse::<Garden>().unwrap();
    *garden.get_locations().iter().min().unwrap()
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
        assert_eq!(t, 35);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/{}/day_{:02}_test_01.txt", YEAR, DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 30);
    }
}
