enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl SpringState {
    pub fn from(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

struct SpringsRow {
    row: Vec<SpringState>,
    groups_lengths: Vec<u8>,
}

impl SpringsRow {
    pub fn from(line: &str) -> Self {
        let mut parts = line.split_ascii_whitespace();

        let row = parts
            .next()
            .unwrap()
            .chars()
            .map(SpringState::from)
            .collect();

        let groups_lengths = parts
            .next()
            .unwrap()
            .split(',')
            .flat_map(str::parse::<u8>)
            .collect();

        Self {
            row,
            groups_lengths,
        }
    }

    pub fn count_arrangements(&self) -> u32 {
        0
    }
}

pub struct HotSprings {
    springs: Vec<SpringsRow>,
}

impl HotSprings {
    pub fn from(input: &[&str]) -> Self {
        Self {
            springs: input.iter().map(|&line| SpringsRow::from(line)).collect(),
        }
    }
}
