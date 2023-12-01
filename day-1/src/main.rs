use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

const DIGITS_ASCII_OFFSET: u8 = 48;

fn puzzle_one(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|&line| {
            let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let second = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            (line.as_bytes()[first] - DIGITS_ASCII_OFFSET) as u32 * 10
                + (line.as_bytes()[second] - DIGITS_ASCII_OFFSET) as u32
        })
        .sum()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let digits_words = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    input
        .iter()
        .map(|&line| {
            let line = digits_words
                .iter()
                .fold(line.to_owned(), |line, (from, to)| line.replace(from, to));

            let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let second = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            (line.as_bytes()[first] - DIGITS_ASCII_OFFSET) as u32 * 10
                + (line.as_bytes()[second] - DIGITS_ASCII_OFFSET) as u32
        })
        .sum()
}
