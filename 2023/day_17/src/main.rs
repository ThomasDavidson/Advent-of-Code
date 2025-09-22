use library::grid::{Direction, GridState, UVec2};
use library::input::{Day, InputType};
use std::ops::Range;
use std::{fmt::Debug, str::FromStr};

mod visited;
use visited::VisitStates;

struct Facility {
    grid: Vec<Vec<usize>>,
    dimensions: (usize, usize),
}
impl Facility {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<usize>> = input.lines().map(parse_line).collect();
        Self {
            dimensions: (grid[0].len(), grid.len()),
            grid,
        }
    }
    fn get_weights(&self, coord: &UVec2<usize>, coord2: &UVec2<usize>) -> usize {
        let y_range = if coord.y == coord2.y {
            coord.y..=coord2.y
        } else if coord.y < coord2.y {
            coord.y..=(coord2.y - 1)
        } else {
            (coord2.y + 1)..=coord.y
        };

        let x_range = if coord.x == coord2.x {
            coord.x..=coord2.x
        } else if coord.x < coord2.x {
            coord.x..=(coord2.x - 1)
        } else {
            (coord2.x + 1)..=coord.x
        };

        let mut sum = 0;
        for y in y_range {
            for x in x_range.clone() {
                sum += self.grid[y][x];
            }
        }
        sum
    }
}
fn parse_line<T: FromStr>(line: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    line.chars()
        .map(|c| c.to_string().parse::<T>().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct CrucibleState {
    grid: GridState,
    run: usize,
    weight: usize,
}
impl CrucibleState {
    const fn score(&self, (goal_x, goal_y): (usize, usize)) -> usize {
        self.distance_score((goal_x, goal_y))
    }
    const fn distance_score(&self, (goal_x, goal_y): (usize, usize)) -> usize {
        self.grid.coords.x.abs_diff(goal_x) + self.grid.coords.y.abs_diff(goal_y)
    }

    fn translate(
        &self,
        facility: &Facility,
        direction: Direction,
        distance: usize,
    ) -> Option<Self> {
        let grid = GridState {
            direction,
            coords: self.grid.coords,
        };
        let add = direction * distance as i32;
        let new_grid = (grid + add).ok()?;

        if !new_grid.check_bounds(facility.dimensions.0, facility.dimensions.1) {
            return None;
        }

        let weight = match new_grid.direction {
            Direction::None => self.weight,
            _ => self.weight + facility.get_weights(&new_grid.coords, &self.grid.coords),
        };

        Some(CrucibleState {
            grid: new_grid,
            run: if direction == self.grid.direction {
                self.run + distance
            } else {
                0
            },
            weight,
        })
    }
}
type NextDirection<'a> = dyn Fn(CrucibleState) -> Vec<(Range<usize>, Direction)>;
struct Crucible<'a> {
    next_direction: &'a NextDirection<'a>,
    max_distance: usize,
}
impl Crucible<'_> {
    // take current state and return valid states for next iteration
    fn crucible_move(&self, facility: &Facility, state: CrucibleState) -> Vec<CrucibleState> {
        (self.next_direction)(state.clone())
            .into_iter()
            .flat_map(|(range, direction)| range.map(move |d| (d, direction)))
            .filter(|(distance, direction)| {
                !(state.run + distance > self.max_distance + 1 && direction != &Direction::None)
            })
            .filter_map(|(distance, direction)| state.translate(facility, direction, distance))
            .collect()
    }
}

struct Visited {
    grid: Vec<Vec<VisitStates>>,
}
impl Visited {
    fn get(&self, x: usize, y: usize) -> &VisitStates {
        &self.grid[y][x]
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut VisitStates {
        &mut self.grid[y][x]
    }

    fn _debug(&self) {
        for y in &self.grid {
            for x in y {
                let w = x.get_stopped();
                let mw = x._iter().min_by(|a, b| a.1.cmp(b.1));
                let (direction, direction_weight) = if let Some(r) = mw {
                    (r.0 .0, *r.1)
                } else {
                    (Direction::None, 0)
                };

                let c = if w == usize::MAX {
                    format!("{} {direction_weight}", direction.to_char())
                } else {
                    format!("{w}")
                };

                print!("{c:<5},");
            }
            println!();
        }
        println!();
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
    facility: &Facility,
    initial: &CrucibleState,
    (goal_x, goal_y): (usize, usize),
    crucible: Crucible,
) -> usize {
    // bounds check
    let mut states: Vec<CrucibleState> = vec![initial.clone()];

    let mut visited = Visited {
        grid: facility
            .grid
            .iter()
            .map(|a| a.iter().map(|_| VisitStates::new()).collect())
            .collect(),
    };

    while !states.is_empty() {
        if visited.get(goal_x, goal_y).get_stopped() == usize::MAX {
            states.sort_by(|a, b| {
                let a_dist = a.distance_score((goal_x, goal_y));
                let b_dist = b.distance_score((goal_x, goal_y));

                b_dist.partial_cmp(&a_dist).unwrap()
            });
        }

        let state = states.pop().unwrap();

        visited
            .get_mut(state.grid.coords.x, state.grid.coords.y)
            .set_weight(state.run, state.weight, state.grid.direction);

        let mut new_states = crucible.crucible_move(facility, state.clone());
        new_states.sort_by(|a, b| {
            a.score((goal_x, goal_y))
                .partial_cmp(&b.score((goal_x, goal_y)))
                .unwrap()
        });

        for s in new_states {
            let pos_min_weight = visited
                .get(s.grid.coords.x, s.grid.coords.y)
                .get_weight(s.grid.direction, s.run);

            let goal_min_weight = visited.get(goal_x, goal_y).get_stopped();

            let distance_to_goal = s.distance_score((goal_x, goal_y));
            // compares with other iterations that have visited this node
            // and that have visited the goal with a distance penalty
            if pos_min_weight > s.weight && goal_min_weight > (s.weight + distance_to_goal - 1) {
                states.push(s);
            }
        }
    }

    visited.get(goal_x, goal_y).get_stopped()
}

struct Day17;
const DAY: Day17 = Day17;
impl Day<usize> for Day17 {
    fn part_1(&self, input: &str) -> usize {
        let facility = Facility::parse(input);
        let (width, height) = facility.dimensions;
        let (width, height) = (width - 1, height - 1);

        let initial_grid = GridState::new(0, 0, Direction::None);
        let initial: CrucibleState = CrucibleState {
            grid: initial_grid,
            run: 1,
            weight: 0,
        };

        const DEF_RANGE: Range<usize> = 1..4;

        let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction)
        {
            // stop cannot start again
            (0, Direction::None) => vec![],
            // Starting state
            (1, Direction::None) => {
                vec![
                    (DEF_RANGE, Direction::East),
                    (DEF_RANGE, Direction::West),
                    (DEF_RANGE, Direction::North),
                    (DEF_RANGE, Direction::South),
                ]
            }
            // all except inverse
            (0..=3, d) => {
                vec![
                    (DEF_RANGE, d.right()),
                    (DEF_RANGE, d.left()),
                    (0..1, Direction::None),
                ]
            }
            _ => panic!("Invalid state"),
        };

        let crucible = Crucible {
            next_direction: filter,
            max_distance: 3,
        };

        get_lowest_heat_loss(&facility, &initial, (width, height), crucible)
    }
    fn part_2(&self, input: &str) -> usize {
        let facility = Facility::parse(input);
        // bounds check
        let (width, height) = facility.dimensions;
        let (width, height) = (width - 1, height - 1);

        let initial_grid = GridState::new(0, 0, Direction::None);
        let initial: CrucibleState = CrucibleState {
            grid: initial_grid,
            run: 1,
            weight: 0,
        };

        const DEF_RANGE: Range<usize> = 4..11;

        let filter: &NextDirection = &|state: CrucibleState| match (state.run, state.grid.direction)
        {
            // stop cannot start again
            (0, Direction::None) => vec![],
            // Starting state
            (1, Direction::None) => {
                vec![
                    (DEF_RANGE, Direction::East),
                    (DEF_RANGE, Direction::West),
                    (DEF_RANGE, Direction::North),
                    (DEF_RANGE, Direction::South),
                ]
            }
            (0..=9, d) => {
                vec![
                    (DEF_RANGE, d.right()),
                    (DEF_RANGE, d.left()),
                    (0..1, Direction::None),
                ]
            }

            _ => panic!("Invalid state {state:?}"),
        };
        let crucible = Crucible {
            next_direction: filter,
            max_distance: 10,
        };
        get_lowest_heat_loss(&facility, &initial, (width, height), crucible)
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use crate::{Day17, Facility};
    use library::grid::UVec2;
    use library::input::Day;
    const DAY: Day17 = Day17;
    #[test]
    fn check_example_1() {
        let input = include_str!("../example.txt");
        assert_eq!(DAY.part_1(input), 102);
    }

    #[test]
    fn check_example_2() {
        let input = include_str!("../example2.txt");
        assert_eq!(DAY.part_2(input), 71);
    }
    #[test]
    fn check_example_3() {
        let input = include_str!("../example3.txt");
        assert_eq!(DAY.part_2(input), 59);
    }
    #[test]
    fn check_example_4() {
        let input = include_str!("../example4.txt");
        assert_eq!(DAY.part_2(input), 44);
    }
    #[test]
    fn check_example_5() {
        let input = include_str!("../example5.txt");
        assert_eq!(DAY.part_2(input), 8);
    }
    #[test]
    fn check_weight() {
        let input = include_str!("../example.txt");

        let facility = Facility::parse(input);

        let tests: Vec<(UVec2<_>, UVec2<_>, usize)> = vec![
            (UVec2::new(0, 0), UVec2::new(0, 1), 2),
            (UVec2::new(0, 0), UVec2::new(1, 0), 2),
            (UVec2::new(0, 1), UVec2::new(0, 0), 3),
            (UVec2::new(1, 0), UVec2::new(0, 0), 4),
            (UVec2::new(0, 0), UVec2::new(0, 2), 5),
            (UVec2::new(0, 0), UVec2::new(2, 0), 6),
            (UVec2::new(0, 2), UVec2::new(0, 0), 6),
            (UVec2::new(2, 0), UVec2::new(0, 0), 5),
        ];

        for (coord, coord2, expected) in tests {
            let score = facility.get_weights(&coord, &coord2);
            assert_eq!(score, expected);
        }
    }
}
