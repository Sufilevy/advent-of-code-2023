mod network;

use std::fs;

use network::Network;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    let network = Network::from(input);
    network.traverse()
}

fn puzzle_two(input: &str) -> u64 {
    let network = Network::from(input);
    network.traverse2()
}
