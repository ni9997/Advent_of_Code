use itertools::Itertools;

use crate::utils::get_input;

const YEAR: usize = 2023;
const DAY: usize = 5;

enum Part {
    One,
    Two,
}

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let input = get_input(YEAR, DAY).unwrap();
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

struct MapIntoIter {
    map: Map,
    map_index: usize,
    location_index: usize,
}

impl Iterator for MapIntoIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Map Index: {}\nLocation Index: {}", self.map_index, self.location_index);
        if self.location_index < self.map.maps[self.map_index].2 {
            self.location_index += 1;
            Some(self.map.maps[self.map_index].1 + self.location_index - 1)
        } else {
            self.map_index += 1;
            self.location_index = 0;
            if self.map_index < self.map.maps.len() {
                Some(self.map.maps[self.map_index].1 + self.location_index)
            } else {
                None
            }
        }
        // result
    }
}

#[derive(Debug, Clone)]
struct Map {
    maps: Vec<(usize, usize, usize)>,
}

impl IntoIterator for Map {
    type Item = usize;

    type IntoIter = MapIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            map: self,
            map_index: 0,
            location_index: 0,
        }
    }
}

impl Map {
    fn map(&self, input: usize) -> usize {
        for (source, destination, range) in &self.maps {
            if input >= *source && input < source + range {
                return input - source + destination;
            }
        }
        input
    }

    fn from_lines(input: &str) -> Map {
        let mut maps = vec![];
        for x in input.split('\n').skip(1) {
            let mut temp = x.split(' ');
            let destination = temp.next().unwrap().parse().unwrap();
            let source = temp.next().unwrap().parse().unwrap();
            let range = temp.next().unwrap().parse().unwrap();
            maps.push((source, destination, range));
        }
        // println!("{:?}", maps);
        Map { maps }
    }
}

#[derive(Debug, Clone)]
struct Garden {
    seeds: Vec<usize>,
    seed_maps: Option<Map>,
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
        self.seeds
            .iter()
            .map(|x| self.seed_to_soil_map.map(*x))
            .map(|x| self.soil_to_fetilizer_map.map(x))
            .map(|x| self.fetilizer_to_water_map.map(x))
            .map(|x| self.water_to_light_map.map(x))
            .map(|x| self.light_to_temperature_map.map(x))
            .map(|x| self.temperature_to_humidity_map.map(x))
            .map(|x| self.humidity_to_location_map.map(x))
            .collect()
    }

    fn get_location(&self, seed: usize) -> usize {
        self.humidity_to_location_map.map(
            self.temperature_to_humidity_map.map(
                self.light_to_temperature_map.map(
                    self.water_to_light_map.map(
                        self.fetilizer_to_water_map.map(
                            self.soil_to_fetilizer_map
                                .map(self.seed_to_soil_map.map(seed)),
                        ),
                    ),
                ),
            ),
        )
    }

    fn from_str(input: &str, part: Part) -> Result<Garden, GardenParseError> {
        let mut input = input.split("\n\n");
        let mut seed_map = None;
        let seeds: Vec<usize> = match part {
            Part::One => input
                .next()
                .unwrap()
                .replace("seeds: ", "")
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect(),
            Part::Two => {
                seed_map = Some(Map {
                    maps: input
                        .next()
                        .unwrap()
                        .replace("seeds: ", "")
                        .split(' ')
                        .chunks(2)
                        .into_iter()
                        .map(|mut a| {
                            let start = a.next().unwrap().parse().unwrap();
                            let range = a.next().unwrap().parse().unwrap();
                            (start, start, range)
                        })
                        .collect(),
                });
                vec![]
            }
        };
        // println!("LEN Seeds: {}", seeds.len());
        // println!("{:?}", seed_map);
        let seed_to_soil_map = Map::from_lines(input.next().unwrap());
        let soil_to_fetilizer_map = Map::from_lines(input.next().unwrap());
        let fetilizer_to_water_map = Map::from_lines(input.next().unwrap());
        let water_to_light_map = Map::from_lines(input.next().unwrap());
        let light_to_temperature_map = Map::from_lines(input.next().unwrap());
        let temperature_to_humidity_map = Map::from_lines(input.next().unwrap());
        let humidity_to_location_map = Map::from_lines(input.next().unwrap());
        Ok(Garden {
            seed_to_soil_map,
            soil_to_fetilizer_map,
            fetilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
            seeds,
            seed_maps: seed_map,
        })
    }
}

#[derive(Debug)]
struct GardenParseError;

pub fn part1(input: &str) -> usize {
    let garden = Garden::from_str(input, Part::One).unwrap();
    *garden.get_locations().iter().min().unwrap()
}

pub fn part2(input: &str) -> usize {
    let garden = Garden::from_str(input, Part::Two).unwrap();
    let mut minimum = usize::MAX;
    for seed in garden.clone().seed_maps.unwrap() {
        let location = garden.get_location(seed);
        if location < minimum {
            minimum = location;
        }
    }
    minimum
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
        assert_eq!(t, 46);
    }
}
