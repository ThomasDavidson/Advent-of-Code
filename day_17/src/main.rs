use library::grid::{Direction, GridState};

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
    .filter(|&d| d == state.grid.direction)
    .collect();

    // bounds check
    let width = grid[0].len();
    let height = grid.len();

    directions
        .iter()
        .map(|&direction| 
            // set run to zero if not same direction
            CrucibleState {
            grid: GridState {
                direction: direction,
                ..state.grid
            },
            run: if direction == state.grid.direction {state.run + 1} else {0},
            ..state
        })
        .filter(|state| state.run <=3)
        .filter(|state| state.grid.check_bounds(width, height))
        .map(|state| {
        let (x, y) = state.grid.direction.get_translation();
        // println!("x {x} y {y} -> {} {}", state.x as i16 + x, state.y as i16 + y);
        CrucibleState {
            grid: GridState {
                x: (state.grid.x as i16 + x) as usize,
                y: (state.grid.y as i16 + y) as usize,
                ..state.grid
            },
            ..state
        }
    })
    .map(|state| 
    CrucibleState{
        weight: state.weight + grid[state.grid.y][state.grid.x],
        ..state
    })
    .collect()
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

    let initial = GridState {
        direction: Direction::East,
        x: 0,
        y: 0,
    };
}
