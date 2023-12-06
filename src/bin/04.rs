use regex::Regex;
use std::{cell::RefCell, collections::HashMap, str::FromStr};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let r_nmb = Regex::new("[0-9|]+").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let mut iter = r_nmb.find_iter(line);
        iter.next();
        let mut winning_nums = Vec::new();
        let mut my_nums = Vec::new();
        let mut first_part = true;
        for i in iter {
            if (i.as_str() == "|") {
                first_part = false;
                continue;
            }

            if first_part {
                winning_nums.push(i32::from_str(i.as_str()).unwrap());
            } else {
                my_nums.push(i32::from_str(i.as_str()).unwrap());
            }
        }
        let mut price = 0;
        for num in my_nums {
            if winning_nums.contains(&num) {
                if price == 0 {
                    price = 1;
                } else {
                    price *= 2;
                }
            }
        }
        result += price;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let r_nmb = Regex::new("[0-9|]+").unwrap();
    let mut result = 0;
    let mut bonus_points = (0..input.lines().count()).map(|_| 1).collect::<Vec<_>>();
    for line in input.lines() {
        let bonus_point = bonus_points.pop().unwrap();
        result += bonus_point;
        let mut iter = r_nmb.find_iter(line);
        iter.next();
        let mut winning_nums = Vec::new();
        let mut my_nums = Vec::new();
        let mut first_part = true;
        for i in iter {
            if (i.as_str() == "|") {
                first_part = false;
                continue;
            }

            if first_part {
                winning_nums.push(i32::from_str(i.as_str()).unwrap());
            } else {
                my_nums.push(i32::from_str(i.as_str()).unwrap());
            }
        }
        let mut matches = 0;
        for num in my_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }
        let mut iter = bonus_points.iter_mut().rev();
        for m in 0..matches {
            *iter.next().unwrap() += bonus_point;
        }
    }
    Some(result)
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
