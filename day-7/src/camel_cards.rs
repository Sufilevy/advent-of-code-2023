use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    StraightThree,
    FullHouse,
    StraightFour,
    StraightFive,
}

impl Kind {
    pub fn from_cards(cards: &str) -> Self {
        for card in cards.chars() {
            match cards.chars().filter(|&c| c == card).count() {
                5 => return Kind::StraightFive,
                4 => return Kind::StraightFour,
                3 => {
                    let other_cards: String = cards.chars().filter(|&c| c != card).collect();
                    return match Self::from_cards(&other_cards) {
                        Kind::OnePair => Kind::FullHouse,
                        _ => Kind::StraightThree,
                    };
                }
                2 => {
                    let other_cards: String = cards.chars().filter(|&c| c != card).collect();
                    return match Self::from_cards(&other_cards) {
                        Kind::StraightThree => Kind::FullHouse,
                        Kind::OnePair => Kind::TwoPair,
                        _ => Kind::OnePair,
                    };
                }
                _ => {}
            }
        }
        Self::HighCard
    }
}
pub struct Hand<'a> {
    kind: Kind,
    cards: &'a str,
    pub bid: u32,
}

impl<'a> Hand<'a> {
    pub fn from_line(cards: &'a str) -> Self {
        let mut parts = cards.split_ascii_whitespace();
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();

        Self {
            kind: Kind::from_cards(cards),
            cards,
            bid,
        }
    }

    pub fn cmp(&self, other: &Hand) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.cmp_cards(other),
            ord => ord,
        }
    }

    fn cmp_cards(&self, other: &Hand) -> Ordering {
        self.cards
            .chars()
            .zip(other.cards.chars())
            .find_map(|(self_card, other_card)| {
                let self_card_value = Self::value_of_card(self_card);
                let other_card_value = Self::value_of_card(other_card);

                match self_card_value.cmp(&other_card_value) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                }
            })
            .unwrap_or(Ordering::Equal)
    }

    fn value_of_card(card: char) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
}
