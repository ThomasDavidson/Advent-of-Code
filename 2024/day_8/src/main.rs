use std::{collections::HashMap, time::Instant};

use itertools::Itertools;
use library::grid::{find_in_coord, UVec2};
type Coord = UVec2<usize>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Tile {
    Floor,
    Antenna(char),
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            c => Self::Antenna(c),
        }
    }
}

struct Roof {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Roof {
    fn from_input(input: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            width,
            height,
        }
    }
    fn get_antennas(&self) -> Vec<&Tile> {
        let mut tiles: Vec<&Tile> = self.tiles.iter().flatten().collect();
        tiles.sort();
        tiles.dedup();

        tiles
    }

    fn get_antenna_coords(&self, find_tile: &Tile) -> Vec<Coord> {
        // let tiles: Vec<(Coord, &Tile)> = self
        //     .tiles
        //     .iter()
        //     .enumerate()
        //     .map(|(y, line)| {
        //         line.iter()
        //             .enumerate()
        //             .map(|(x, tile)| (Coord::new(x, y), tile))
        //             .collect::<Vec<(Coord, &Tile)>>()
        //     })
        //     .flatten()
        //     .collect();

        // tiles
        //     .iter()
        //     .filter(|(_, tile)| tile == &find_tile)
        //     .map(|(coord, _)| *coord)
        //     .collect()
        find_in_vec2(&self.tiles, find_tile)
    }
}

fn calculate_annode_part_1(
    coord1: &Coord,
    coord2: &Coord,
    max_x: usize,
    max_y: usize,
) -> Option<Coord> {
    let diff_x = coord1.x as i64 - coord2.x as i64;
    let diff_y = coord1.y as i64 - coord2.y as i64;

    let annode_y: usize = (coord1.y as i64 + diff_y).try_into().ok()?;
    let annode_x: usize = (coord1.x as i64 + diff_x).try_into().ok()?;

    let annode_coord = Coord::new(annode_x, annode_y);

    if annode_coord.check_bounds(max_x, max_y) {
        return None;
    }

    Some(annode_coord)
}

fn part_1(input: &str) -> u64 {
    let roof = Roof::from_input(input);

    let antennas = roof.get_antennas();

    let mut annodes: HashMap<Coord, bool> = HashMap::new();

    for antenna in antennas.iter().filter(|t| t != &&&Tile::Floor) {
        let coords = roof.get_antenna_coords(antenna);

        for comb_coords in coords.iter().combinations(2) {
            let coord1 = comb_coords[0];
            let coord2 = comb_coords[1];

            match calculate_annode_part_1(coord1, coord2, roof.width, roof.height) {
                Some(coord) => {
                    annodes.insert(coord, true);
                }
                None => (),
            }
            match calculate_annode_part_1(coord2, coord1, roof.width, roof.height) {
                Some(coord) => {
                    annodes.insert(coord, true);
                }
                None => (),
            }
        }
    }

    annodes.keys().len() as u64
}

fn offset_coord(coord: &Coord, (offset_x, offset_y): (i64, i64)) -> Option<Coord> {
    let x: usize = (coord.x as i64 - offset_x).try_into().ok()?;
    let y: usize = (coord.y as i64 - offset_y).try_into().ok()?;

    Some(Coord::new(x, y))
}

fn calculate_annode_part_2(
    tower: &Coord,
    resonator: &Coord,
    max_x: usize,
    max_y: usize,
) -> Vec<Coord> {
    let diff_x = tower.x as i64 - resonator.x as i64;
    let diff_y = tower.y as i64 - resonator.y as i64;

    if diff_x == 0 || diff_y == 0 {
        panic!();
    }

    let mut annodes: Vec<Coord> = vec![*tower, *resonator];
    let mut check_coord = *tower;

    loop {
        let Some(annode) = offset_coord(&check_coord, (diff_x, diff_y)) else {
            break;
        };
        if annode.check_bounds(max_x, max_y) {
            break;
        }
        check_coord = annode;
        annodes.push(annode);
    }

    annodes
}

fn part_2(input: &str) -> u64 {
    let roof = Roof::from_input(input);

    let antennas = roof.get_antennas();

    let mut annodes: HashMap<Coord, bool> = HashMap::new();

    for antenna in antennas.iter().filter(|t| t != &&&Tile::Floor) {
        let coords = roof.get_antenna_coords(antenna);

        for comb_coords in coords.iter().combinations(2) {
            let coord1 = comb_coords[0];
            let coord2 = comb_coords[1];

            for annode in calculate_annode_part_2(coord1, coord2, roof.width, roof.height) {
                annodes.insert(annode, true);
            }

            for annode in calculate_annode_part_2(coord2, coord1, roof.width, roof.height) {
                annodes.insert(annode, true);
            }
        }
    }

    annodes.keys().len() as u64
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
