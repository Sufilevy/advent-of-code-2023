use std::collections::HashSet;

use grid::Grid;
use priority_queue::PriorityQueue;

/// (row, col)
type BlockPos = (usize, usize);

/// The total cost of the best path to get to a block.
type TotalCost = u16;

struct Block {
    pub cost: u8,
    pub total_cost: TotalCost,
}

pub struct City {
    blocks: Grid<Block>,
    visited: HashSet<BlockPos>,
}

impl City {
    const STARTING_BLOCK_TOTAL_COST: u16 = 0;

    pub fn from(input: &[&str]) -> Self {
        let mut blocks: Vec<Block> = input
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| Block {
                        cost: c.to_digit(10).unwrap() as u8,
                        total_cost: u16::MAX,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        blocks[0].total_cost = Self::STARTING_BLOCK_TOTAL_COST;

        City {
            blocks: Grid::from_vec(blocks, input.len()),
            visited: HashSet::new(),
        }
    }

    pub fn get_best_path_cost(&mut self) -> u32 {
        let mut unvisited = PriorityQueue::<BlockPos, TotalCost>::new();
        unvisited.push((0, 0), Self::STARTING_BLOCK_TOTAL_COST);

        while let Some((block, cost)) = unvisited.pop() {
            if self.is_target_block(block) {
                break;
            }

            self.visited.insert(block);
            let neighbors = self.unvisited_neighbors_of(block);
        }

        0
    }

    fn is_target_block(&self, block: BlockPos) -> bool {
        block == (self.blocks.rows() - 1, self.blocks.cols() - 1)
    }

    fn unvisited_neighbors_of(&self, block: (usize, usize)) -> Vec<BlockPos> {
        todo!()
    }
}
