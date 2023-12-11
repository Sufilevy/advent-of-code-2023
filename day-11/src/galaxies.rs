/// (y, x)
type Point = (usize, usize);

pub struct Galaxy {
    galaxies: Vec<Point>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Galaxy {
    pub fn from(input: &[&str]) -> Self {
        let galaxies = input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.char_indices()
                    .filter_map(move |(x, ch)| (ch == '#').then_some((y, x)))
            })
            .collect::<Vec<Point>>();

        let empty_rows = input
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.chars().all(|c| c == '.').then_some(index))
            .collect::<Vec<usize>>();

        let empty_cols = input[0]
            .char_indices()
            .filter_map(|(index, _)| {
                input
                    .iter()
                    .all(|row| row.chars().nth(index).unwrap() == '.')
                    .then_some(index)
            })
            .collect::<Vec<usize>>();

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    pub fn shortest_paths_lengths(&self, expansion_size: u64) -> Vec<u64> {
        let num_galaxies = self.galaxies.len();
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(index, &first)| {
                (index + 1..num_galaxies).map(move |second_index| {
                    self.shortest_path_length(first, self.galaxies[second_index], expansion_size)
                })
            })
            .collect()
    }

    fn shortest_path_length(&self, first: Point, second: Point, expansion_size: u64) -> u64 {
        let end_point = (first.0.max(second.0), first.1.max(second.1));
        let mut current_point = (first.0.min(second.0), first.1.min(second.1));

        let mut steps = 0;
        loop {
            if current_point >= end_point {
                break;
            }
            if current_point.0 < end_point.0 {
                steps += if self.empty_rows.contains(&current_point.0) {
                    expansion_size
                } else {
                    1
                };
                current_point.0 += 1;
            }
            if current_point.1 < end_point.1 {
                steps += if self.empty_cols.contains(&current_point.1) {
                    expansion_size
                } else {
                    1
                };
                current_point.1 += 1;
            }
        }
        steps
    }
}
