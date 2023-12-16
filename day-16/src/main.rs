mod mirrors;

use std::fs;

use mirrors::{Direction, Mirrors};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut mirrors = Mirrors::from(input);
    mirrors.advance_beam((0, 0), Direction::Right);
    mirrors.count_energized_tiles()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let mirrors = Mirrors::from(input);
    let (rows, cols) = (mirrors.rows(), mirrors.cols());

    let rows_max = [(0, Direction::Right), (cols - 1, Direction::Left)]
        .iter()
        .map(|&(col, dir)| {
            (0..rows)
                .map(|row| {
                    let mut mirrors = mirrors.clone();
                    mirrors.advance_beam((row, col), dir);
                    mirrors.count_energized_tiles()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    let cols_max = [(0, Direction::Down), (rows - 1, Direction::Up)]
        .iter()
        .map(|&(row, dir)| {
            (0..cols)
                .map(|col| {
                    let mut mirrors = mirrors.clone();
                    mirrors.advance_beam((row, col), dir);
                    mirrors.count_energized_tiles()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    rows_max.max(cols_max)
}
