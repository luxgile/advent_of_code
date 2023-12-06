advent_of_code::solution!(2);

use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut game_id = 1;
    for line in input.lines() {
        let line = String::from_str(line).unwrap();
        let (r, g, b) = get_cubes(&line);
        if (r <= 12 && g <= 13 && b <= 14) {
            result += game_id;
        }
        game_id += 1;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut power = 0;
    for line in input.lines() {
        let line = String::from_str(line).unwrap();
        let (r, g, b) = get_cubes(&line);
        power += r * g * b;
    }
    Some(power)
}

pub fn get_cubes(line: &String) -> (u32, u32, u32) {
    let r = get_single_cube(line, "red");
    let g = get_single_cube(line, "green");
    let b = get_single_cube(line, "blue");
    (r, g, b)
}

pub fn get_single_cube(line: &String, cube: &'static str) -> u32 {
    let idxs = line.as_str().match_indices(cube).map(|c| c.0);
    let mut max = 0;
    for id in idxs {
        let mut chars = line.chars();
        let c = format!("{}{}", chars.nth(id - 3).unwrap(), chars.next().unwrap());
        let n = u32::from_str(c.trim()).unwrap();
        if n > max {
            max = n;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2679));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(77607));
    }
}
