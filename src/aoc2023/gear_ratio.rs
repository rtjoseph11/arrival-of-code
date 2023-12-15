use crate::utils::file::read_lines;
use either::*;
use std::collections::HashSet;

type Schematic = Vec<Vec<Option<Either<i32, char>>>>;

fn get_schematic() -> Schematic {
    let mut result = Vec::new();
    let lines = match read_lines("./src/aoc2023/data/2023_advent_of_code_3.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("failed to read file: {}", e);
            return result;
        },
    };
    for line in lines {
        let raw = match line {
            Ok(r) => r,
            Err(e) => {
                println!("failed to read line: {}", e);
                return result;
            },
        };
        let mut current = Vec::new();
        for c in raw.as_str().chars() {
            match c {
                '.' => current.push(None),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => current.push(Some(Left(c.to_digit(10).map(|i| i as i32).unwrap()))),
                _ => current.push(Some(Right(c)))
            }
        }
        result.push(current);
    }
    return result;
}

pub fn part_one() -> i32 {
    let schematic = get_schematic();
    let mut seen = HashSet::new();
    let mut result = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Some(Right(_)) => {
                    result += find_numbers(&schematic, &mut seen, i, j).iter().fold(0, |acc, curr| acc + curr);
                },
                _ => (),

            }
        }
    }
    return result;
}

pub fn part_two() -> i32 {
    let schematic = get_schematic();
    let mut seen = HashSet::new();
    let mut result = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Some(Right('*')) => {
                    let numbers = find_numbers(&schematic, &mut seen, i, j);
                    if numbers.len() == 2 {
                        result += numbers.iter().fold(1, |acc, curr| acc * curr);
                    }
                },
                _ => (),

            }
        }
    }
    return result;
}

fn find_numbers(schematic: &Schematic, seen: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> Vec<i32> {
    let needles = [(i - 1, j - 1,), (i - 1, j), (i - 1, j + 1), (i, j - 1), (i, j + 1), (i + 1, j - 1,), (i + 1, j), (i + 1, j + 1)];
    let mut result = Vec::new();

    for (test_i, test_j) in needles {
        let mut curr_j = test_j;
        let mut building = Vec::new();
        while valid(schematic, test_i, curr_j).is_some() && !seen.contains(&(test_i, curr_j)) {
            let mut head = vec![valid(schematic, test_i, curr_j).unwrap()];
            head.append(&mut building);
            building = head;
            seen.insert((test_i, curr_j));
            if curr_j > 0 {
                curr_j -= 1
            } else {
                break; // handle negative usize
            }
            
        }

        if building.len() == 0 {
            continue; // don't search right when nothing was found left or equal
        }

        curr_j = test_j + 1;
        while valid(schematic, test_i, curr_j).is_some() && !seen.contains(&(test_i, curr_j)) {
            building.push(valid(schematic, test_i, curr_j).unwrap());
            seen.insert((test_i, curr_j));
            curr_j += 1
        }
        
        let next = building.iter().rev().enumerate().fold(0, |acc, (x, y)| {
            return acc + (y * 10_i32.pow(x.try_into().unwrap()));
        });
        result.push(next);
    }
    return result;
}

fn valid(schematic: &Schematic, i: usize, j: usize) -> Option<i32> {
    return schematic.get(i).map(|row| row.get(j)).flatten().map(|e| {
        match e {
            Some(Left(i)) => Some(*i),
            _ => None
        }
    }).flatten();
}