use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let num_winning = num_winning(line);
            if num_winning == 0 {
                return 0;
            }

            2u32.pow(num_winning - 1)
        })
        .sum()
}

fn num_winning(line: &str) -> u32 {
    let mut parts = line.split(':').nth(1).unwrap().split(" | ");
    let winning_numbers: Vec<u8> = get_numbers(parts.next().unwrap()).collect();
    let your_numbers = get_numbers(parts.next().unwrap());

    your_numbers
        .filter(|number| winning_numbers.contains(number))
        .count() as u32
}

fn get_numbers(part: &str) -> impl Iterator<Item = u8> + '_ {
    part.split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
}

type Cards = Vec<(usize, u32)>;

fn puzzle_two(input: &[&str]) -> u32 {
    let cards: Cards = input
        .iter()
        .enumerate()
        .map(|(index, line)| (index, num_winning(line)))
        .collect();

    cards
        .iter()
        .map(|&(card_index, _)| num_cards_from_card(card_index, &cards))
        .sum::<u32>()
        + cards.len() as u32
}

fn num_cards_from_card(card_index: usize, cards: &Cards) -> u32 {
    let num_winning = cards[card_index].1;
    cards
        .iter()
        .skip(card_index + 1)
        .take(num_winning as usize)
        .map(|&(current_card_index, _)| num_cards_from_card(current_card_index, cards))
        .sum::<u32>()
        + num_winning
}
