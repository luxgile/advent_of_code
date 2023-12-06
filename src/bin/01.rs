use std::str::FromStr;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.lines() {
        let line_res = get_numbers_from_line(line);
        result += u32::from_str(line_res.as_str()).unwrap()
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.lines() {
        let line = replace_numbers(line);
        let line_res = get_numbers_from_line(line.as_str());
        result += u32::from_str(line_res.as_str()).unwrap()
    }
    Some(result)
}

fn replace_numbers(line: &str) -> String {
    let mut nline = String::from_str(line).unwrap();
    nline = nline.replace("zero", "z0o");
    nline = nline.replace("one", "o1e");
    nline = nline.replace("two", "t2o");
    nline = nline.replace("three", "t3e");
    nline = nline.replace("four", "f4r");
    nline = nline.replace("five", "f5e");
    nline = nline.replace("six", "s6x");
    nline = nline.replace("seven", "s7v");
    nline = nline.replace("eight", "e8t");
    nline = nline.replace("nine", "n9e");
    nline
}

fn get_numbers_from_line(line: &str) -> String {
    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = digits.clone().next().unwrap();
    let last = digits.next_back().unwrap();
    format!("{}{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55607));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55291));
    }
}
