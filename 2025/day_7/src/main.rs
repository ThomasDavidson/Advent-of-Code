use library::grid;
use library::grid::{Coord, Vec2};
use library::input::{Day, InputType};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

type Offset = Vec2<isize>;

struct TachyonManifold {
    grid: Vec<Vec<Tile>>,
}
impl TachyonManifold {
    fn parse(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line.chars().map(Tile::parse).collect())
                .collect(),
        }
    }

    fn propagate_laser(&mut self) -> usize {
        let height = self.grid.len();
        let width = self.grid[0].len();

        let mut visited: Vec<Vec<usize>> = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|tile| if *tile == Tile::Start { 1 } else { 0 })
                    .collect()
            })
            .collect();

        for y in 0..height {
            for x in 0..width {
                let coord = Coord { x, y };

                let laser = self[coord].laser_property();

                for offset in laser.offsets() {
                    let Ok(next_x) = usize::try_from(coord.x as isize + offset.x) else {
                        continue;
                    };

                    let Ok(next_y) = usize::try_from(coord.y as isize + offset.y) else {
                        continue;
                    };

                    let next = Coord::new(next_x, next_y);

                    if next.check_bounds(height + 1, width + 1) {
                        continue;
                    }

                    match self[next] {
                        Tile::EmptySpace => self[next] = Tile::Laser,
                        Tile::Splitter => self[next] = Tile::ActivatedSplitter,
                        _ => (),
                    }

                    visited[next.y][next.x] += visited[coord.y][coord.x];
                }
            }
        }

        visited.into_iter().last().unwrap().iter().sum()
    }
}
impl fmt::Display for TachyonManifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.grid {
            for tile in line {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Index<Coord> for TachyonManifold {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.grid[index.y][index.x]
    }
}
impl IndexMut<Coord> for TachyonManifold {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.grid[index.y][index.x]
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    EmptySpace,
    Start,
    Splitter,
    Laser,
    ActivatedSplitter,
}
impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::EmptySpace,
            'S' => Self::Start,
            '^' => Self::Splitter,
            c => panic!("Invalid character: {c}"),
        }
    }
    fn laser_property(&self) -> Laser {
        match self {
            Tile::Laser | Tile::Start => Laser::Move,
            Tile::ActivatedSplitter => Laser::Split,
            Tile::EmptySpace | Tile::Splitter => Laser::None,
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Laser => '|',
            Tile::Splitter => '^',
            Tile::Start => 'S',
            Tile::EmptySpace => '.',
            Tile::ActivatedSplitter => 'W',
        };
        write!(f, "{c}")
    }
}

enum Laser {
    Move,
    Split,
    None,
}
impl Laser {
    fn offsets(&self) -> Vec<Offset> {
        match self {
            Self::Move => vec![Offset::new(0, 1)],
            Self::Split => vec![Offset::new(-1, 1), Offset::new(1, 1)],
            Self::None => vec![],
        }
    }
}

struct Day7;
const DAY: Day7 = Day7;
impl Day<u64> for Day7 {
    fn part_1(&self, input: &str) -> u64 {
        let mut tachyon_manifolds = TachyonManifold::parse(input);

        tachyon_manifolds.propagate_laser();

        grid::find_in_coord(&tachyon_manifolds.grid, &Tile::ActivatedSplitter).len() as u64
    }
    fn part_2(&self, input: &str) -> u64 {
        TachyonManifold::parse(input).propagate_laser() as u64
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
