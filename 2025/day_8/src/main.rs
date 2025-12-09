use itertools::Itertools;
use library::grid::Vec3;
use library::input::{Day, InputType};

type Coord = Vec3<i64>;

#[derive(Debug, Clone)]
struct Circuit {
    junctions: Vec<Coord>,
}
impl Circuit {
    fn new(junction: Coord) -> Self {
        Self {
            junctions: vec![junction],
        }
    }
    fn contains(&self, junction: &Coord) -> bool {
        self.junctions.contains(junction)
    }
    fn append(&mut self, circuits: Self) {
        let mut junctions = circuits.junctions;
        self.junctions.append(&mut junctions)
    }
}

#[derive(Debug)]
struct Circuits {
    circuits: Vec<Circuit>,
}
impl Circuits {
    fn from_playground(playground: Playground) -> Self {
        Self {
            circuits: playground
                .junction_boxes
                .into_iter()
                .map(Circuit::new)
                .collect(),
        }
    }

    fn connect(&mut self, junction_1: &Coord, junction_2: &Coord) {
        let contains_1 = self.position(junction_1);
        let contains_2 = self.position(junction_2);

        match (contains_1, contains_2) {
            (Some(position_1), Some(position_2)) => {
                if position_1 == position_2 {
                    return;
                }

                let circuit = self.circuits[position_1].clone();
                self.circuits[position_2].append(circuit);

                self.circuits.remove(position_1);
            }
            _ => panic!(),
        }
    }
    fn position(&self, coord: &Coord) -> Option<usize> {
        self.circuits
            .iter()
            .position(|circuit| circuit.contains(coord))
    }
    fn three_largest(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = self
            .circuits
            .iter()
            .map(|circuit| circuit.junctions.len())
            .collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes[0..3].to_vec()
    }
}

struct Playground {
    junction_boxes: Vec<Coord>,
}
impl Playground {
    fn parse(text: &str) -> Self {
        Self {
            junction_boxes: text.lines().filter_map(Coord::parse).collect(),
        }
    }
    fn calculate_distances(&self) -> Vec<(u32, Coord, Coord)> {
        self.junction_boxes
            .iter()
            .combinations(2)
            .map(|boxes| (boxes[0], boxes[1]))
            .map(|(junction_1, junction_2)| (*junction_1 - *junction_2, *junction_1, *junction_2))
            .map(|(diff, junction_1, junction_2)| {
                (
                    (diff.x.pow(2) + diff.y.pow(2) + diff.z.pow(2))
                        .isqrt()
                        .try_into()
                        .unwrap(),
                    junction_1,
                    junction_2,
                )
            })
            .collect()
    }
}

const NUM_PAIRS: usize = 1000; // 10; 
struct Day8;
const DAY: Day8 = Day8;
impl Day<u32> for Day8 {
    fn part_1(&self, input: &str) -> u32 {
        let playground = Playground::parse(input);

        let mut distances = playground.calculate_distances();
        distances.sort_by(|a, b| a.0.cmp(&b.0));

        let mut circuits = Circuits::from_playground(playground);

        for (_distance, junction_1, junction_2) in &distances[0..NUM_PAIRS] {
            circuits.connect(junction_1, junction_2);
        }

        circuits
            .three_largest()
            .iter()
            .fold(1, |acc, &x| acc * (x as u32))
    }
    fn part_2(&self, input: &str) -> u32 {
        let playground = Playground::parse(input);

        let mut distances = playground.calculate_distances();
        distances.sort_by(|a, b| a.0.cmp(&b.0));

        let mut circuits = Circuits::from_playground(playground);

        let mut last: Option<(Coord, Coord)> = None;
        for (_distance, junction_1, junction_2) in &distances {
            circuits.connect(junction_1, junction_2);

            if circuits.circuits.len() == 1 {
                last = Some((*junction_1, *junction_2));
                break;
            }
        }

        if let Some(last) = last {
            (last.0.x * last.1.x) as u32
        } else {
            panic!()
        }
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
