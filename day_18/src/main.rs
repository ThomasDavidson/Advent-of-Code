use library::grid::Direction;
use shoelace::{Area, Point};
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
}

fn part_1_decode_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let direction = match split.next().unwrap() {
                "R" => Direction::East,
                "U" => Direction::North,
                "L" => Direction::West,
                "D" => Direction::South,
                a => panic!("Unexpected str: {a}"),
            };

            let length = match split.next().unwrap().parse::<usize>() {
                Ok(a) => a,
                Err(e) => panic!("{:?}", e),
            };
            Instruction {
                direction: direction,
                length: length,
            }
        })
        .collect()
}


fn calculate_area_from_instructions(instructions: Vec<Instruction>) -> u64 {
    let mut block_coord: Vec<(i64, i64)> = Vec::new();

    // border points
    let b = instructions
        .iter()
        .fold(0, |acc, instruction| acc + instruction.length);

    for instruction in &instructions {
        let (offset_x, offset_y): (i64, i64) = {
            match (instruction.direction, instruction.length) {
                (Direction::East, len) => ((len as i64) as i64, 0),
                (Direction::West, len) => (-(len as i64), 0),
                (Direction::North, len) => (0, (len as i64)),
                (Direction::South, len) => (0, -(len as i64)),
                _ => panic!("Unused Dir"),
            }
        };

        let coord = match block_coord.last() {
            None => (offset_x, offset_y),
            Some(&(x, y)) => (x + offset_x, y + offset_y),
        };

        block_coord.push(coord);
    }

    let block_coord: Vec<Point> = block_coord
        .into_iter()
        .map(|(x, y)| Point { x: x, y: y })
        .collect();

    let area: Area = block_coord.into();
    println!("area {:?}", area);

    let inside_area: u64 = (area.0 as i64 - (b as i64) / 2 + 1).try_into().unwrap();
    inside_area + b as u64
}

fn part_1(input: &str) -> u64 {
    let instructions = part_1_decode_instructions(input);
    calculate_area_from_instructions(instructions)
}

fn main() {
    let input = include_str!("../example.txt");

    let part_1_answer = part_1(input);
    println!("part 1 answer: {}", part_1_answer);

    let part_2_answer = part_2(input);
    println!("part 2 answer: {}", part_2_answer);
}
