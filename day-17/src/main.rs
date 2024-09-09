mod crucibles;

use std::fs;
use crate::crucibles::City;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    City::from(input).get_best_path_cost()
}

fn puzzle_two(_input: &[&str]) -> u32 {
    0
}
