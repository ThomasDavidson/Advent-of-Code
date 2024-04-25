use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn get_translation(self) -> (i16, i16) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BeamState {
    direction: Direction,
    x: usize,
    y: usize,
}
impl BeamState {
    fn check_bounds(&self, width: usize, height: usize) -> bool {
        match self.direction {
            Direction::South => {
                if self.y + 1 == height {
                    false
                } else {
                    true
                }
            }
            Direction::East => {
                if self.x + 1 == width {
                    false
                } else {
                    true
                }
            }
            Direction::North => {
                if self.y == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::West => {
                if self.x == 0 {
                    false
                } else {
                    true
                }
            }
        }
    }
}

fn beam_move(contraction: &Vec<Vec<char>>, state: BeamState) -> Vec<BeamState> {
    let tile = contraction[state.y][state.x];

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
        .map(|&direction| BeamState {
            direction: direction,
            ..state
        })
        .filter(|state| state.check_bounds(width, height))
        .map(|state| {
            let (x, y) = state.direction.get_translation();
            // println!("x {x} y {y} -> {} {}", state.x as i16 + x, state.y as i16 + y);
            BeamState {
                x: (state.x as i16 + x) as usize,
                y: (state.y as i16 + y) as usize,
                ..state
            }
        })
        .collect()
}

fn get_energized_count(contraction: &Vec<Vec<char>>, initial: &BeamState) -> usize {
    // North, East, South, West
    let mut visited: Vec<Vec<[bool; 4]>> = contraction
        .iter()
        .map(|a| a.iter().map(|_| [false; 4]).collect())
        .collect();

    let mut states = vec![initial.clone()];
    visited[initial.y][initial.x][Direction::East as usize] = true;

    while !states.is_empty() {
        let new_states = states
            .into_iter()
            .map(|state| beam_move(&contraction, state));

        states = new_states
            .flat_map(|a| a)
            // check if spot has been visited in same direction
            .filter(|state| !visited[state.y][state.x][state.direction as usize])
            .collect();
        for state in &states {
            visited[state.y][state.x][state.direction as usize] = true;
        }
    }

    visited
        .into_iter()
        .flatten()
        .map(|tile| tile.iter().any(|&a| a))
        .fold(0, |acc, b| acc + if b { 1 } else { 0 })
}

fn part_1(contraction: &Vec<Vec<char>>) -> usize {
    let initial = BeamState {
        direction: Direction::East,
        x: 0,
        y: 0,
    };

    get_energized_count(contraction, &initial)
}

fn part_2(contraction: &Vec<Vec<char>>) -> usize {
    let width = (contraction[0].len() - 1) as usize;
    let height = (contraction.len() - 1) as usize;

    let north_initial: Vec<BeamState> = (0..width)
        .map(|i| BeamState {
            x: i,
            y: 0,
            direction: Direction::South,
        })
        .collect();

    let west_initial: Vec<BeamState> = (0..width)
        .map(|i| BeamState {
            x: 0,
            y: i,
            direction: Direction::South,
        })
        .collect();
    let east_initial: Vec<BeamState> = (0..width)
        .map(|i| BeamState {
            x: i,
            y: width,
            direction: Direction::South,
        })
        .collect();

    let south_initial: Vec<BeamState> = (0..width)
        .map(|i| BeamState {
            x: i,
            y: height,
            direction: Direction::South,
        })
        .collect();

    let inital_states: Vec<BeamState> = vec![
        north_initial,
        west_initial,
        east_initial,
        south_initial,
    ].into_iter().flatten().collect();


    inital_states
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
    println!("Part 1 anwer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&contraction);
    let duration = start.elapsed();
    println!("Part 2 anwer: {}, time: {:?}", part_2_answer, duration);
}
