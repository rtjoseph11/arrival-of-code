mod aoc2022;
mod aoc2023;
mod utils;
use crate::aoc2022::max_calories::calc_calories_1;
use crate::aoc2022::max_calories::calc_calories_2;

fn main() {
    println!("result 1 is: {}", calc_calories_1());
    println!("result 2 is: {}", calc_calories_2());
}
