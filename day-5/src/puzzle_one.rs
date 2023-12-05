use std::ops::Range;

struct LayerRange {
    src: Range<u64>,
    dest: Range<u64>,
}

impl LayerRange {
    pub fn convert_seed(&self, seed: u64) -> Option<u64> {
        if self.src.contains(&seed) {
            let offset = seed - self.src.start;
            Some(self.dest.start + offset)
        } else {
            None
        }
    }
}

type Seeds = Vec<u64>;
type Layer = Vec<LayerRange>;

pub struct Almanac {
    pub seeds: Seeds,
    layers: Vec<Layer>,
}

impl Almanac {
    pub fn from(input: &str) -> Self {
        let mut parts = input.split("\r\n\r\n");

        let seeds = Self::parse_seeds(parts.next().unwrap());
        let layers = parts.map(Self::parse_map).collect();

        Self { seeds, layers }
    }

    fn parse_seeds(input: &str) -> Seeds {
        input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(str::parse::<u64>)
            .collect()
    }

    fn parse_map(input: &str) -> Layer {
        input.lines().skip(1).map(Self::parse_range).collect()
    }

    fn parse_range(input: &str) -> LayerRange {
        let parts = input
            .split_ascii_whitespace()
            .flat_map(str::parse::<u64>)
            .collect::<Vec<_>>();

        LayerRange {
            src: parts[1]..parts[1] + parts[2],
            dest: parts[0]..parts[0] + parts[2],
        }
    }

    pub fn convert_seed(&self, seed: u64) -> u64 {
        self.layers.iter().fold(seed, |current_seed, layer| {
            let new_seed = layer
                .iter()
                .filter_map(|range| range.convert_seed(current_seed))
                .next();

            new_seed.unwrap_or(current_seed)
        })
    }
}
