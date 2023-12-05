use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    let mut sum = 0;

    for (line_index, line) in input.iter().enumerate() {
        let mut current_num_str = String::new();

        for (index, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                current_num_str.push(char);
                continue;
            }

            if !current_num_str.is_empty() {
                if any_neighbors(is_symbol, input, line_index, index, current_num_str.len()) {
                    sum += current_num_str.parse::<u32>().unwrap();
                }
                current_num_str.clear();
            }
        }

        if !current_num_str.is_empty()
            && any_neighbors(
                is_symbol,
                input,
                line_index,
                line.len(),
                current_num_str.len(),
            )
        {
            sum += current_num_str.parse::<u32>().unwrap();
        }
    }

    sum
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn any_neighbors(
    predicate: impl Fn(char) -> bool,
    schematic: &[&str],
    line_index: usize,
    end_index: usize,
    mut len: usize,
) -> bool {
    let predicate_at = |x: usize, y: usize| predicate(schematic[y].as_bytes()[x] as char);

    let schematic_height = schematic.len();
    let schematic_width = schematic[0].len();
    let mut start_index = end_index - len;

    if start_index + len < schematic_width {
        if predicate_at(start_index + len, line_index) {
            return true;
        }
        len += 1;
    }

    if start_index > 0 {
        start_index -= 1;
        len += 1;
        if predicate_at(start_index, line_index) {
            return true;
        }
    }

    if line_index > 0 {
        let y = line_index - 1;
        if schematic[y][start_index..start_index + len]
            .chars()
            .any(&predicate)
        {
            return true;
        }
    }

    if line_index + 1 < schematic_height {
        let y = line_index + 1;
        if schematic[y][start_index..start_index + len]
            .chars()
            .any(&predicate)
        {
            return true;
        }
    }

    false
}

fn puzzle_two(input: &[&str]) -> u32 {
    0 // I'm not stuck or anything, I know how to do this part. I'm just lazy.
}
