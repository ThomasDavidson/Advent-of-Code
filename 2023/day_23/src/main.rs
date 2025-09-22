use colored::Colorize;
use core::{fmt, str};
use library::grid::{Coord, Direction};
use library::input::{Day, InputType};
use std::{collections::HashMap, fmt::Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect();

        Self { grid }
    }
    fn get_start(&self) -> Coord {
        for (x, t) in self.grid[0].iter().enumerate() {
            if *t == Tile::Path {
                return Coord::new(x, 0);
            }
        }
        panic!("Start cannot be found");
    }
    fn get_tile(&self, coords: &Coord) -> &Tile {
        &self.grid[coords.y][coords.x]
    }
    fn adjacent_tiles(&self, coords: &Coord, part_1: bool) -> Vec<Coord> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let tile = self.get_tile(coords);

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
    fn adjacent_nodes(&self, coords: &Coord) -> Vec<(Coord, usize)> {
        let mut adjacent_nodes = Vec::new();

        let height = self.grid.len();
        let initlal = Hiker::new(*coords);

        let mut hikers: Vec<Hiker> = vec![initlal];
        while let Some(hiker) = hikers.pop() {
            if (hiker.coords.y + 1 == height || hiker.coords.y == 0) && *coords != hiker.coords {
                let node = (hiker.coords, hiker.score);
                adjacent_nodes.push(node);
                continue;
                // don't continue after finding node
            }
            let mut next = hiker.hike(self, false);

            if next.len() >= 2 && *coords != hiker.coords {
                let node = (hiker.coords, hiker.score);
                adjacent_nodes.push(node);
                continue;
                // don't continue after finding node
            }
            hikers.append(&mut next);
        }

        adjacent_nodes
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

#[derive(Debug)]
struct NodeMap {
    nodes: HashMap<Coord, Vec<(Coord, usize)>>,
}
impl NodeMap {
    fn from_forest(forest: &Forest) -> Self {
        let start = forest.get_start();

        let mut node_map = NodeMap {
            nodes: HashMap::new(),
        };

        let mut nodes = vec![start];

        while let Some(node) = nodes.pop() {
            let next_nodes = forest.adjacent_nodes(&node);

            node_map.nodes.insert(node, next_nodes.clone());

            for next_node in next_nodes {
                if !node_map.nodes.contains_key(&next_node.0) {
                    nodes.push(next_node.0);
                }
            }
        }
        node_map
    }
    fn traverse_node(&self, hiker: &Hiker) -> Vec<Hiker> {
        let next_nodes = self.nodes.get(&hiker.coords).unwrap();
        // todo filter previous nodes
        next_nodes
            .iter()
            .filter(|(next_node, _weight)| !hiker.previously_visited(next_node))
            .map(|(next_node, weight)| {
                let mut new_previous = hiker.previous.clone();
                new_previous.push(*next_node);

                Hiker {
                    coords: *next_node,
                    previous: new_previous,
                    score: hiker.score + weight,
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Hiker {
    coords: Coord,
    previous: Vec<Coord>,
    score: usize,
}
impl Hiker {
    fn new(start: Coord) -> Self {
        Self {
            coords: start,
            previous: vec![start],
            score: 0,
        }
    }

    fn previously_visited(&self, coord: &Coord) -> bool {
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
                new_previous.push(*coords);

                Self {
                    coords: *coords,
                    previous: new_previous,
                    score: self.score + 1,
                }
            })
            .collect()
    }

    fn _print_path(&self, forest: &Forest) {
        for (y, line) in forest.grid.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                let coord = Coord { x, y };

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
}

#[derive(Clone)]
struct Day23;
const DAY: Day23 = Day23;
impl Day<usize> for Day23 {
    fn part_1(&self, input: &str) -> usize {
        let forest = Forest::from_str(input);
        let height = forest.grid.len();
        let start = forest.get_start();

        let initial = Hiker::new(start);

        let mut hikers: Vec<Hiker> = vec![initial];

        let mut longest_hike_len: usize = 0;
        let mut longest_hike: Option<Hiker> = None;

        while let Some(hiker) = hikers.pop() {
            if hiker.coords.y + 1 == height && longest_hike_len < hiker.score {
                longest_hike_len = hiker.score;
                longest_hike = Some(hiker.clone());
            }
            let mut next = hiker.hike(&forest, true);
            hikers.append(&mut next);
        }

        longest_hike.unwrap().score
    }
    fn part_2(&mut self, input: &str) -> usize {
        let forest = Forest::from_str(input);
        let node_map = NodeMap::from_forest(&forest);

        let height = forest.grid.len();
        let start = forest.get_start();

        let initial = Hiker::new(start);

        let mut hikers: Vec<Hiker> = vec![initial];

        let mut longest_hike_len: usize = 0;
        let mut longest_hike: Option<Hiker> = None;

        while let Some(hiker) = hikers.pop() {
            if hiker.coords.y + 1 == height && longest_hike_len < hiker.score {
                longest_hike_len = hiker.score;
                longest_hike = Some(hiker.clone());
            }
            let mut next = node_map.traverse_node(&hiker);
            hikers.append(&mut next);
        }

        longest_hike.unwrap().score
    }
}

fn main() -> std::io::Result<()> {
    DAY.clone().run(InputType::UserInput)
}
