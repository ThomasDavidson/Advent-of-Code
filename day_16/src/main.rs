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

fn main() {
    let input = include_str!("../input.txt");

    let contraction: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // for line in &contraction {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // North, East, South, West
    let mut visited: Vec<Vec<[bool; 4]>> = contraction
        .iter()
        .map(|a| a.iter().map(|_| [false; 4]).collect())
        .collect();

    let mut states = vec![BeamState {
        direction: Direction::East,
        x: 0,
        y: 0,
    }];
    visited[0][0][Direction::East as usize] = true;

    while !states.is_empty() {
        // println!("State {:?}", states);
        let new_states = states
            .into_iter()
            .map(|state| beam_move(&contraction, state));
        // println!("{:?}", new_states);

        states = new_states
            .flat_map(|a| a)
            // check if spot has been visited in same direction
            .filter(|state| !visited[state.y][state.x][state.direction as usize])
            .collect();
        for state in &states {
            visited[state.y][state.x][state.direction as usize] = true;
        }
    }

    // println!("");

    // for line in &visited {
    //     for tile in line {
    //         match tile.iter().any(|&a| a) {
    //             true => print!("#"),
    //             false => print!("."),
    //         }
    //     }
    //     println!("");
    // }

    let part_1_answer = visited
        .into_iter()
        .flatten()
        .map(|tile| tile.iter().any(|&a| a))
        .fold(0, |acc, b| acc + if b { 1 } else { 0 });
    println!("part 1 answer: {}", part_1_answer);
}
