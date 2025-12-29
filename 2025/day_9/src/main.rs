use itertools::Itertools;
use library::grid::Vec2;
use library::input::{Day, InputType};

type Coord = Vec2<usize>;

struct Theater {
    red_tiles: Vec<Coord>,
}
impl Theater {
    fn parse(input: &str) -> Self {
        let red_tiles: Vec<Coord> = input
            .lines()
            .map(|line| line.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .map(|(x, y)| Coord { x, y })
            .collect();

        Self { red_tiles }
    }
}

fn rectangle_size(coord1: &Coord, coord2: &Coord) -> u64 {
    let x_diff = (coord1.x.abs_diff(coord2.x) + 1) as u64;
    let y_diff = (coord1.y.abs_diff(coord2.y) + 1) as u64;
    x_diff * y_diff
}

struct Day9;
const DAY: Day9 = Day9;
impl Day<u64> for Day9 {
    fn part_1(&self, input: &str) -> u64 {
        let theater = Theater::parse(input);

        let mut part_1_answer = 0;

        for red_tile in theater.red_tiles.iter().combinations(2) {
            let tile1 = red_tile[0];
            let tile2 = red_tile[1];

            part_1_answer = part_1_answer.max(rectangle_size(tile1, tile2))
        }
        part_1_answer
    }
    fn part_2(&self, _input: &str) -> u64 {
        0
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
