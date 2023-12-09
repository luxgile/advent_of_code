use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug)]
struct MapRange {
    start: i64,
    dest: i64,
    len: i64,
}
impl MapRange {
    pub fn map_value(&self, value: i64) -> Option<i64> {
        if value >= self.start && value < self.start + self.len {
            return Some(self.dest + (value - self.start));
        }
        None
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}
impl Map {
    pub fn map_value(&self, value: i64) -> i64 {
        for range in self.ranges.iter() {
            if let Some(val) = range.map_value(value) {
                return val;
            }
        }
        value
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = i64::MAX;
    let seeds = get_seeds(input);
    let maps = get_maps(input);
    for seed in seeds {
        let mut value = seed;
        for map in maps.iter() {
            value = map.map_value(value);
        }
        if value < result {
            result = value;
        }
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = i64::MAX;
    let seeds = get_ranged_seeds(input);
    let maps = get_maps(input);
    for seed in seeds {
        let mut value = seed;
        for map in maps.iter() {
            value = map.map_value(value);
        }
        if value < result {
            result = value;
        }
    }
    Some(result as u32)
}

fn get_seeds(input: &str) -> Vec<i64> {
    let mut seeds = Vec::new();
    let r_number = Regex::new("[0-9]+").unwrap();
    let first_line = input.lines().next().unwrap();
    for number in r_number.find_iter(first_line) {
        let nmb = i64::from_str(number.as_str()).unwrap();
        seeds.push(nmb);
    }
    seeds
}

fn get_ranged_seeds(input: &str) -> Vec<i64> {
    let mut seeds = Vec::new();
    let r_number = Regex::new("[0-9]+").unwrap();
    let first_line = input.lines().next().unwrap();
    let mut iter = r_number.find_iter(first_line).collect::<Vec<_>>();
    for number in iter.chunks(2) {
        let nmb = i64::from_str(number[0].as_str()).unwrap();
        let rng = i64::from_str(number[1].as_str()).unwrap();
        seeds.extend((nmb..nmb + rng).collect::<Vec<_>>());
    }
    seeds
}

fn get_maps(input: &str) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut active_map = None;
    for line in input.lines() {
        let substrings = line.split(" ").collect::<Vec<_>>();
        if substrings.len() == 2 && substrings[1] == "map:" {
            if let Some(map) = active_map {
                maps.push(map);
            }
            active_map = Some(Map {
                name: String::from_str(substrings[0]).unwrap(),
                ranges: Vec::new(),
            });
            continue;
        }

        if substrings.len() != 3 {
            continue;
        }

        let map = active_map.as_mut().unwrap();
        let range = MapRange {
            dest: i64::from_str(substrings[0]).unwrap(),
            start: i64::from_str(substrings[1]).unwrap(),
            len: i64::from_str(substrings[2]).unwrap(),
        };
        map.ranges.push(range);
    }
    maps.push(active_map.unwrap());
    maps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
