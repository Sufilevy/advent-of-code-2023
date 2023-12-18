use grid::Grid;
use priority_queue::PriorityQueue;

type Block = (usize, usize);

pub struct City {
    blocks: Grid<u8>,
}

impl City {
    pub fn from(input: &[&str]) -> Self {
        let blocks = input
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        City {
            blocks: Grid::from_vec(blocks, input.len()),
        }
    }

    pub fn find_best_path(&self) -> u32 {
        let mut visited_blocks = Vec::new();
        let mut pushed_blocks = Vec::new();

        let mut to_visit = PriorityQueue::<Block, u8>::new();
        to_visit.push((0, 0), 0);

        while let Some((block, cost)) = to_visit.pop() {
            if block == (self.blocks.rows() - 1, self.blocks.cols() - 1) {
                break;
            }
        }
        0
    }
}
