use crate::utils::file::read_lines;
use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct GameResult {
    red: i32,
    blue: i32,
    green: i32
}


#[derive(Debug)]
struct Game {
    id: i32,
    results: Vec<GameResult>
}

fn get_games() -> Vec<Game> {
    let mut result = Vec::new();
    let lines = match read_lines("./src/aoc2023/data/2023_advent_of_code_2.txt") {
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
        let game = parse_line(&raw);
        result.push(game);
    }
    return result;
}

pub fn part_one() -> i32 {
    let games = get_games();
    let mut result = 0;
    for game in games {
        result += match game.results.iter().find(|gr| gr.red > 12 || gr.blue > 14 || gr.green > 13) {
            Some(_) => 0,
            None => game.id
        }
    }
    return result;
}

pub fn part_two() -> i32 {
    let games = get_games();
    let mut result = 0;
    for game in games {
        let min_cubes = game.results.iter().fold(GameResult{red: 0, green: 0, blue: 0}, |acc, gr| {
            GameResult {
                red: max(acc.red, gr.red),
                blue: max(acc.blue, gr.blue),
                green: max(acc.green, gr.green) 
            }
        });
        result += min_cubes.red * min_cubes.blue * min_cubes.green;
    }
    return result;
}

fn parse_line(raw: &str) -> Game {
    let mut result = Game {
        id: 0,
        results: Vec::new()
    };
    let game_id_matcher = Regex::new(r"Game (\d+): (.*)").unwrap();
    let Some(caps) = game_id_matcher.captures(raw) else {
        println!("no game id capture");
        return result;
    };
    result.id = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
    let Some(results) = caps.get(2).map(|m| m.as_str().split(";").into_iter()) else {
        println!("failed to get games");
        return result;
    };
    let green_matcher = Regex::new(r"(\d+) green").unwrap();
    let red_matcher = Regex::new(r"(\d+) red").unwrap();
    let blue_matcher = Regex::new(r"(\d+) blue").unwrap();
    result.results = results.map(|gr| {
        let green = green_matcher.captures(gr).map_or(0, |h| h.get(1).map_or(0, |m| m.as_str().parse().unwrap()));
        let red = red_matcher.captures(gr).map_or(0, |h| h.get(1).map_or(0, |m| m.as_str().parse().unwrap()));
        let blue = blue_matcher.captures(gr).map_or(0, |h| h.get(1).map_or(0, |m| m.as_str().parse().unwrap()));
        GameResult {
            green: green,
            red: red,
            blue: blue
        }
    }).collect();
    return result;
    
}

