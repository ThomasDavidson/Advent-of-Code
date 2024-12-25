use std::collections::{HashMap, HashSet};

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

        let start = Coord::new(0, 0);

        let mut next_coords = vec![start];
        let mut curr_plot_type = &grid[0][0];
        let mut plot = GardenPlot::init(*curr_plot_type);

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
                next_coords.push(adj);
            }
        }
        plots.push(plot);

        Self { plots }
    }
}

fn main() {
    let input = include_str!("../example2.txt");

    let garden = Garden::from_input(input);

    let mut part_1_answer = 0;

    for plot in &garden.plots {
        part_1_answer += plot.get_area() * plot.get_perimeter();
        println!(
            "{} {} * {} = {}",
            plot.plant_type,
            plot.get_area(),
            plot.get_perimeter(),
            plot.get_area() * plot.get_perimeter()
        )
    }
    println!("part 1 answer: {part_1_answer}");
}
