use std::time::Instant;

use library::grid::{Direction, GridState};

fn beam_move(contraction: &Vec<Vec<char>>, state: GridState) -> Vec<GridState> {
    let tile = contraction[state.coords.y][state.coords.x];

    let directions: Vec<Direction> = match (tile, state.direction) {
        ('.', _) => vec![state.direction],
        ('-', Direction::North | Direction::South) => vec![Direction::East, Direction::West],
        ('|', Direction::East | Direction::West) => vec![Direction::North, Direction::South],
        ('|' | '-', _) => vec![state.direction],
        ('/', Direction::East) => vec![Direction::North],
        ('/', Direction::North) => vec![Direction::East],
        ('/', Direction::South) => vec![Direction::West],
        ('/', Direction::West) => vec![Direction::South],
        ('\\', Direction::East) => vec![Direction::South],
        ('\\', Direction::North) => vec![Direction::West],
        ('\\', Direction::South) => vec![Direction::East],
        ('\\', Direction::West) => vec![Direction::North],
        _ => panic!("Unexpected input: {} {:?}", tile, state.direction),
    };

    // bounds check
    let width = contraction[0].len();
    let height = contraction.len();

    directions
        .iter()
        .map(|&direction| GridState { direction, ..state })
        .filter(|state| state.check_bounds(width, height))
        .map(|state| {
            let (x, y): (i16, i16) = state.direction.get_translation();
            GridState {
                coords: (state.coords + state.direction).unwrap(),
                ..state
            }
        })
        .collect()
}

fn get_energized_count(contraction: &Vec<Vec<char>>, initial: &GridState) -> usize {
    // North, East, South, West
    let mut visited: Vec<Vec<[bool; 5]>> = contraction
        .iter()
        .map(|a| a.iter().map(|_| [false; 5]).collect())
        .collect();

    let mut states = vec![initial.clone()];
    visited[initial.coords.y][initial.coords.x][Direction::East as usize] = true;

    while !states.is_empty() {
        let new_states = states
            .into_iter()
            .map(|state| beam_move(&contraction, state));

        states = new_states
            .flat_map(|a| a)
            // check if spot has been visited in same direction
            .filter(|state| !visited[state.coords.y][state.coords.x][state.direction as usize])
            .collect();
        for state in &states {
            visited[state.coords.y][state.coords.x][state.direction as usize] = true;
        }
    }

    visited
        .into_iter()
        .flatten()
        .map(|tile| tile.iter().any(|&a| a))
        .fold(0, |acc, b| acc + if b { 1 } else { 0 })
}

fn part_1(contraction: &Vec<Vec<char>>) -> usize {
    let initial = GridState::new(0, 0, Direction::East);

    get_energized_count(contraction, &initial)
}

fn part_2(contraction: &Vec<Vec<char>>) -> usize {
    let width = contraction[0].len() - 1;
    let height = contraction.len() - 1;

    let north_initial: Vec<GridState> = (0..width)
        .map(|i| GridState::new(i, 0, Direction::South))
        .collect();

    let west_initial: Vec<GridState> = (0..width)
        .map(|i| GridState::new(0, i, Direction::South))
        .collect();
    let east_initial: Vec<GridState> = (0..width)
        .map(|i| GridState::new(i, width, Direction::South))
        .collect();

    let south_initial: Vec<GridState> = (0..width)
        .map(|i| GridState::new(i, height, Direction::South))
        .collect();

    let initial_states: Vec<GridState> =
        vec![north_initial, west_initial, east_initial, south_initial]
            .into_iter()
            .flatten()
            .collect();

    initial_states
        .iter()
        .map(|initial| get_energized_count(contraction, initial))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    let contraction: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&contraction);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&contraction);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}
