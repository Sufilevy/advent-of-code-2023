use grid::Grid;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Tile {
    Pipe(Direction, Direction),
    Start,
    Ground,
}

impl Tile {
    fn from(c: char) -> Self {
        use Direction::*;
        use Tile::*;
        match c {
            '|' => Pipe(North, South),
            '-' => Pipe(East, West),
            'L' => Pipe(North, East),
            'J' => Pipe(North, West),
            '7' => Pipe(South, West),
            'F' => Pipe(South, East),
            'S' => Start,
            _ => Ground,
        }
    }

    fn has_direction(&self, dir: Direction) -> bool {
        match self {
            &Self::Pipe(first, second) => first == dir || second == dir,
            Self::Start => true,
            Self::Ground => false,
        }
    }

    fn directions(&self) -> Vec<Direction> {
        use Direction::*;
        match self {
            &Self::Pipe(first, second) => vec![first, second],
            Self::Start => vec![North, South, East, West],
            Self::Ground => unreachable!(),
        }
    }
}

type TilePosition = (usize, usize);

pub struct Pipes {
    tiles: Grid<Tile>,
    starting_pipe: TilePosition,
}

impl Pipes {
    pub fn from(input: &[&str]) -> Self {
        let tiles = input
            .iter()
            .flat_map(|line| line.chars().map(Tile::from))
            .collect::<Vec<Tile>>();
        let tiles = Grid::from_vec(tiles, input[0].len());
        Self {
            starting_pipe: Self::starting_pipe_pos(&tiles),
            tiles,
        }
    }

    pub fn pipes_loop(&self) -> Vec<TilePosition> {
        let mut pipes = vec![self.starting_pipe];
        let mut last_dir = self.next_pipe_of(&self.starting_pipe, None);
        let mut current_tile = self.next_tile_to_direction(&self.starting_pipe, last_dir);

        while current_tile != self.starting_pipe {
            pipes.push(current_tile);
            last_dir = self.next_pipe_of(&current_tile, Some(last_dir));
            current_tile = self.next_tile_to_direction(&current_tile, last_dir);
        }

        pipes
    }

    fn starting_pipe_pos(tiles: &Grid<Tile>) -> TilePosition {
        tiles
            .indexed_iter()
            .find(|&(_, tile)| *tile == Tile::Start)
            .unwrap()
            .0
    }

    fn next_pipe_of(&self, tile: &TilePosition, last_direction: Option<Direction>) -> Direction {
        for dir in self.tiles[*tile].directions() {
            let opposite = dir.opposite();
            let next_tile = self.next_tile_to_direction(tile, dir);
            if last_direction != Some(opposite) && self.tiles[next_tile].has_direction(opposite) {
                return dir;
            }
        }
        unreachable!()
    }

    fn next_tile_to_direction(&self, tile: &TilePosition, dir: Direction) -> TilePosition {
        match dir {
            Direction::North => (tile.0 - 1, tile.1),
            Direction::South => (tile.0 + 1, tile.1),
            Direction::East => (tile.0, tile.1 + 1),
            Direction::West => (tile.0, tile.1 - 1),
        }
    }

    pub fn count_enclosed_tiles(&self, input: &[&str]) -> u32 {
        let pipes_loop = self.pipes_loop();
        input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                let mut crossings = 0;
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        'S' | '|' | 'F' | '7' if pipes_loop.contains(&(y, x)) => {
                            crossings += 1;
                            0
                        }
                        _ if !pipes_loop.contains(&(y, x)) => {
                            if crossings % 2 != 0 {
                                1
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}
