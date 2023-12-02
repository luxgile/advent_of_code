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
        print!("{}", line);
        let line = replace_numbers(line);
        print!(" - {}", line);
        let line_res = get_numbers_from_line(line.as_str());
        print!(" - {}\n", line_res);
        result += u32::from_str(line_res.as_str()).unwrap()
    }
    Some(result)
}

fn replace_numbers(line: &str) -> String {
    let mut nline = String::from_str(line).unwrap();
    nline = nline.replace("zero", "0");
    nline = nline.replace("one", "1");
    nline = nline.replace("two", "2");
    nline = nline.replace("three", "3");
    nline = nline.replace("four", "4");
    nline = nline.replace("five", "5");
    nline = nline.replace("six", "6");
    nline = nline.replace("seven", "7");
    nline = nline.replace("eight", "8");
    nline = nline.replace("nine", "9");
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
