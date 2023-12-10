mod pipes;

use std::fs;

use pipes::Pipes;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let pipes = Pipes::from(input);
    (pipes.pipes_loop().len() as u32 + 1) / 2
}

fn puzzle_two(input: &[&str]) -> u32 {
    let pipes = Pipes::from(input);
    pipes.count_enclosed_tiles(input)
}
