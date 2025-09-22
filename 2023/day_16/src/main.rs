use library::grid::{Direction, GridState};
use library::input::{Day, InputType};

struct Contraption {
    layout: Vec<Vec<char>>,
}
impl Contraption {
    fn parse(input: &str) -> Self {
        Self {
            layout: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn get_energized_count(&self, initial: &GridState) -> usize {
        // North, East, South, West
        let mut visited: Vec<Vec<[bool; 5]>> = self
            .layout
            .iter()
            .map(|a| a.iter().map(|_| [false; 5]).collect())
            .collect();

        let mut states = vec![*initial];
        visited[initial.coords.y][initial.coords.x][Direction::East as usize] = true;

        while !states.is_empty() {
            let new_states = states
                .into_iter()
                .map(|state| beam_move(&self.layout, state));

            states = new_states
                .flatten()
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
}

fn beam_move(contraption: &[Vec<char>], state: GridState) -> Vec<GridState> {
    let tile = contraption[state.coords.y][state.coords.x];

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
    let width = contraption[0].len();
    let height = contraption.len();

    directions
        .iter()
        .map(|&direction| GridState { direction, ..state })
        .filter(|state| state.check_bounds(width, height))
        .map(|state| GridState {
            coords: (state.coords + state.direction).unwrap(),
            ..state
        })
        .collect()
}

struct Day16;
const DAY: Day16 = Day16;
impl Day<usize> for Day16 {
    fn part_1(&self, input: &str) -> usize {
        let contraption = Contraption::parse(input);
        let initial = GridState::new(0, 0, Direction::East);
        contraption.get_energized_count(&initial)
    }

    fn part_2(&self, input: &str) -> usize {
        let contraption = Contraption::parse(input);

        let width = contraption.layout[0].len() - 1;
        let height = contraption.layout.len() - 1;

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
            .map(|initial| contraption.get_energized_count(initial))
            .max()
            .unwrap()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
