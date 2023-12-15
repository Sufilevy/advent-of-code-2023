use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &str) -> u32 {
    input.split(',').map(hash_word).sum()
}

fn hash(state: u32, char: char) -> u32 {
    ((state + char as u32) * 17) % 256
}

fn hash_word(word: &str) -> u32 {
    word.chars().fold(0, hash)
}

type LightBox = Vec<(String, u32)>;

fn puzzle_two(input: &str) -> u32 {
    let mut boxes = vec![LightBox::new(); 256];
    for step in input.split(',') {
        run_step(&mut boxes, step);
    }
    focus_power_of(&boxes)
}

fn run_step(boxes: &mut [LightBox], step: &str) {
    if step.ends_with('-') {
        let lens_label = step.trim_end_matches('-');

        let box_index = hash_word(lens_label) as usize;
        let light_box = boxes.get_mut(box_index).unwrap();

        remove_from_box(light_box, lens_label);
    } else {
        let mut parts = step.split('=');
        let lens_label = parts.next().unwrap();
        let lens_power = parts.next().unwrap().parse::<u32>().unwrap();

        let box_index = hash_word(lens_label) as usize;
        let light_box = boxes.get_mut(box_index).unwrap();

        add_to_box(light_box, lens_label, lens_power);
    }
}

fn remove_from_box(light_box: &mut LightBox, lens: &str) {
    light_box.retain(|(lens_name, _)| lens_name != lens);
}

fn add_to_box(light_box: &mut LightBox, lens: &str, lens_power: u32) {
    if light_box.iter().any(|(lens_name, _)| lens_name == lens) {
        light_box
            .iter_mut()
            .find(|(lens_name, _)| lens_name == lens)
            .unwrap()
            .1 = lens_power;
    } else {
        light_box.push((lens.to_string(), lens_power));
    }
}

fn focus_power_of(boxes: &[LightBox]) -> u32 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, light_box)| {
            light_box
                .iter()
                .enumerate()
                .map(|(lens_index, (_, power))| {
                    power * (box_index as u32 + 1) * (lens_index as u32 + 1)
                })
                .sum::<u32>()
        })
        .sum()
}
