use std::{collections::VecDeque, fmt, time::Instant};
use crate::Tile::{GardenPlot, Rocks, Start};
use library::grid::{Direction, GridState};

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
    fn to_char(&self) -> char {
        match self {
            Start => 'S',
            GardenPlot => '.',
            Rocks => '#',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<Tile>>,
    steps: Vec<Vec<u16>>,
}

impl Garden {
    fn from_string(input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
            .collect();

        // create grid with zero weight
        let steps: Vec<Vec<u16>> = grid
            .iter()
            .map(|l| l.iter().map(|_| u16::MAX).collect())
            .collect();

        Self { grid, steps }
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
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn find_next(
        &mut self,
        x: usize,
        y: usize,
        steps: u16,
        max_steps: u16,
    ) -> VecDeque<(usize, usize, u16)> {
        let width = self.width();
        let height = self.height();

        if self.steps[x][y] <= steps {
            return VecDeque::new();
        }
        self.steps[x][y] = steps;

        if steps == max_steps {
            return VecDeque::new();
        }

        let directions = Direction::MOVE;

        let mut coords: VecDeque<(usize, usize, u16)> = VecDeque::new();

        for direction in directions {
            let grid_state = GridState { direction, x, y };

            if !grid_state.check_bounds(width, height) {
                continue;
            }

            let (x_offset, y_offset) = direction.get_translation();
            let (next_x, next_y) = (
                (x as i16 + x_offset) as usize,
                (y as i16 + y_offset) as usize,
            );

            let next_tile = &self.grid[next_x][next_y];

            if *next_tile == Rocks {
                continue;
            }

            if self.steps[next_x][next_y] < steps {
                continue;
            }

            coords.push_front((next_x, next_y, steps + 1));
        }
        coords
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut garden = Garden::from_string(input);
    println!("height: {} width: {}", garden.height(), garden.width());

    let (start_x, start_y) = garden.find_start();

    let mut coords: VecDeque<(usize, usize, u16)> = VecDeque::from([(start_x, start_y, 0)]);

    let max_step = 200;

    let start: Instant = Instant::now();
    let mut n: u32 = 0;
    while let Some((x, y, steps)) = coords.pop_front() {
        n += 1;
        if n % 10000000 == 0 {
            println!("{n} size: {}", coords.len());
        }
        let mut next = garden.find_next(x, y, steps, max_step);
        coords.append(&mut next);
    }
    let mut part_1_answer = 0;

    for (y, line) in garden.grid.iter().enumerate() {
        for (x, t) in line.iter().enumerate() {
            if garden.steps[y][x] != u16::MAX && garden.steps[y][x] % 2 == 0 {
                part_1_answer += 1;
                // print!("O")
            } else {
                // print!("{}", t);
            }
        }
        // println!();
    }

    let duration = start.elapsed();
    println!("Part 1 answer: {part_1_answer}, time: {:?}", duration);
}
