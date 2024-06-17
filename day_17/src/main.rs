use std::{time::Instant, usize};

use library::grid::{Direction, DirectionFilter, GridState};

#[derive(Debug, Clone, Copy, PartialEq)]
struct CrucibleState {
    grid: GridState,
    run: usize,
    weight: usize,
}
type NextDirection = dyn Fn(CrucibleState) -> Vec<Direction>;

// take current state and return valid states for next iteration
fn crucible_move(
    grid: &Vec<Vec<usize>>,
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
                direction: direction,
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
        .map(|new_state| {
            let (x, y) = new_state.grid.direction.get_translation();
            CrucibleState {
                grid: GridState {
                    x: (new_state.grid.x as i16 + x) as usize,
                    y: (state.grid.y as i16 + y) as usize,
                    ..new_state.grid
                },
                ..new_state
            }
        })
        // Caclulate weight
        .map(|new_state| {
            // stopping doesn't add any weight
            let new_weight = match new_state.grid.direction {
                Direction::None => new_state.weight,
                _ => new_state.weight + grid[new_state.grid.y][new_state.grid.x],
            };
            CrucibleState {
                weight: new_weight,
                ..new_state
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct VisitState {
    direction: Direction,
    weight: usize,
    run: usize,
}
struct Visited {
    grid: Vec<Vec<Vec<VisitState>>>,
}
impl Visited {
    fn get_weight(&self, state: &CrucibleState) -> usize {
        let visit_state = &self.grid[state.grid.y][state.grid.x];
        let res = visit_state
            .iter()
            .find(|&visit| (visit.direction == state.grid.direction) && (visit.run == state.run));

        match res {
            Some(a) => a.weight,
            None => usize::MAX,
        }
    }
    fn get_min_weight(&self, x: usize, y: usize) -> usize {
        let res = self.grid[y][x]
            .iter()
            .find(|state| state.direction == Direction::None);

        match res {
            Some(a) => a.weight,
            None => usize::MAX,
        }
    }
    fn set_weight(&mut self, state: &CrucibleState) -> bool {
        let visit_state = &mut self.grid[state.grid.y][state.grid.x];

        let visit_find = visit_state
            .iter_mut()
            .find(|visit| (visit.direction == state.grid.direction) && (visit.run == state.run));

        if visit_find.is_some() {
            let visit = visit_find.unwrap();
            // if new weight is lower than current weight
            if state.weight < visit.weight {
                *visit = VisitState {
                    weight: state.weight,
                    ..*visit
                };
            } else {
                return false;
            }
        } else {
            visit_state.push(VisitState {
                direction: state.grid.direction,
                weight: state.weight,
                run: state.run,
            });
        }
        true
    }
    fn print_weights(&self, threshold: usize) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                let weight = &self.grid[y][x];
                for v_state in weight.iter() {
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
            println!("");
        }
    }
}

fn print_path(paths: &Vec<GridState>, width: usize, height: usize) {
    let mut path_grid: Vec<Vec<char>> = (0..=height)
        .map(|_| (0..=width).map(|_| '.').collect())
        .collect();

    for path in paths {
        path_grid[path.y][path.x] = path.direction.to_char();
    }

    print_grid(path_grid);
}

fn print_grid(grid: Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!("");
    }
}

fn get_lowest_heat_loss(
    grid: &Vec<Vec<usize>>,
    initial: &CrucibleState,
    (goal_x, goal_y): (usize, usize),
    next_direction: &NextDirection,
) -> usize {
    // bounds check
    let mut states: Vec<CrucibleState> = vec![initial.clone()];

    let mut visited = Visited {
        grid: grid
            .iter()
            .map(|a| a.iter().map(|_| Vec::new()).collect())
            .collect(),
    };

    while !states.is_empty() {
        if visited.get_min_weight(goal_x, goal_y) == usize::MAX {
            states.sort_by(|a, b| {
                let a_dist = a.grid.x.abs_diff(goal_x) + a.grid.y.abs_diff(goal_y);
                let b_dist = b.grid.x.abs_diff(goal_x) + b.grid.y.abs_diff(goal_y);

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

            let distance_to_goal = s.grid.x.abs_diff(goal_x) + s.grid.y.abs_diff(goal_y);
            // compares with other iterations that have visited this node
            // and that have visited the goal with a distance penalty
            if pos_min_weight > s.weight && goal_min_weight > (s.weight + distance_to_goal - 1) {
                states.push(s);
            }
        }
    }

    visited.get_min_weight(goal_x, goal_y)
}

fn part_1(grid: Vec<Vec<usize>>) -> usize {
    let initial_grid = GridState {
        direction: Direction::None,
        x: 0,
        y: 0,
    };
    let initial: CrucibleState = CrucibleState {
        grid: initial_grid,
        run: 1,
        weight: 0,
    };

    // bounds check
    let width = (grid[0].len() - 1) as usize;
    let height = (grid.len() - 1) as usize;

    let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction) {
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

fn part_2(grid: Vec<Vec<usize>>) -> usize {
    let initial: CrucibleState = CrucibleState {
        grid: GridState {
            direction: Direction::None,
            x: 0,
            y: 0,
        },
        run: 1,
        weight: 0,
    };

    // bounds check
    let width = (grid[0].len() - 1) as usize;
    let height = (grid.len() - 1) as usize;

    let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction) {
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

fn main() {
    let input = include_str!("../input.txt");

    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let start: Instant = Instant::now();
    let part_1_answer = part_1(grid.clone());
    let duration = start.elapsed();
    println!("Part 1 anwer: {part_1_answer}, time: {:?}", duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(grid.clone());
    let duration = start.elapsed();
    println!("Part 2 anwer: {part_2_answer}, time: {:?}", duration);
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
