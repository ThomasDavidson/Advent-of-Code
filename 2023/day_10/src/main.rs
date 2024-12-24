use colorize::AnsiColor;
use library::grid::{Direction, UVec2};
use std::{collections::HashSet, time::Instant};

type Coord = UVec2<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    None,
}

impl Tile {
    fn symbol(&self) -> char {
        match self {
            Self::NS => '│', //'|',\

            Self::EW => '─', //'-',\

            Self::NE => '└', //'L',\

            Self::SW => '┐', //'7',\

            Self::NW => '┘', //'J',\

            Self::SE => '┌', //'F',
            Self::None => '.',
        }
    }
    fn from_char(c: &char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::None,
            c => panic!("Invalid {}", c),
        }
    }

    fn pipe_directions(&self) -> [Direction; 2] {
        match self {
            Self::NS => [Direction::North, Direction::South],
            Self::EW => [Direction::East, Direction::West],
            Self::NE => [Direction::North, Direction::East],
            Self::NW => [Direction::North, Direction::West],
            Self::SW => [Direction::South, Direction::West],
            Self::SE => [Direction::East, Direction::South],
            Self::None => [Direction::None, Direction::None],
        }
    }
}

struct HotSprings {
    grid: Vec<Vec<char>>,
}
impl HotSprings {
    fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }
    fn get_char(&self, coord: &Coord) -> Option<&char> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        if coord.check_bounds(width, height) {
            return None;
        }
        return Some(&self.grid[coord.y][coord.x]);
    }

    fn get_tile(&self, coord: &Coord) -> Option<Tile> {
        let Some(c) = self.get_char(coord) else {
            return None;
        };
        Some(Tile::from_char(c))
    }
    fn get_tile_p2(&self, coord: &Coord) -> Option<Tile> {
        let c = match self.get_char(coord) {
            None => return None,
            Some('S') => self.get_start_tile(coord).unwrap(),
            Some(c) => *c,
        };
        Some(Tile::from_char(&c))
    }

    fn get_start_tile(&self, start_coord: &Coord) -> Option<char> {
        let pipe_directions_vec: Vec<Direction> = Direction::MOVE
            .into_iter()
            // check if there is an adjacent tile
            .filter_map(|d| match *start_coord + d {
                Ok(coord) => {
                    // get tile
                    match self.get_tile(&coord) {
                        Some(tile) => Some((d, tile)),
                        None => None,
                    }
                }
                Err(_) => None,
            })
            // reverse directions for origin pipe
            // check if the tile has a pipe pointing twords it
            .filter_map(|(d, adjacent_tile)| {
                let adjacent_pipe_directions = adjacent_tile.pipe_directions();
                if adjacent_tile != Tile::None && adjacent_pipe_directions.contains(&d.inverse()) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        // should have 2 pipe directions
        let pipe_directions: [Direction; 2] = match pipe_directions_vec.try_into() {
            Ok(pd) => pd,
            _ => return None,
        };

        // compare with pipes
        let cmp =
            ['|', '-', 'L', 'J', '7', 'F'].map(|c| (c, Tile::from_char(&c).pipe_directions()));

        let pipe_index = cmp
            .iter()
            .position(|(_c, d)| d.iter().all(|d| pipe_directions.contains(d)))
            .unwrap();

        Some(cmp[pipe_index].0)
    }

    fn debug_visited(&self, visited: &HashSet<Coord>, p: Option<Coord>, n: Option<Coord>) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, _c) in line.iter().enumerate() {
                let coord = Coord::new(x, y);
                let debug = self.get_tile_p2(&coord).unwrap().symbol();

                if Some(coord) == n {
                    print!("{}", format!("{debug}").green());
                } else if Some(coord) == p {
                    print!("{}", format!("{debug}").red());
                } else if visited.contains(&coord) {
                    print!("{}", format!("{debug}").cyan());
                } else {
                    print!("{debug}")
                }
            }
            println!();
        }
    }
    fn debug(&self) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, _c) in line.iter().enumerate() {
                let coord = Coord::new(x, y);
                let debug = self.get_tile_p2(&coord).unwrap().symbol();

                print!("{}", &debug);
            }
            println!();
        }
    }

    fn get_next_tile(
        &self,
        back: Direction,
        current_symbol: char,
        current_location: Coord,
    ) -> Option<(Coord, Direction)> {
        let current_dirs = get_tile_connections(&current_symbol);
        let width = self.grid[0].len();
        let height = self.grid.len();

        for dir in current_dirs {
            // skip if visited previously
            if back == dir {
                continue;
            }
            // skip if out of bounds
            let next_coord = match check_direction(width, height, &current_location, &dir) {
                None => {
                    continue;
                }
                Some(a) => a,
            };

            let next_tile = self.get_char(&next_coord).unwrap();

            let next_dirs = get_tile_connections(&next_tile);

            // switch direction
            let required_dir = dir.inverse();

            // check if the next current_dirs has a pipe facing in this direction
            if !next_dirs.contains(&required_dir) {
                continue;
            }

            // sets to opposite of current_direction
            return Some((next_coord, required_dir));
        }
        return None;
    }
}

fn get_tile_connections(symbol: &char) -> Vec<Direction> {
    match symbol {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        '.' => vec![],
        'S' => Direction::MOVE.to_vec(),
        _ => panic!("above should match all"),
    }
}

fn check_direction(width: usize, height: usize, coord: &Coord, dir: &Direction) -> Option<Coord> {
    // check if out of bounds for each direction
    let next_coord = match *coord + *dir {
        Ok(res) => res,
        Err(_) => return None,
    };

    if next_coord.check_bounds(width, height) {
        return None;
    }

    Some(next_coord)
}

fn get_start(input: &str) -> Option<Coord> {
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if 'S' == c {
                return Some(Coord::new(x, y));
            }
        }
    }
    None
}

fn part_1(input: &str) -> u64 {
    let mut part_1_answer: u64 = 0;

    let hotsprings = HotSprings::from_str(input);
    hotsprings.debug();

    let starting_point = match get_start(input) {
        Some(a) => a,
        None => panic!("No start"),
    };

    let mut current_location = starting_point;
    let mut back: Direction = Direction::None;

    // the number of nodes searched will never be larger than the size of the input
    loop {
        let current_symbol = hotsprings.get_char(&current_location).unwrap();

        part_1_answer += 1;

        let next_tile_res = hotsprings.get_next_tile(back, *current_symbol, current_location);
        match next_tile_res {
            Some(a) => {
                current_location = a.0;
                back = a.1;
            }
            None => panic!("Hit dead end"),
        }
        if current_location == starting_point {
            break;
        }
    }

    part_1_answer / 2
}

fn shoe_string(hotsprings: HotSprings) -> u64 {
    let mut inside: HashSet<Coord> = HashSet::new();
    let height = hotsprings.grid.len();
    let width = hotsprings.grid[0].len();

    let mut prev = Tile::None;
    let mut visited: HashSet<Coord> = HashSet::new();

    for y in 0..height {
        let mut is_inside = false;

        for x in 0..width {
            let coord = Coord::new(x, y);
            let tile = &hotsprings.get_tile_p2(&coord).unwrap();

            if prev == Tile::SE && tile == &Tile::NW {
                // ┌┘
                is_inside = !is_inside;
            } else if prev == Tile::NE && tile == &Tile::SW {
                // └┐
                is_inside = !is_inside;
            } else if tile == &Tile::NS {
                is_inside = !is_inside;
            } else if is_inside && tile == &Tile::None {
                inside.insert(coord);
            }

            if is_inside {
                visited.insert(coord);
            }
            if tile != &Tile::EW {
                prev = tile.clone();
            }
        }
    }

    hotsprings.debug_visited(&visited, None, None);
    inside.iter().len() as u64
}

fn part_2(input: &str) -> u64 {
    let mut visited: HashSet<Coord> = HashSet::new();

    let mut hotsprings = HotSprings::from_str(input);

    let starting_point = match get_start(input) {
        Some(a) => a,
        None => panic!("No start"),
    };

    let mut current_location = starting_point;
    let mut back: Direction = Direction::None;

    // the number of nodes searched will never be larger than the size of the input
    loop {
        visited.insert(current_location);
        let current_symbol = hotsprings.get_char(&current_location).unwrap();

        let next_tile_res = hotsprings.get_next_tile(back, *current_symbol, current_location);
        match next_tile_res {
            Some(a) => {
                current_location = a.0;
                back = a.1;
            }
            None => panic!("Hit dead end"),
        }
        if current_location == starting_point {
            break;
        }
    }

    for (y, line) in hotsprings.grid.iter_mut().enumerate() {
        for (x, c) in line.iter_mut().enumerate() {
            let coord = Coord::new(x, y);
            if !visited.contains(&coord) {
                *c = '.';
            }
        }
    }
    println!();
    hotsprings.debug();

    shoe_string(hotsprings)
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

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn test_example_1() {
        let input = include_str!("../example.txt");
        let result = part_2(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_example_4() {
        let input = include_str!("../example4.txt");
        let result = part_2(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_example_5() {
        let input = include_str!("../example5.txt");
        let result = part_2(input);
        assert_eq!(result, 8);
    }
    #[test]
    fn test_example_6() {
        let input = include_str!("../example6.txt");
        let result = part_2(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_example_7() {
        let input = include_str!("../example7.txt");
        let result = part_2(input);
        assert_eq!(result, 10);
    }
}
