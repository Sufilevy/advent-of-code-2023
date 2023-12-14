use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    None,
}

impl Rock {
    pub fn from(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Square,
            '.' => Self::None,
            _ => unreachable!(),
        }
    }
}

pub enum VerticalDirection {
    North,
    South,
}

pub enum HorizontalDirection {
    West,
    East,
}
#[derive(Clone, PartialEq, Eq)]
pub struct Dish {
    platform: Vec<Vec<Rock>>,
}

impl Dish {
    pub fn from(input: &[&str]) -> Self {
        Self {
            platform: input
                .iter()
                .map(|line| line.chars().map(Rock::from).collect::<Vec<Rock>>())
                .collect(),
        }
    }

    pub fn tilt_vertically(&mut self, dir: VerticalDirection) {
        for col in 0..self.platform[0].len() {
            let mut empty_rows = VecDeque::<usize>::new();
            for row in 0..self.platform.len() {
                let row = match dir {
                    VerticalDirection::North => row,
                    VerticalDirection::South => self.platform.len() - 1 - row,
                };
                match self.platform[row][col] {
                    Rock::Round => {
                        if let Some(empty_row) = empty_rows.pop_front() {
                            self.platform[row][col] = Rock::None;
                            self.platform[empty_row][col] = Rock::Round;
                            empty_rows.push_back(row);
                        }
                    }
                    Rock::None => empty_rows.push_back(row),
                    Rock::Square => empty_rows.clear(),
                }
            }
        }
    }

    pub fn tilt_horizontally(&mut self, dir: HorizontalDirection) {
        for row in 0..self.platform.len() {
            let mut empty_cols = VecDeque::<usize>::new();
            for col in 0..self.platform[0].len() {
                let col = match dir {
                    HorizontalDirection::West => col,
                    HorizontalDirection::East => self.platform[0].len() - 1 - col,
                };
                match self.platform[row][col] {
                    Rock::Round => {
                        if let Some(empty_col) = empty_cols.pop_front() {
                            self.platform[row][col] = Rock::None;
                            self.platform[row][empty_col] = Rock::Round;
                            empty_cols.push_back(col);
                        }
                    }
                    Rock::None => empty_cols.push_back(col),
                    Rock::Square => empty_cols.clear(),
                }
            }
        }
    }

    pub fn north_load(&self) -> u32 {
        self.platform
            .iter()
            .rev()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .filter(|rock| matches!(rock, Rock::Round))
                    .count()
                    * (row + 1)
            })
            .sum::<usize>() as u32
    }
}

impl ToString for Dish {
    fn to_string(&self) -> String {
        let mut result = "".to_owned();
        for row in self.platform.iter() {
            for rock in row {
                result += match rock {
                    Rock::Round => "O",
                    Rock::Square => "#",
                    Rock::None => ".",
                }
            }
            result += "\n";
        }
        result
    }
}
