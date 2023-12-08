use std::collections::BTreeMap;

type Node<'a> = (&'a str, &'a str);

pub struct Network<'a> {
    instructions: &'a str,
    nodes: BTreeMap<&'a str, Node<'a>>,
}

impl<'a> Network<'a> {
    pub fn from(input: &'a str) -> Self {
        let mut parts = input.split("\r\n\r\n");
        let instructions = parts.next().unwrap();
        let nodes = parts.next().unwrap().lines().map(Self::node_from).collect();

        Self {
            instructions,
            nodes,
        }
    }

    fn node_from(input: &'a str) -> (&'a str, Node<'a>) {
        let mut parts = input.split(" = ");
        let key = parts.next().unwrap();
        let mut parts = parts.next().unwrap().split(", ");
        (
            key,
            (
                parts.next().unwrap().strip_prefix('(').unwrap(),
                parts.next().unwrap().strip_suffix(')').unwrap(),
            ),
        )
    }

    pub fn traverse(&self) -> u32 {
        let mut current_node = "AAA";
        self.instructions
            .chars()
            .cycle()
            .enumerate()
            .find_map(|(index, instruction)| {
                current_node = match instruction {
                    'L' => self.nodes[current_node].0,
                    _ => self.nodes[current_node].1,
                };

                if current_node == "ZZZ" {
                    Some(index as u32 + 1)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn traverse2(&self) -> u64 {
        let results: Vec<u64> = self
            .get_starting_nodes()
            .iter()
            .map(|&node| {
                let mut current_node = node;
                self.instructions
                    .chars()
                    .cycle()
                    .enumerate()
                    .find_map(|(index, instruction)| {
                        current_node = match instruction {
                            'L' => self.nodes[current_node].0,
                            _ => self.nodes[current_node].1,
                        };

                        if current_node.ends_with('Z') {
                            Some(index as u64 + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .collect();

        Self::lcm(&results)
    }

    fn get_starting_nodes(&self) -> Vec<&'a str> {
        self.nodes
            .keys()
            .filter_map(|node| node.ends_with('A').then_some(*node))
            .collect()
    }

    fn lcm(numbers: &[u64]) -> u64 {
        let a = numbers[0];
        if numbers.len() == 1 {
            return a;
        }
        let b = Self::lcm(&numbers[1..]);
        a * b / Self::gcd(a, b)
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
