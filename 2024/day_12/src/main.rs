use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    time::Instant,
};

use library::grid::{Direction, UVec2};
type Coord = UVec2<usize>;

#[derive(Debug)]
struct GardenPlot {
    area: Vec<Coord>,
    plant_type: char,
}
impl GardenPlot {
    fn init(plant_type: char) -> Self {
        Self {
            area: Vec::new(),
            plant_type,
        }
    }
    fn get_area(&self) -> u64 {
        self.area.iter().count() as u64
    }
    fn get_perimeter(&self) -> u64 {
        let mut adjacent_sides = 0;

        for coord in &self.area {
            for dir in Direction::MOVE {
                let adj = match *coord + dir {
                    Err(_) => continue,
                    Ok(adj) => adj,
                };
                if self.area.contains(&adj) {
                    adjacent_sides += 1;
                }
            }
        }

        self.get_area() * 4 - adjacent_sides
    }
    fn walls(&self) -> HashMap<Coord, HashSet<Direction>> {
        let mut walls: HashMap<Coord, HashSet<Direction>> = HashMap::new();

        for coord in &self.area {
            let mut wall = HashSet::new();
            for dir in Direction::MOVE {
                let adj = (*coord + dir).ok();

                if let Some(adj) = adj {
                    if self.area.contains(&adj) {
                        continue;
                    }
                }
                wall.insert(dir);
            }
            walls.insert(*coord, wall);
        }
        walls
    }
    fn coord_range(&self) -> (Range<usize>, Range<usize>) {
        let minx = self.area.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let maxx = self.area.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let miny = self.area.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        let maxy = self.area.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

        (minx..(maxx + 2), miny..(maxy + 2))
    }
    fn add_to_plot(&mut self, coord: Coord) {
        self.area.push(coord);
    }
}

#[derive(Debug)]
struct Garden {
    plots: Vec<GardenPlot>,
}

impl Garden {
    fn from_input(input: &str) -> Self {
        let mut plots: Vec<GardenPlot> = Vec::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let width = grid[0].len();
        let height = grid.len();

        let mut next_coords = Vec::new();

        for (y, line) in grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let coord = Coord::new(x, y);
                if visited.contains(&coord) {
                    continue;
                }
                let curr_plot_type = c;
                let mut plot = GardenPlot::init(*curr_plot_type);

                next_coords.push(coord);
                while let Some(next_coord) = next_coords.pop() {
                    if visited.contains(&next_coord) {
                        continue;
                    }
                    if &grid[next_coord.y][next_coord.x] != curr_plot_type {
                        continue;
                    }
                    plot.add_to_plot(next_coord);
                    visited.insert(next_coord);

                    for dir in Direction::MOVE {
                        let adj = match next_coord + dir {
                            Err(_) => continue,
                            Ok(adj) => adj,
                        };
                        if adj.check_bounds(width, height) {
                            continue;
                        }
                        next_coords.push(adj);
                    }
                }
                plots.push(plot);
            }
        }

        Self { plots }
    }
}

fn part_1(input: &str) -> u64 {
    let garden = Garden::from_input(input);

    let mut part_1_answer = 0;

    for plot in &garden.plots {
        part_1_answer += plot.get_area() * plot.get_perimeter();
    }
    part_1_answer
}

fn part_2(input: &str) -> u64 {
    let garden = Garden::from_input(input);

    let mut part_2_answer = 0;

    for plot in &garden.plots {
        let coord_walls = plot.walls();

        let (rangex, rangey) = plot.coord_range();
        let mut num_walls = 0;

        for dir in [Direction::North, Direction::South] {
            let mut len = 0;
            for y in rangey.clone() {
                for x in rangex.clone() {
                    let coord = Coord::new(x, y);
                    let Some(walls) = coord_walls.get(&coord) else {
                        len = 0;
                        continue;
                    };
                    if walls.contains(&dir) {
                        len += 1;
                    } else {
                        len = 0;
                        continue;
                    }

                    if len == 1 {
                        num_walls += 1;
                    }
                }
            }
        }

        for dir in [Direction::East, Direction::West] {
            let mut len = 0;
            for x in rangex.clone() {
                for y in rangey.clone() {
                    let coord = Coord::new(x, y);
                    let Some(walls) = coord_walls.get(&coord) else {
                        len = 0;
                        continue;
                    };
                    if walls.contains(&dir) {
                        len += 1;
                    } else {
                        len = 0;
                        continue;
                    }

                    if len == 1 {
                        num_walls += 1;
                    }
                }
            }
        }

        part_2_answer += plot.get_area() * num_walls;
    }
    part_2_answer
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}
