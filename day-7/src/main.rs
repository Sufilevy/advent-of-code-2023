#![allow(unused)]

mod camel_cards;
mod camel_cards2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut hands: Vec<camel_cards::Hand> = input
        .iter()
        .map(|line| camel_cards::Hand::from_line(line))
        .collect();
    hands.sort_by(|a, b| a.cmp(b));
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u32 + 1))
        .sum()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let mut hands: Vec<camel_cards2::Hand> = input
        .iter()
        .map(|line| camel_cards2::Hand::from_line(line))
        .collect();
    hands.sort_by(|a, b| a.cmp(b));
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u32 + 1))
        .sum()
}
