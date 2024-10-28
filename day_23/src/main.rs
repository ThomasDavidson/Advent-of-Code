use colored::Colorize;
use core::{fmt, str};
use library::grid::{Coords, Direction};
use std::{fmt::Formatter, time::Instant};

#[derive(PartialEq)]
enum Tile {
    Slope(Direction),
    Path,
    Forest,
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Tile::Slope(Direction::East),
            '<' => Tile::Slope(Direction::West),
            '^' => Tile::Slope(Direction::North),
            'v' => Tile::Slope(Direction::South),
            '.' => Tile::Path,
            '#' => Tile::Forest,
            c => panic!("Invalid Tile: {c}"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Slope(Direction::East) => '>',
            Self::Slope(Direction::West) => '<',
            Self::Slope(Direction::North) => '^',
            Self::Slope(Direction::South) => 'v',
            Self::Slope(Direction::None) => panic!(),
            Self::Path => '.',
            Self::Forest => '#',
        }
    }
    fn to_directions(&self) -> Vec<Direction> {
        match self {
            Self::Slope(d) => vec![*d],
            Self::Path => Direction::MOVE.to_vec(),
            Self::Forest => panic!("Cannot travel from forest"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

struct Forest {
    grid: Vec<Vec<Tile>>,
}
impl Forest {
    fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().map(|c| Tile::from_char(c)).collect())
            .collect();

        Self { grid }
    }
    fn get_start(&self) -> Coords<usize> {
        for (x, t) in self.grid[0].iter().enumerate() {
            if *t == Tile::Path {
                return Coords::new(x as usize, 0);
            }
        }
        panic!("Start cannot be found");
    }
    fn get_tile(&self, coords: &Coords<usize>) -> &Tile {
        &self.grid[coords.y][coords.x]
    }
    fn adjacent_tiles(&self, coords: &Coords<usize>, part_1: bool) -> Vec<Coords<usize>> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let tile = self.get_tile(&coords);

        if part_1 {
        tile.to_directions()
        } else {
            Direction::MOVE.to_vec()
        }
        .into_iter()
        .filter_map(|d| (*coords + d).ok())
            .filter(|coords| !coords.check_bounds(width, height))
            .collect()
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.grid.iter() {
            for t in line.iter() {
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Hiker {
    coords: Coords<usize>,
    previous: Vec<Coords<usize>>,
}
impl Hiker {
    fn new(start: Coords<usize>) -> Self {
        Self {
            coords: start,
            previous: Vec::new(),
        }
    }

    fn previously_visited(&self, coord: &Coords<usize>) -> bool {
        self.previous.contains(coord)
    }

    fn hike(&self, forest: &Forest, part_1: bool) -> Vec<Self> {
        forest
            .adjacent_tiles(&self.coords, part_1)
            .iter()
            .filter(|coords| *forest.get_tile(coords) != Tile::Forest)
            .filter(|coords| !self.previously_visited(coords))
            .map(|coords| {
                let mut new_previous = self.previous.clone();
                new_previous.push(coords.clone());

                let hiker = Self {
                    coords: *coords,
                    previous: new_previous,
                    ..self.clone()
                };

                hiker
            })
            .collect()
    }

    fn print_path(&self, forest: &Forest) {
        for (y, line) in forest.grid.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                let coord = Coords { x, y };

                if self.previously_visited(&coord) {
                    let s = format!("{t}").green();
                    print!("{s}");
                } else {
                    print!("{t}");
                }
            }
            println!();
        }
    }
    fn get_hike_length(&self) -> usize {
        self.previous.len()
    }
}

fn part_1(input: &str) -> usize {
    let forest = Forest::from_str(input);
    let height = forest.grid.len();
    let start = forest.get_start();

    let initlal = Hiker::new(start);

    let mut hikers: Vec<Hiker> = vec![initlal];

    let mut longest_hike_len: usize = 0;

    while let Some(hiker) = hikers.pop() {
        if hiker.coords.y + 1 == height {
            longest_hike_len = longest_hike_len.max(hiker.get_hike_length());
        }
        let mut next = hiker.hike(&forest, true);
        hikers.append(&mut next);
    }

    longest_hike_len
}

fn main() {
    let input = include_str!("../input.txt");
    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);
}
