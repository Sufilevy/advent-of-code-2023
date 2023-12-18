mod crucibles;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(_input: &[&str]) -> u32 {
    0
}

fn puzzle_two(_input: &[&str]) -> u32 {
    0
}
