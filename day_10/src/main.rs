use std::time::Instant;
use library::grid::{Direction, UVec2};
type Coord = UVec2<usize>;

fn get_tile_connections(symbol: &char) -> Vec<Direction> {
    match symbol {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        '.' => vec![],
        'S' => vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ],
        _ => panic!("above should match all"),
    }
}

fn check_direction(lines: &Vec<&str>, coord: &Coord, dir: &Direction) -> Option<Coord> {
    let width = lines[0].len();
    let height = lines.len();

    // check if out of bounds for each direction
    let next_coord = match *coord + *dir {
        Ok(res) => res,
        Err(_) => return None,
    };

    if next_coord.check_bounds(width, height) {
        return None;
    }

    Some(next_coord)
}

fn get_symbol(lines: &Vec<&str>, coord: &Coord) -> char {
    let row = lines.get(coord.y).unwrap();
    let symbol_row: Vec<char> = row.chars().collect();
    let symbol = symbol_row.get(coord.x).unwrap();

    symbol.clone()
}

fn get_start(input: &str) -> Option<Coord> {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if 'S' == c {
                return Some(Coord::new(x, y));
            }
        }
    }
    None
}

fn get_next_tile(
    lines: &Vec<&str>,
    back: Direction,
    current_symbol: char,
    current_location: Coord,
) -> Option<(Coord, Direction)> {
    let current_dirs = get_tile_connections(&current_symbol);

    for dir in current_dirs {
        // skip if visited previously
        if back == dir {
            continue;
        }
        // skip if out of bounds
        let next_coord = match check_direction(&lines, &current_location, &dir) {
            None => {
                continue;
            }
            Some(a) => a,
        };

        let next_tile = get_symbol(&lines, &next_coord);

        let next_dirs = get_tile_connections(&next_tile);

        // switch direction
        let required_dir = match dir {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::None => panic!("Should not be None"),
        };
        // check if the next current_dirs has a pipe facing in this direction
        if !next_dirs.contains(&required_dir) {
            continue;
        }

        // sets to opposite of current_direction
        return Some((next_coord, required_dir));
    }
    return None;
}

fn part_1(input: &str) -> u64 {
    let mut part_1_answer: u64 = 0;

    let lines: Vec<&str> = input.lines().collect();

    let starting_point = match get_start(input) {
        Some(a) => a,
        None => panic!("No start"),
    };

    let mut current_location = starting_point;
    let mut back: Direction = Direction::None;

    // the number of nodes searched will never be larger than the size of the input
    for i in 0..input.len() {
        let current_symbol = get_symbol(&lines, &current_location);

        if current_symbol == 'S' && i != 0 {
            break;
        }
        part_1_answer += 1;

        let next_tile_res = get_next_tile(&lines, back, current_symbol, current_location);
        match next_tile_res {
            Some(a) => {
                current_location = a.0;
                back = a.1;
            }
            None => panic!("Hit dead end"),
        }
    }

    part_1_answer / 2
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);
}
