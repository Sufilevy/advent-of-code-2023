use grid::Grid;

type Point = (usize, usize);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }

    pub fn advance(&self, pos: Point) -> Point {
        match self {
            Self::Up => (pos.0 - 1, pos.1),
            Self::Down => (pos.0 + 1, pos.1),
            Self::Left => (pos.0, pos.1 - 1),
            Self::Right => (pos.0, pos.1 + 1),
        }
    }
}

pub struct Instruction {
    dir: Direction,
    meters: u8,
    color: String,
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap().chars().next().unwrap();
        let meters = parts.next().unwrap().parse::<u8>().unwrap();
        let color = parts
            .next()
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .to_string();

        Self {
            dir: Direction::from(dir),
            meters,
            color,
        }
    }
}

pub struct Lagoon {
    instructions: Vec<Instruction>,
    pit: Grid<bool>,
}

impl Lagoon {
    pub fn from(input: &[&str]) -> Self {
        let instructions = input.iter().map(|line| Instruction::from(line)).collect();
        let pit = Grid::new(700, 700);

        Self { instructions, pit }
    }

    pub fn dig(&mut self) {
        let mut current_pos = (300, 300);
        for instruction in &self.instructions {
            self.pit[current_pos] = true;

            for _ in 0..instruction.meters {
                current_pos = instruction.dir.advance(current_pos);
                self.pit[current_pos] = true;
            }
        }
    }

    pub fn capacity(&self) -> u32 {
        let mut capacity = 0;

        for row in self.pit.iter_rows() {
            let first = row.clone().enumerate().find(|&(_, block)| *block);

            let second = row.enumerate().rev().find(|&(_, block)| *block);

            if first.is_some() && second.is_some() {
                capacity += second.unwrap().0 - first.unwrap().0 + 1;
            }
        }

        capacity as u32
    }
}
