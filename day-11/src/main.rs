mod galaxies;

use std::fs;

use galaxies::Galaxy;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u64 {
    let galaxies = Galaxy::from(input);
    galaxies.shortest_paths_lengths(2).iter().sum()
}

fn puzzle_two(input: &[&str]) -> u64 {
    let galaxies = Galaxy::from(input);
    galaxies.shortest_paths_lengths(1_000_000).iter().sum()
}
