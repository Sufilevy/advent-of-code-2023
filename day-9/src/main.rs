use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|line| {
            let line = line
                .split_ascii_whitespace()
                .flat_map(str::parse::<i32>)
                .collect::<Vec<i32>>();
            extrapolate_next(&line)
        })
        .sum()
}

fn extrapolate_next(line: &[i32]) -> i32 {
    let diffs = diffs_of(line);
    line.last().unwrap()
        + if diffs.iter().all(|&diff| diff == 0) {
            0
        } else {
            extrapolate_next(&diffs)
        }
}

fn diffs_of(line: &[i32]) -> Vec<i32> {
    line.iter()
        .enumerate()
        .skip(1)
        .map(|(index, num)| num - line[index - 1])
        .collect()
}

fn puzzle_two(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|line| {
            let line = line
                .split_ascii_whitespace()
                .flat_map(str::parse::<i32>)
                .collect::<Vec<i32>>();
            extrapolate_previous(&line)
        })
        .sum()
}

fn extrapolate_previous(line: &[i32]) -> i32 {
    let diffs = diffs_of(line);
    line.first().unwrap()
        - if diffs.iter().all(|&diff| diff == 0) {
            0
        } else {
            extrapolate_previous(&diffs)
        }
}
