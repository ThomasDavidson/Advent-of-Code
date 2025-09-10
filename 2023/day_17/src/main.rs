use library::grid::{Direction, DirectionFilter, GridState};
use library::input::{Day, InputType};
use std::{fmt::Debug, str::FromStr};

fn parse_line<T: FromStr>(line: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    line.chars()
        .map(|c| c.to_string().parse::<T>().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CrucibleState {
    grid: GridState,
    run: usize,
    weight: usize,
}
type NextDirection = dyn Fn(CrucibleState) -> Vec<Direction>;

// take current state and return valid states for next iteration
fn crucible_move(
    grid: &[Vec<usize>],
    state: CrucibleState,
    next_direction: &NextDirection,
) -> Vec<CrucibleState> {
    let directions: Vec<Direction> = next_direction(state);

    // bounds check
    let width = grid[0].len();
    let height = grid.len();

    directions
        .iter()
        .map(|&direction| CrucibleState {
            // set run to zero if not same direction
            grid: GridState {
                direction,
                ..state.grid
            },
            run: if direction == state.grid.direction {
                state.run + 1
            } else {
                0
            },
            ..state
        })
        .filter(|new_state| new_state.grid.check_bounds(width, height))
        .map(|new_state| CrucibleState {
            grid: (new_state.grid + new_state.grid.direction).unwrap(),
            ..new_state
        })
        // Calculate weight
        .map(|new_state| {
            // stopping doesn't add any weight
            let new_weight = match new_state.grid.direction {
                Direction::None => new_state.weight,
                _ => new_state.weight + grid[new_state.grid.coords.y][new_state.grid.coords.x],
            };
            CrucibleState {
                weight: new_weight,
                ..new_state
            }
        })
        .collect()
}
struct VisitStates {
    states: Vec<VisitState>,
}
impl VisitStates {
    fn set_weight(&mut self, run: usize, weight: usize, direction: Direction) -> bool {
        let visit_find = self.find_mut(direction, run);
        if let Some(visit) = visit_find {
            // if new weight is lower than current weight
            if weight < visit.weight {
                *visit = VisitState { weight, ..*visit };
            } else {
                return false;
            }
        } else {
            self.set(VisitState {
                direction,
                weight,
                run,
            });
        }
        true
    }
    fn find_mut(&mut self, direction: Direction, run: usize) -> Option<&mut VisitState> {
        self.states
            .iter_mut()
            .find(|visit| (visit.direction == direction) && (visit.run == run))
    }
    fn find_stopped(&self) -> Option<&VisitState> {
        self.states
            .iter()
            .find(|state| state.direction == Direction::None)
    }
    fn get_weight(&self, direction: Direction, run: usize) -> Option<&VisitState> {
        self.states
            .iter()
            .find(|visit| (visit.direction == direction) && (visit.run == run))
    }

    fn set(&mut self, new_state: VisitState) {
        self.states.push(new_state);
    }
    fn new() -> Self {
        Self { states: Vec::new() }
    }
}

#[derive(Debug, Clone, Copy)]
struct VisitState {
    direction: Direction,
    weight: usize,
    run: usize,
}
struct Visited {
    grid: Vec<Vec<VisitStates>>,
}
impl Visited {
    fn get_weight(&self, state: &CrucibleState) -> usize {
        let visit_state = &self.grid[state.grid.coords.y][state.grid.coords.x];

        match visit_state.get_weight(state.grid.direction, state.run) {
            Some(a) => a.weight,
            None => usize::MAX,
        }
    }
    fn get_min_weight(&self, x: usize, y: usize) -> usize {
        match self.grid[y][x].find_stopped() {
            Some(a) => a.weight,
            None => usize::MAX,
        }
    }
    fn set_weight(&mut self, state: &CrucibleState) -> bool {
        self.grid[state.grid.coords.y][state.grid.coords.x].set_weight(
            state.run,
            state.weight,
            state.grid.direction,
        )
    }
    fn _print_weights(&self, threshold: usize) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                let weight = &self.grid[y][x];
                for v_state in weight.states.iter() {
                    if v_state.weight <= threshold {
                        match v_state.direction {
                            Direction::North => print!("N"),
                            Direction::East => print!("E"),
                            Direction::South => print!("S"),
                            Direction::West => print!("W"),
                            Direction::None => print!("N"),
                        }
                        if v_state.weight == usize::MAX {
                            print!(".");
                        } else {
                            print!("{} ", v_state.weight);
                        }
                    }
                }

                print!(",");
            }
            println!();
        }
    }
}

fn _print_path(paths: &[GridState], width: usize, height: usize) {
    let mut path_grid: Vec<Vec<char>> = (0..=height)
        .map(|_| (0..=width).map(|_| '.').collect())
        .collect();

    for path in paths {
        path_grid[path.coords.y][path.coords.x] = path.direction.to_char();
    }

    _print_grid(&path_grid);
}

fn _print_grid(grid: &[Vec<char>]) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

fn get_lowest_heat_loss(
    grid: &[Vec<usize>],
    initial: &CrucibleState,
    (goal_x, goal_y): (usize, usize),
    next_direction: &NextDirection,
) -> usize {
    // bounds check
    let mut states: Vec<CrucibleState> = vec![*initial];

    let mut visited = Visited {
        grid: grid
            .iter()
            .map(|a| a.iter().map(|_| VisitStates::new()).collect())
            .collect(),
    };

    while !states.is_empty() {
        if visited.get_min_weight(goal_x, goal_y) == usize::MAX {
            states.sort_by(|a, b| {
                let a_dist = a.grid.coords.x.abs_diff(goal_x) + a.grid.coords.y.abs_diff(goal_y);
                let b_dist = b.grid.coords.x.abs_diff(goal_x) + b.grid.coords.y.abs_diff(goal_y);

                b_dist.partial_cmp(&a_dist).unwrap()
            });
        }

        let state = states.pop().unwrap();
        visited.set_weight(&state);

        let mut new_states = crucible_move(grid, state, next_direction);
        new_states.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        for s in new_states {
            let pos_min_weight = visited.get_weight(&s);
            let goal_min_weight = visited.get_min_weight(goal_x, goal_y);

            let distance_to_goal =
                s.grid.coords.x.abs_diff(goal_x) + s.grid.coords.y.abs_diff(goal_y);
            // compares with other iterations that have visited this node
            // and that have visited the goal with a distance penalty
            if pos_min_weight > s.weight && goal_min_weight > (s.weight + distance_to_goal - 1) {
                states.push(s);
            }
        }
    }

    visited.get_min_weight(goal_x, goal_y)
}

struct Day17;
const DAY: Day17 = Day17;
impl Day<usize> for Day17 {
    fn part_1(&self, input: &str) -> usize {
        let grid: Vec<Vec<usize>> = input.lines().map(parse_line).collect();

        let initial_grid = GridState::new(0, 0, Direction::None);
        let initial: CrucibleState = CrucibleState {
            grid: initial_grid,
            run: 1,
            weight: 0,
        };

        // bounds check
        let width = grid[0].len() - 1;
        let height = grid.len() - 1;

        let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction)
        {
            // stop cannot start again
            (0, Direction::None) => vec![],
            // Starting state
            (1, Direction::None) => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            // all except inverse
            (0..=1, d) => d.next(vec![
                DirectionFilter::Forward,
                DirectionFilter::Turn,
                DirectionFilter::Stop,
            ]),
            // only right or left
            (2, d) => d.next(vec![DirectionFilter::Turn, DirectionFilter::Stop]),
            _ => panic!("Invalid state"),
        };

        get_lowest_heat_loss(&grid, &initial, (width, height), &filter)
    }
    fn part_2(&self, input: &str) -> usize {
        let grid: Vec<Vec<usize>> = input.lines().map(parse_line).collect();

        let initial: CrucibleState = CrucibleState {
            grid: GridState::new(0, 0, Direction::None),
            run: 1,
            weight: 0,
        };

        // bounds check
        let width = (grid[0].len() - 1) as usize;
        let height = (grid.len() - 1) as usize;

        let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction)
        {
            // stop cannot start again
            (0, Direction::None) => vec![],
            // Starting state
            (1, Direction::None) => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            // all except inverse
            (0..=2, d) => d.next(vec![DirectionFilter::Forward]),
            (3..=8, d) => d.next(vec![
                DirectionFilter::Forward,
                DirectionFilter::Turn,
                DirectionFilter::Stop,
            ]),
            // only right or left
            (9, d) => d.next(vec![DirectionFilter::Turn, DirectionFilter::Stop]),
            _ => panic!("Invalid state"),
        };
        get_lowest_heat_loss(&grid, &initial, (width, height), &filter)
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_example_1() {
        let input = include_str!("../example.txt");
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let part_1_answer = part_1(grid.clone());

        assert_eq!(part_1_answer, 102);
    }

    #[test]
    fn check_example_2() {
        let input = include_str!("../example2.txt");
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let part_1_answer = part_2(grid.clone());

        assert_eq!(part_1_answer, 71);
    }
}
