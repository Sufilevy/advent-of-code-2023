mod digging;

use std::fs;

use digging::Lagoon;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut lagoon = Lagoon::from(input);
    lagoon.dig();
    lagoon.capacity()
}

fn puzzle_two(_input: &[&str]) -> u32 {
    0
}
