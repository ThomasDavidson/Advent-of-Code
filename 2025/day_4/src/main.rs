use library::grid::Vec2;
use library::input::{Day, InputType};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;

type Coord = Vec2<usize>;
fn coord_add(
    coord: Coord,
    x_offset: i8,
    y_offset: i8,
    width: usize,
    height: usize,
) -> Option<Coord> {
    let Ok(x): Result<usize, _> = (x_offset as i16 + coord.x as i16).try_into() else {
        return None;
    };

    let Ok(y): Result<usize, _> = (y_offset as i16 + coord.y as i16).try_into() else {
        return None;
    };

    let new_coord = Coord::new(x, y);

    if new_coord.check_bounds(width, height) {
        return None;
    }

    Some(new_coord)
}

#[derive(Clone)]
struct PrintingDepartment {
    grid: Vec<Vec<Floor>>,
}

impl PrintingDepartment {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Floor::parse).collect())
            .collect();
        Self { grid }
    }
    fn get(&self, coord: Coord) -> &Floor {
        &self.grid[coord.y][coord.x]
    }
    fn get_mut(&mut self, coord: Coord) -> &mut Floor {
        &mut self.grid[coord.y][coord.x]
    }

    fn adjacent_paper(&self, coord: Coord) -> u8 {
        const CHECK: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let width = self.grid[0].len();
        let height = self.grid.len();

        let mut num_adjacent_papers = 0;
        for (x_offset, y_offset) in CHECK {
            let Some(new_coord) = coord_add(coord, x_offset, y_offset, width, height) else {
                continue;
            };

            let floor = self.get(new_coord);

            if *floor == Floor::Paper {
                num_adjacent_papers += 1;
            }
        }

        num_adjacent_papers
    }
}
impl fmt::Display for PrintingDepartment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for floor in &self.grid {
            for x in floor {
                write!(f, "{x}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Floor {
    Empty,
    Paper,
}
impl Floor {
    fn parse(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => panic!("Invalid input"),
        }
    }
}
impl fmt::Display for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Floor::Empty => write!(f, "."),
            Floor::Paper => write!(f, "@"),
        }
    }
}

struct Day4;
const DAY: Day4 = Day4;
impl Day<u32> for Day4 {
    fn part_1(&self, input: &str) -> u32 {
        let department = PrintingDepartment::parse(input);

        let mut part_1_answer = 0;

        for (coord, floor) in Coord::enumerate(&department.grid) {
            if floor == Floor::Paper {
                if department.adjacent_paper(coord) >= 4 {
                    continue;
                }
                part_1_answer += 1;
            }
        }

        part_1_answer
    }
    fn part_2(&self, input: &str) -> u32 {
        let mut department = PrintingDepartment::parse(input);

        let mut part_2_answer = 0;

        loop {
            let mut remove_coords: Vec<Coord> = Vec::new();
            for (coord, floor) in Coord::enumerate(&department.grid) {
                if floor == Floor::Paper {
                    if department.adjacent_paper(coord) >= 4 {
                        continue;
                    }
                    remove_coords.push(coord);
                }
            }
            if remove_coords.len() == 0 {
                break;
            }
            part_2_answer += remove_coords.len() as u32;

            for remove_coord in remove_coords {
                let floor = department.get_mut(remove_coord);
                *floor = Floor::Empty
            }
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
