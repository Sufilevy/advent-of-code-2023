mod reflector_dish;

use std::fs;

use reflector_dish::{Dish, HorizontalDirection, VerticalDirection};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut dish = Dish::from(input);
    dish.tilt_vertically(VerticalDirection::North);
    dish.north_load()
}

fn puzzle_two(input: &[&str]) -> u32 {
    let mut dish = Dish::from(input);
    let mut states = Vec::<Dish>::new();
    for cycle in 0..1_000_000_000 {
        dish.tilt_vertically(VerticalDirection::North);
        dish.tilt_horizontally(HorizontalDirection::West);
        dish.tilt_vertically(VerticalDirection::South);
        dish.tilt_horizontally(HorizontalDirection::East);
        let current_state = dish.clone();
        for (index, state) in states.iter().enumerate() {
            if current_state == *state {
                let loop_size = cycle - index;
                let loop_offset = (1_000_000_000 - cycle - 1) % loop_size;
                return states[index + loop_offset].north_load();
            }
        }
        states.push(current_state);
    }
    dish.north_load()
}
