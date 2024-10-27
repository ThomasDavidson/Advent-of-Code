use core::fmt;
use std::fmt::Formatter;

use library::grid::Direction;

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
            Tile::Slope(Direction::East) => '>',
            Tile::Slope(Direction::West) => '<',
            Tile::Slope(Direction::North) => '^',
            Tile::Slope(Direction::South) => 'v',
            Tile::Slope(Direction::None) => panic!(),
            Tile::Path => '.',
            Tile::Forest => '#',
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

fn main() {
    let input = include_str!("../example.txt");

    let forest = Forest::from_str(input);

    println!("{}", forest);
}
