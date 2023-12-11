use crate::utils::file::read_lines;
use std::collections::HashMap;

pub fn calc_calibration_1() -> Result<i32, String> {
    let lines = match read_lines("./src/aoc2023/data/2023_advent_of_code_1.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("failed to read file: {}", e);
            return Err(e.to_string());
        },
    };
    let mut result = 0;
    for line in lines {
        let raw = match line {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        let left = maybe_char_to_int(raw.as_str().chars().find(|c| c.is_numeric())) * 10;
        let right = maybe_char_to_int(raw.as_str().chars().rfind(|c| c.is_numeric()));
        result += left + right;

    }
    return Ok(result);
}

pub fn calc_calibration_2() -> Result<i32, String> {
    let lines = match read_lines("./src/aoc2023/2023_advent_of_code_1.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("failed to read file: {}", e);
            return Err(e.to_string());
        },
    };
    let mut result = 0;

    for line in lines {
        let raw = match line {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };
        let line_num = line_to_int(raw.as_str());
        // println!("line value is: {}", line_num);
        result += line_num;

    }

    return Ok(result);
}

fn line_to_int(line: &str) -> i32 {
    let mut current = String::from("");
    let mut result = 0;
    for c in line.chars() {
        current += c.to_string().as_str();
        if c.is_numeric() {
            result += 10 * maybe_char_to_int(Some(c));
            break;
        } else {
            match str_is_digit(current.as_str(), true) {
                Some(d) => {
                    result += 10 * d;
                    break;
                },
                _ => (),
            }
        }
    }

    current = String::from("");
    for c in line.chars().rev() {
        current = c.to_string() + current.as_str();
        if c.is_numeric() {
            result += maybe_char_to_int(Some(c));
            break;
        } else {
            match str_is_digit(current.as_str(), false) {
                Some(d) => {
                    result += d;
                    break;
                },
                _ => (),
            }
        }   
    }
    return result;
}

fn str_is_digit(raw: &str, is_left: bool) -> Option<i32> {
    let checks = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]);
    return checks.iter().find(|&(k, _)| {
        if is_left {
            return raw.ends_with(k);
        } else {
            return raw.starts_with(k);
        }
    }).map(|(_, v)| *v);
}

fn maybe_char_to_int(maybe_c: Option<char>) -> i32 {
    maybe_c.map(|c| c.to_digit(10).map(|x| x as i32)).flatten().unwrap_or_default()
}