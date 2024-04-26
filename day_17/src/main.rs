use library::grid::{Direction, GridState};

#[derive(Debug, Clone, Copy, PartialEq)]
struct CrucibleState {
    grid: GridState,
    run: usize,
    weight: usize,
}

// take current state and return valid states for next iteration
fn crucible_move(grid: &Vec<Vec<usize>>, state: CrucibleState) -> Vec<CrucibleState> {
    let directions: Vec<Direction> = vec![
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .filter(|&d| !(d == state.grid.direction.inverse()))
    .collect();

    // println!("directions: {:?}", directions);

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
        // filter out direction if it results in a strait line for over 3 in a row
        .filter(|new_state| new_state.run < 3)
        //  check if the result is in bounds
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
        .map(|new_state| CrucibleState {
            weight: new_state.weight + grid[new_state.grid.y][new_state.grid.x],
            ..new_state
        })
        .collect()
}

struct VisitState {
    direction: Direction,
    weight: usize,
    run: usize,
}
struct Visited {
    grid: Vec<Vec<Vec<VisitState>>>,
}
impl Visited {
    fn get_weight(&self, state: CrucibleState) -> usize {
        let visit_state = &self.grid[state.grid.y][state.grid.x];
        let res = visit_state
            .iter()
            .find(|&visit| (visit.direction == state.grid.direction) && (visit.run == state.run));

        match res {
            Some(a) => a.weight,
            None => usize::MAX,
        }
    }
    fn set_weight(&mut self, state: CrucibleState) {
        let visit_state = &mut self.grid[state.grid.y][state.grid.x];

        let visit_find = visit_state
            .iter_mut()
            .find(|visit| (visit.direction == state.grid.direction) && (visit.run == state.run));

        if visit_find.is_some() {
            let visit = visit_find.unwrap();
            *visit = VisitState {
                direction: state.grid.direction,
                weight: state.weight,
                run: state.run,
            };
        } else {
            visit_state.push(VisitState {
                direction: state.grid.direction,
                weight: state.weight,
                run: state.run,
            });
        }
    }
}

fn get_lowest_heat_loss(grid: &Vec<Vec<usize>>, initial: &CrucibleState) -> usize {
    // bounds check
    let width = grid[0].len();
    let height = grid.len();

    let mut states: Vec<CrucibleState> = vec![CrucibleState {
        weight: grid[initial.grid.y][initial.grid.x],
        ..*initial
    }];
    println!("Initital: {:?}", states);

    let mut visited = Visited {
        grid: grid
            .iter()
            .map(|a| a.iter().map(|_| Vec::new()).collect())
            .collect(),
    };

    // only check the lowest for each iteration
    while !states.is_empty() {
        states.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
        // println!("States {:?}", states);

        let state = states.pop().unwrap();
        visited.set_weight(state);

        let new_states = crucible_move(grid, state);
        // println!("New States {:?}", new_states);


        for s in new_states {
            if visited.get_weight(s) > s.weight {
                states.push(s);
            }
        }
    }

    for (y, line) in visited.grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let weight = &visited.grid[y][x];
            for v_state in weight.iter() {
                match v_state.direction {
                    Direction::North => print!("N"),
                    Direction::East => print!("E"),
                    Direction::South => print!("S"),
                    Direction::West => print!("W"),
                }
                if v_state.weight == usize::MAX {
                    print!(".");
                } else {
                    print!("{} ", v_state.weight);
                }
            }

            print!(",");
        }
        println!("");
    }

    visited
        .grid
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .min_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
        .unwrap()
        .weight
}

fn main() {
    let input = include_str!("../example.txt");

    let crucible: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    for line in input.lines() {
        println!("{line}");
    }

    let initial = CrucibleState {
        grid: GridState {
            direction: Direction::East,
            x: 0,
            y: 0,
        },
        run: 0,
        weight: 0,
    };

    let test = get_lowest_heat_loss(&crucible, &initial);
    println!("test {test}");
}
