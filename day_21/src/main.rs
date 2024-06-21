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

    println!("Start: {:?}", garden.find_start());
}
