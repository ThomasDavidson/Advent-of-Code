use std::{collections::btree_map::Keys, time::Duration};

#[derive(PartialEq)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}
struct Tile {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_tile_connections(symbol: &char) -> Vec<Direction> {
    match symbol {
        '|' => vec![
            Direction::North,
            Direction::South,
        ],
        '-' => vec![
            Direction::East,
            Direction::West,
        ],
        'L' => vec![
            Direction::North,
            Direction::East,
        ],
        'J' => vec![
            Direction::North,
            Direction::West,
        ],
        '7' => vec![
            Direction::South,
            Direction::West,
        ],
        'F' => vec![
            Direction::South,
            Direction::East,
        ],
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

fn check_direction(lines: &Vec<&str>, coord: &Coord, dir: Direction) -> bool {
    let width = lines[0].len();
    let height = lines.len();
    // check if out of bounds
    if coord.x + 1 > width {
        return false;
    }
    if coord.y + 1 > height {
        return false;
    }
    if coord.x == 0 {
        return false;
    }
    if coord.y == 0 {
        return false;
    }
    true
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
                return Some(Coord { x: x, y: y });
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../example.txt");

    let lines: Vec<&str> = input.lines().collect();

    let starting_point = match get_start(input) {
        Some(a) => a,
        None => panic!("No start"),
    };
    println!("Starting point: {:?}", starting_point);

    let mut current_location = starting_point;
    let mut previous_direction: Direction = Direction::None;

    while true {
        let current_symbol = get_symbol(&lines, &current_location);

        let tile = get_tile_connections(&current_symbol);

        for dir in tile {
            if previous_direction != dir {
                // check bounries
                if check_direction(&lines, &current_location, Direction::North) {
                    previous_direction = Direction::North;
                }
            }
        }
        break;
    }
}
