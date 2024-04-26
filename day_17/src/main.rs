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
        .filter(|state| state.run <= 3)
        //  check if the result is in bounds
        .filter(|state| state.grid.check_bounds(width, height))
        .map(|state| {
            let (x, y) = state.grid.direction.get_translation();
            CrucibleState {
                grid: GridState {
                    x: (state.grid.x as i16 + x) as usize,
                    y: (state.grid.y as i16 + y) as usize,
                    ..state.grid
                },
                ..state
            }
        })
        // Caclulate weight
        .map(|state| CrucibleState {
            weight: state.weight + grid[state.grid.y][state.grid.x],
            ..state
        })
        .collect()
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

    let mut visited: Vec<Vec<[(bool, usize); 4]>> = grid
        .iter()
        .map(|a| a.iter().map(|_| [(false, usize::MAX); 4]).collect())
        .collect();


    // visited[initial.grid.y][initial.grid.x][Direction::East as usize].0 = true;
    



    // only check the lowest for each iteration
    while !states.is_empty() {
        // println!("Before Sort {:?}", states);
        states.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        let state = states.pop().unwrap();
        visited[state.grid.y][state.grid.x][state.grid.direction as usize].0 = true;
        visited[state.grid.y][state.grid.x][state.grid.direction as usize].1 = state.weight;

        let new_states = crucible_move(grid, state.clone());

        for s in new_states {
            let tile_weight = visited[s.grid.y][s.grid.x][s.grid.direction as usize].1;
            if tile_weight > s.weight {
                states.push(s);
            }
        }
    }
    for (y, line) in visited.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let weight = visited[y][x];
            for (i, &(_, c_weight)) in weight.iter().enumerate() {
                match i {
                    0 => print!("N"),
                    1 => print!("E"),
                    2 => print!("S"),
                    3 => print!("W"),
                    _ => panic!(""),
                }
                if c_weight == usize::MAX {
                    print!(".");
                } else {
                    print!("{c_weight} ");
                }
            }

            print!(",");
        }
        println!("");
    }
    println!("Sort {:?}", states);

    visited
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .min_by(|&&a, &&b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .1
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
