mod aoc2023;
mod utils;
use crate::aoc2023::parse_calibration::calc_calibration_1;
use crate::aoc2023::parse_calibration::calc_calibration_2;

fn main() {
    println!("result 1 is: {}", calc_calibration_1().ok().unwrap_or_default());
    println!("result 2 is: {}", calc_calibration_2().ok().unwrap_or_default());
}
