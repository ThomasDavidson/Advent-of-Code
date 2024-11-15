use colorize::AnsiColor;
use library::grid::{Direction, UVec2};
use std::time::Instant;

type Coord = UVec2<usize>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Traverse {
    Direction(Direction),
    // Direction, Side of wall
    Side(Direction, Direction),
}
impl Traverse {
    const WEST_WEST: Self = Self::Side(Direction::West, Direction::West);
    const WEST_EAST: Self = Self::Side(Direction::West, Direction::East);
    const WEST_SOUTH: Self = Self::Side(Direction::West, Direction::South);
    const WEST_NORTH: Self = Self::Side(Direction::West, Direction::North);

    const NORTH_WEST: Self = Self::Side(Direction::North, Direction::West);
    const NORTH_EAST: Self = Self::Side(Direction::North, Direction::East);
    const NORTH_SOUTH: Self = Self::Side(Direction::North, Direction::South);
    const NORTH_NORTH: Self = Self::Side(Direction::North, Direction::North);

    const SOUTH_WEST: Self = Self::Side(Direction::South, Direction::West);
    const SOUTH_EAST: Self = Self::Side(Direction::South, Direction::East);
    const SOUTH_SOUTH: Self = Self::Side(Direction::South, Direction::South);
    const SOUTH_NORTH: Self = Self::Side(Direction::South, Direction::North);

    const WEST: Self = Self::Direction(Direction::West);
    const EAST: Self = Self::Direction(Direction::East);
    const SOUTH: Self = Self::Direction(Direction::South);
    const NORTH: Self = Self::Direction(Direction::North);

    fn reverse_direction(&self) -> Self {
        match self {
            Self::Direction(d) => Self::Direction(d.inverse()),
            Self::Side(d, s) => Self::Side(d.inverse(), *s),
        }
    }
    fn direction(&self) -> &Direction {
        match self {
            Self::Direction(d) => d,
            Self::Side(d, _s) => d,
        }
    }
    fn from_directions(directions: Vec<Direction>) -> Vec<Self> {
        directions
            .into_iter()
            .map(|dir| Traverse::Direction(dir))
            .collect()
    }
    // inside means that you in inside the area from the first direction to the second direction clockwise
    fn directions(directions: [Direction; 2], inside: Option<bool>) -> Vec<Self> {
        // return all directions if all sides are availiable
        let Some(inside) = inside else {
            return Self::from_directions(Direction::MOVE.to_vec());
        };

        let (start, end) = match inside {
            true => (directions[0], directions[1]),
            false => (directions[1], directions[0]),
        };

        start
            .range(end)
            .into_iter()
            .map(|direction| {
                if direction == start {
                    Self::Side(direction, direction.right())
                } else if direction == end {
                    Self::Side(direction, direction.left())
                } else {
                    Traverse::Direction(direction)
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
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
    fn traverse(&self, inside: Option<bool>) -> Vec<Traverse> {
        Traverse::directions(self.pipe_directions(), inside)
    }

    // returns true if it is on the inside
    fn check_location_in_tile(&self, traverse: &Traverse) -> Option<bool> {
        // Empty is the same as inside
        if *self == Self::None {
            return None;
        }

        let directions = self.pipe_directions();
        let inside_range = directions[0].range(directions[1]);
        println!(
            "t {:?} IR: {:?} pd: {:?}",
            traverse,
            inside_range,
            self.pipe_directions()
        );

        // test with reverse direction

        if Traverse::directions(directions, Some(true)).contains(&traverse.reverse_direction()) {
            return Some(true);
        } else if Traverse::directions(directions, Some(false))
            .contains(&traverse.reverse_direction())
        {
            return Some(false);
        } else {
            return None;
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
            // .map(|(d, adjacent_tile)|(d.inverse(), adjacent_tile))
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

    fn debug_visited(&self, visited: &Vec<Coord>, p: Option<Coord>, n: Option<Coord>) {
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
    fn get_all_entrances(&self) -> Vec<(Traverse, Coord)> {
        let width = self.grid[0].len();
        let height = self.grid.len();

        let mut ret: Vec<(Traverse, Coord)> = Vec::new();
        // north side
        let mut top = (0..width)
            .map(|x| (Traverse::Direction(Direction::South), Coord::new(x, 0)))
            .collect();

        ret.append(&mut top);
        // south side
        let mut bottom = (0..width)
            .map(|x| {
                (
                    Traverse::Direction(Direction::North),
                    Coord::new(x, height - 1),
                )
            })
            .collect();
        ret.append(&mut bottom);

        // west side
        let mut bottom = (0..height)
            .map(|y| (Traverse::Direction(Direction::East), Coord::new(0, y)))
            .collect();
        ret.append(&mut bottom);

        // east side
        let mut bottom = (0..height)
            .map(|y| {
                (
                    Traverse::Direction(Direction::West),
                    Coord::new(width - 1, y),
                )
            })
            .collect();
        ret.append(&mut bottom);

        ret
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

fn check_direction(lines: &Vec<&str>, coord: &Coord, dir: &Direction) -> Option<Coord> {
    let width = lines[0].len();
    let height = lines.len();

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

fn get_symbol(lines: &Vec<&str>, coord: &Coord) -> char {
    let row = lines.get(coord.y).unwrap();
    let symbol_row: Vec<char> = row.chars().collect();
    let symbol = symbol_row.get(coord.x).unwrap();

    symbol.clone()
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

fn get_next_tile(
    lines: &Vec<&str>,
    back: Direction,
    current_symbol: char,
    current_location: Coord,
) -> Option<(Coord, Direction)> {
    let current_dirs = get_tile_connections(&current_symbol);

    for dir in current_dirs {
        // skip if visited previously
        if back == dir {
            continue;
        }
        // skip if out of bounds
        let next_coord = match check_direction(&lines, &current_location, &dir) {
            None => {
                continue;
            }
            Some(a) => a,
        };

        let next_tile = get_symbol(&lines, &next_coord);

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

fn part_1(input: &str) -> u64 {
    let mut part_1_answer: u64 = 0;

    let lines: Vec<&str> = input.lines().collect();

    let starting_point = match get_start(input) {
        Some(a) => a,
        None => panic!("No start"),
    };

    let mut current_location = starting_point;
    let mut back: Direction = Direction::None;

    // the number of nodes searched will never be larger than the size of the input
    for i in 0..input.len() {
        let current_symbol = get_symbol(&lines, &current_location);

        if current_symbol == 'S' && i != 0 {
            break;
        }
        part_1_answer += 1;

        let next_tile_res = get_next_tile(&lines, back, current_symbol, current_location);
        match next_tile_res {
            Some(a) => {
                current_location = a.0;
                back = a.1;
            }
            None => panic!("Hit dead end"),
        }
    }

    part_1_answer / 2
}

fn part_2(input: &str) -> usize {
    let hotsprings = HotSprings::from_str(input);

    let mut traversals: Vec<(Traverse, Coord)> = hotsprings.get_all_entrances();

    let mut visited: Vec<(Traverse, Coord)> = Vec::new();

    while let Some((traverse, previous_coord)) = traversals.pop() {
        let c_visited = &visited.iter().map(|a| a.1).collect::<Vec<Coord>>();

        // skip if cannot add
        let Ok(next_coord) = previous_coord + *traverse.direction() else {
            continue;
        };

        // skip if already visited
        if visited.contains(&(traverse, previous_coord)) {
            continue;
        }

        // skip if tile is out of bounds
        let Some(tile) = hotsprings.get_tile_p2(&next_coord) else {
            continue;
        };

        println!(
            "prev: {} {:?}",
            hotsprings.get_tile_p2(&previous_coord).unwrap().symbol(),
            (traverse, previous_coord)
        );

        // mark as visited
        visited.push((traverse, previous_coord));

        // get the side of the tile that was arrived at
        let inside = tile.check_location_in_tile(&traverse);

        // add next locations to traversals
        let mut next = tile
            .traverse(inside)
            .into_iter()
            .map(|traverse| (traverse, next_coord))
            .collect();

        println!(
            "T: {} Inside: {:?}, next: {:?}",
            tile.symbol(),
            inside,
            next
        );

        traversals.append(&mut next);

        hotsprings.debug_visited(c_visited, Some(previous_coord), Some(next_coord));
        println!();
    }

    let mut entrances: Vec<Coord> = hotsprings
        .get_all_entrances()
        .into_iter()
        .map(|a| a.1)
        .collect();

    let mut visited_coords: Vec<Coord> = visited.into_iter().map(|a| a.1).collect();
    visited_coords.append(&mut entrances);
    visited_coords.sort();
    visited_coords.dedup();

    hotsprings.debug_visited(&visited_coords, None, None);

    hotsprings.grid.len() * hotsprings.grid[0].len() - visited_coords.len()
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
    use crate::{part_2, Tile, Traverse};
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

    #[test]
    fn test_check_travel_direction() {
        let t = Tile::from_char(&'L');
        let traverse = Traverse::WEST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::EAST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));

        let traverse = Traverse::SOUTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::NORTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));

        let traverse = Traverse::WEST_WEST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::WEST_EAST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::WEST_SOUTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));

        let traverse = Traverse::WEST_NORTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(true));

        let t = Tile::from_char(&'|');
        let traverse = Traverse::WEST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(true));

        let traverse = Traverse::EAST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));

        let traverse = Traverse::SOUTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::NORTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::NORTH_WEST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));

        let traverse = Traverse::NORTH_EAST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(true));

        let traverse = Traverse::NORTH_SOUTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);

        let traverse = Traverse::NORTH_NORTH;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, None);
    }

    #[test]
    fn test_check_travel_direction_nw() {
        let t = Tile::from_char(&'J');
        let traverse = Traverse::SOUTH_WEST;
        let result = t.check_location_in_tile(&traverse);
        assert_eq!(result, Some(false));
    }

    #[test]
    fn test_check_direction() {
        let directions = Tile::from_char(&'|').pipe_directions();

        let result = Traverse::directions(directions, Some(true));
        assert_eq!(
            result,
            vec![Traverse::NORTH_EAST, Traverse::EAST, Traverse::SOUTH_EAST]
        );
    }
}
