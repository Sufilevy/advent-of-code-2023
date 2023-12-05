mod puzzle_one;
mod puzzle_two;

use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u64 {
    let almanac = puzzle_one::Almanac::from(input);
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.convert_seed(seed))
        .min()
        .unwrap()
}

fn puzzle_two(input: &str) -> u64 {
    let almanac = puzzle_two::Almanac::from(input);
    almanac
        .seeds
        .iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .into_par_iter()
                .map(|seed| almanac.convert_seed(seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
