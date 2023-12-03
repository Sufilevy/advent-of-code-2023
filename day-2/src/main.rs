use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

type Bag = (u32, u32, u32);

fn puzzle_one(input: &[&str]) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let game = line.split(": ").nth(1).unwrap();

            if game.split("; ").all(does_round_match_bag) {
                index as u32 + 1
            } else {
                0
            }
        })
        .sum()
}

const ELF_BAG: Bag = (12, 13, 14);

fn does_round_match_bag(round: &str) -> bool {
    round.split(", ").all(|color| {
        let num_cubes = num_cubes_of_color(color);

        match color.chars().last().unwrap() {
            'd' => num_cubes <= ELF_BAG.0, // Red
            'n' => num_cubes <= ELF_BAG.1, // Green
            _ => num_cubes <= ELF_BAG.2,   // Blue
        }
    })
}

fn num_cubes_of_color(color: &str) -> u32 {
    color.split(' ').next().unwrap().parse::<u32>().unwrap()
}

fn puzzle_two(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let game = line.split(": ").nth(1).unwrap();

            let min_bag =
                game.split("; ")
                    .map(bag_of_round)
                    .fold(Bag::default(), |min_bag, bag| {
                        (
                            min_bag.0.max(bag.0), // Red
                            min_bag.1.max(bag.1), // Green
                            min_bag.2.max(bag.2), // Blue
                        )
                    });

            min_bag.0 * min_bag.1 * min_bag.2
        })
        .sum()
}

fn bag_of_round(round: &str) -> Bag {
    round.split(", ").fold(Bag::default(), |mut bag, color| {
        let num_cubes = num_cubes_of_color(color);

        match color.chars().last().unwrap() {
            'd' => bag.0 = num_cubes, // Red
            'n' => bag.1 = num_cubes, // Green
            _ => bag.2 = num_cubes,   // Blue
        }

        bag
    })
}
