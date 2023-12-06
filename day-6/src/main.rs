use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    split_to_numbers(input[0])
        .zip(split_to_numbers(input[1]))
        .map(options_for_race)
        .reduce(|acc, n| acc * n)
        .unwrap()
}

fn split_to_numbers(input: &str) -> impl Iterator<Item = f64> + '_ {
    input.split_whitespace().skip(1).flat_map(str::parse::<f64>)
}

fn options_for_race(race: (f64, f64)) -> u32 {
    let a = 1.0;
    let b = -race.0;
    let c = race.1;
    let d = b.powi(2) - 4.0 * a * c;
    let solution1 = (-b - d.sqrt()) / (2.0 * a);
    let solution2 = (-b + d.sqrt()) / (2.0 * a);
    (solution2.floor() - solution1.ceil()) as u32 + 1
}

fn puzzle_two(input: &[&str]) -> u32 {
    let race = (combine_to_number(input[0]), combine_to_number(input[1]));
    options_for_race(race) as u32
}

fn combine_to_number(input: &str) -> f64 {
    input
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}
