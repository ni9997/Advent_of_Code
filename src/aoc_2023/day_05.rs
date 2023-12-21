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

#[derive(Debug)]
struct Map {
    maps: Vec<(usize, usize, usize)>,
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

    fn from_str(input: &str, part: Part) -> Result<Garden, GardenParseError> {
        let mut input = input.split("\n\n");
        let seeds: Vec<usize> = match part {
            Part::One => input
                .next()
                .unwrap()
                .replace("seeds: ", "")
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect(),
            Part::Two => {
                let mut seeds = vec![];
                let seed_line = input.next().unwrap().replace("seeds: ", "");
                let mut split = seed_line.split(' ');
                while let Some(begin) = split.next() {
                    let start: usize = begin.parse().unwrap();
                    let lenght: usize = split.next().unwrap().parse().unwrap();
                    for i in 0..lenght {
                        seeds.push(start + i);
                    }
                }

                seeds
            }
        };
        println!("LEN Seeds: {}", seeds.len());
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
    *garden.get_locations().iter().min().unwrap()
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
