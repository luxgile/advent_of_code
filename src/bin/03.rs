use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(3);

struct Part {
    value: u32,
    start: i32,
    end: i32,
}
struct Symbol {
    value: char,
    idx: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count() as i32 + 1;
    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    let rpart = Regex::new(r"[0-9]+").unwrap();
    let rsym = Regex::new(r"[^0-9a-zA-Z.\n ]").unwrap();
    for item in rpart.find_iter(input) {
        parts.push(Part {
            value: u32::from_str(item.as_str()).unwrap(),
            start: item.start() as i32,
            end: item.end() as i32 - 1,
        })
    }
    for item in rsym.find_iter(input) {
        symbols.push(Symbol {
            value: item.as_str().chars().next().unwrap(),
            idx: item.start() as i32,
        });
    }
    Some(find_parts(width, &parts, &symbols))
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().chars().count() as i32 + 1;
    let mut parts = Vec::new();
    let mut symbols = Vec::new();
    let rpart = Regex::new(r"[0-9]+").unwrap();
    let rsym = Regex::new(r"[*]").unwrap();
    for item in rpart.find_iter(input) {
        parts.push(Part {
            value: u32::from_str(item.as_str()).unwrap(),
            start: item.start() as i32,
            end: item.end() as i32 - 1,
        })
    }
    for item in rsym.find_iter(input) {
        symbols.push(Symbol {
            value: item.as_str().chars().next().unwrap(),
            idx: item.start() as i32,
        });
    }
    Some(find_pairs(width, &parts, &symbols))
}

fn find_parts(width: i32, parts: &Vec<Part>, symbols: &Vec<Symbol>) -> u32 {
    let mut sum = 0;
    for part in parts {
        if is_adyacent_part(width, part, symbols) {
            sum += part.value;
        }
    }
    sum
}

fn is_adyacent_part(width: i32, part: &Part, symbols: &Vec<Symbol>) -> bool {
    for symbol in symbols {
        let sym = symbol.idx;
        let top = (part.start - width - 1, part.end - width + 1);
        let bottom = (part.start + width - 1, part.end + width + 1);
        if sym == part.start - 1
            || sym == part.end + 1
            || (sym >= top.0 && sym <= top.1)
            || (sym >= bottom.0 && sym <= bottom.1)
        {
            return true;
        }
    }
    false
}

fn find_pairs(width: i32, parts: &Vec<Part>, symbols: &Vec<Symbol>) -> u32 {
    let mut sum = 0;
    for symbol in symbols {
        let idx = symbol.idx;
        let mut parts_close = Vec::new();
        for part in parts {
            let top = (part.start - width - 1, part.end - width + 1);
            let bottom = (part.start + width - 1, part.end + width + 1);
            if idx == part.start - 1
                || idx == part.end + 1
                || (idx >= top.0 && idx <= top.1)
                || (idx >= bottom.0 && idx <= bottom.1)
            {
                parts_close.push(part);
                if parts_close.len() > 2 {
                    break;
                }
            }
        }
        if parts_close.len() == 2 {
            sum += parts_close[0].value * parts_close[1].value;
        }
    }
    sum
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
