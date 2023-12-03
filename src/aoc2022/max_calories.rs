use std::cmp;
use crate::utils::file::read_lines;

pub fn calc_calories_1() -> i32 {

    let mut max_calories = 0;
    if let Ok(lines) = read_lines("./src/aoc2022/2022_advent_of_code_1_1.txt") {
        let mut cur_calories = 0;
        for line in lines {
            if let Ok(raw) = line {
                if raw.as_str() == "" {
                    max_calories = cmp::max(cur_calories, max_calories);
                    cur_calories = 0;
                } else {
                    let num = raw.parse::<i32>().unwrap();
                    cur_calories += num;
                }
            } else {
                return 0;
            }
        }
    }
    return max_calories;
}

pub fn calc_calories_2() -> i32 {
    let mut max_calories = Vec::new();
    max_calories.push(0);
    max_calories.push(0);
    max_calories.push(0);
    if let Ok(lines) = read_lines("./src/aoc2022/2022_advent_of_code_1_1.txt") {
        let mut cur_calories = 0;
        for line in lines {
            if let Ok(raw) = line {
                if raw.as_str() == "" {
                    if cur_calories > max_calories[0] {
                        max_calories[0] = cur_calories;
                        max_calories.sort();
                    }
                    cur_calories = 0;
                } else {
                    let num = raw.parse::<i32>().unwrap();
                    cur_calories += num;
                }
            } else {
                return 0;
            }
        }
    }
    return max_calories.iter().sum();
}
