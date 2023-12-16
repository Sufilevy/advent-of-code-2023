use grid::Grid;
use smallvec::{smallvec, SmallVec};

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn advance_pos(&self, pos: Position, rows: usize, cols: usize) -> Option<Position> {
        use Direction::*;
        let new_pos = match self {
            Up => (pos.0.saturating_sub(1), pos.1),
            Down => ((pos.0 + 1).min(rows - 1), pos.1),
            Left => (pos.0, pos.1.saturating_sub(1)),
            Right => (pos.0, (pos.1 + 1).min(cols - 1)),
        };

        (pos != new_pos).then_some(new_pos)
    }
}

#[derive(Clone)]
enum Tile {
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Empty,
}

impl Tile {
    pub fn from(c: char) -> Self {
        match c {
            '/' => Tile::ForwardMirror,
            '\\' => Tile::BackwardMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            '.' => Tile::Empty,
            _ => unreachable!(),
        }
    }

    pub fn pass_beam(&self, beam_dir: Direction) -> SmallVec<[Direction; 2]> {
        use Direction::*;
        use Tile::*;
        match self {
            ForwardMirror => match beam_dir {
                Up => smallvec![Right],
                Down => smallvec![Left],
                Left => smallvec![Down],
                Right => smallvec![Up],
            },
            BackwardMirror => match beam_dir {
                Up => smallvec![Left],
                Down => smallvec![Right],
                Left => smallvec![Up],
                Right => smallvec![Down],
            },
            VerticalSplitter => match beam_dir {
                Up | Down => smallvec![beam_dir],
                Left | Right => smallvec![Up, Down],
            },
            HorizontalSplitter => match beam_dir {
                Up | Down => smallvec![Left, Right],
                Left | Right => smallvec![beam_dir],
            },
            Empty => smallvec![beam_dir],
        }
    }
}

#[derive(Clone)]
pub struct Mirrors {
    tiles: Grid<Tile>,
    energized_tiles: Grid<bool>,
    visited_tiles: Vec<(Position, Direction)>,
}

impl Mirrors {
    pub fn from(input: &[&str]) -> Self {
        let tiles = input
            .iter()
            .flat_map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect();

        Self {
            tiles: Grid::from_vec(tiles, input.len()),
            energized_tiles: Grid::new(input.len(), input[0].len()),
            visited_tiles: Vec::new(),
        }
    }

    pub fn advance_beam(&mut self, beam_start: Position, beam_dir: Direction) {
        let (rows, cols) = (self.tiles.rows(), self.tiles.cols());

        let mut beam_pos = beam_start;
        let mut beam_dir = beam_dir;
        loop {
            self.energized_tiles[beam_pos] = true;

            let new_dirs = self.tiles[beam_pos].pass_beam(beam_dir);
            beam_dir = new_dirs[0];
            if let Some(new_dir) = new_dirs.get(1) {
                self.advance_beam(beam_pos, *new_dir);
            }

            if let Some(new_pos) = beam_dir.advance_pos(beam_pos, rows, cols) {
                beam_pos = new_pos;
            } else {
                break;
            }

            if self.visited_tiles.contains(&(beam_pos, beam_dir)) {
                break;
            }
            self.visited_tiles.push((beam_pos, beam_dir));
        }
    }

    pub fn count_energized_tiles(&self) -> u32 {
        self.energized_tiles.iter().filter(|&x| *x).count() as u32
    }

    pub fn rows(&self) -> usize {
        self.tiles.rows()
    }

    pub fn cols(&self) -> usize {
        self.tiles.cols()
    }
}
