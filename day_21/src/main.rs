use library::grid::{Direction, GridState};
use crate::Tile::{GardenPlot, Rocks, Start};

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Start,
    GardenPlot,
    Rocks,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            'S' => Start,
            '.' => GardenPlot,
            '#' => Rocks,
            a => panic!("Invalid tile map: {}", a),
        }
    }
}

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<Tile>>,
}

impl Garden {
    fn from_string(input: &str) -> Self {
        let grid = input.lines().map(|line|
            line.chars().map(|c| Tile::from_char(c)).collect()
        ).collect();

        Self {
            grid,
        }
    }
    fn find_start(&self) -> (usize, usize) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if *tile == Start {
                    return (x, y);
                }
            }
        }
        panic!("Cannot find start")
    }
}

fn main() {
    let input = include_str!("../example.txt");

    let garden = Garden::from_string(input);

    let (x, y) = garden.find_start();
    println!("Start: {}, {}", x, y);
    let (width, height) = (garden.grid[0].len(), garden.grid.len());

    let directions = Direction::ALL.to_vec();

    for direction in directions {
        let (x_offset, y_offset) = direction.get_translation();
        let (next_x, next_y) = ((x as i16 + x_offset) as usize, (y as i16 + y_offset) as usize);

        let grid_state = GridState {
            direction,
            x: next_x,
            y: next_y,
        };

        if !grid_state.check_bounds(width, height) {
            continue;
        }
        let next_tile = &garden.grid[next_x][next_y];

        if *next_tile == Rocks {
            continue;
        }
        println!("Next: {}, {}: {:?}", grid_state.x, grid_state.y, next_tile);
    }
}
