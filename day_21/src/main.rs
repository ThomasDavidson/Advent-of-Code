use crate::Tile::{GardenPlot, Rocks, Start};
use library::grid::{Direction, GridState};
use std::{collections::VecDeque, fmt, time::Instant};

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
    pub const DEFAULT_STEP: u16 = u16::MAX;

    fn from_string(input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
            .collect();

        // create grid with zero weight
        let steps: Vec<Vec<u16>> = grid
            .iter()
            .map(|l| l.iter().map(|_| Garden::DEFAULT_STEP).collect())
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

    fn visited(&self, elf: &Elf) -> bool {
        let (x, y) = elf.coords;
        // checks if spot has been visited before with fewer steps
        self.steps[x][y] <= elf.steps
    }

    fn visit(&mut self, elf: &Elf) {
        let (x, y) = elf.coords;
        self.steps[x][y] = elf.steps;
    }
    fn get_step_coords(&self) -> impl Iterator<Item = &u16> {
        self.steps.iter().flatten()
    }

    fn find_next(&mut self, elf: Elf) -> VecDeque<Elf> {
        let width = self.width();
        let height = self.height();
        let (x, y) = elf.coords;

        if elf.steps == elf.max_steps {
            return VecDeque::new();
        }

        let directions = Direction::MOVE;

        let mut coords = VecDeque::new();

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

            let new_elf = Elf {
                coords: (next_x, next_y),
                steps: elf.steps + 1,
                ..elf
            };

            if self.visited(&new_elf) {
                continue;
            }

            coords.push_front(new_elf);
        }
        coords
    }
}

struct Elf {
    max_steps: u16,
    steps: u16,
    coords: (usize, usize),
}

fn part_1(input: &str) -> i32 {
    let mut garden = Garden::from_string(input);

    let gardener = Elf {
        max_steps: 64,
        steps: 0,
        coords: garden.find_start(),
    };

    let mut gardeners = VecDeque::from([gardener]);

    while let Some(gardener) = gardeners.pop_front() {
        if garden.visited(&gardener) {
            continue;
        }

        garden.visit(&gardener);
        let mut next = garden.find_next(gardener);
        gardeners.append(&mut next);
    }

    garden
        .get_step_coords()
        .map(|step| {
            // check if visited from step count
            if step % 2 == 0 && *step != Garden::DEFAULT_STEP {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);
}
