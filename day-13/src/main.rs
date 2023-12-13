mod reflections;

use std::fs;

use reflections::Pattern;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    input
        .split("\r\n\r\n")
        .map(|pattern| Pattern::from(pattern).find_reflection(false).value())
        .sum()
}

fn puzzle_two(input: &str) -> u32 {
    input
        .split("\r\n\r\n")
        .map(|pattern| Pattern::from(pattern).find_reflection(true).value())
        .sum()
}
