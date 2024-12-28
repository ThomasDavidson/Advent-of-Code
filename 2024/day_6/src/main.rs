use colorize::AnsiColor;
use core::fmt;
use library::grid::{Direction, UVec2};
use std::time::Instant;
type Coord = UVec2<usize>;

#[derive(PartialEq, Clone)]
enum Tile {
    Floor,
    Obstruction,
    Start,
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            '#' => Self::Obstruction,
            '^' => Self::Start,
            _ => panic!("Invalid tile char"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Floor => '.',
            Self::Obstruction => '#',
            Self::Start => '^',
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone)]
struct Lab {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}
impl Lab {
    fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
            .collect();
        let width = grid.len();
        let height = grid[0].len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn clone_with_wall(&self, coords: &Coord) -> Self {
        let mut clone: Lab = self.clone();

        clone.grid[coords.y][coords.x] = Tile::Obstruction;
        clone
    }

    fn get_start(&self) -> Coord {
        let Some(start_y) = self
            .grid
            .iter()
            .position(|line| line.contains(&Tile::Start))
        else {
            panic!("Cannot find start y position");
        };

        let Some(start_x) = self.grid[start_y].iter().position(|c| c == &Tile::Start) else {
            panic!("Cannot find start x position");
        };

        Coord::new(start_x, start_y)
    }

    fn get_tile(&self, coords: &Coord) -> &Tile {
        &self.grid[coords.y][coords.x]
    }

    fn next(&self, direction: Direction, current: &Coord) -> Option<Coord> {
        let next = (*current + direction).ok()?;

        if next.check_bounds(self.width, self.height) {
            return None;
        }

        if self.get_tile(&next) == &Tile::Obstruction {
            return None;
        }

        Some(next)
    }

    fn is_exit(&self, current: &Coord, direction: &Direction) -> bool {
        if current.x == 0 && direction == &Direction::West {
            true
        } else if current.x + 1 == self.width && direction == &Direction::East {
            true
        } else if current.y == 0 && direction == &Direction::North {
            true
        } else if current.y + 1 == self.height && direction == &Direction::South {
            true
        } else {
            false
        }
    }

    fn check_loop(&self, current: &Coord, visited: &Vec<Vec<Visited>>) -> bool {
        let mut direction = Direction::North;
        let mut current = self.get_start();
        let mut visited = vec![vec![Visited::init(); self.width]; self.height];

        loop {
            if visited[current.y][current.x].get(&direction) {
                // self.debug_2(&current, &visited);

                return true;
            }
            visited[current.y][current.x].set(&direction);
            // check if exit
            if self.is_exit(&current, &direction) {
                return false;
            }
            // find next tile in same direction
            let next = self.next(direction, &current);

            if let Some(next) = next {
                // if wall or obsticle then turn
                current = next;
            } else {
                direction = direction.right();
            }
        }
    }

    fn debug(&self, current: &Coord, visited: &Vec<Vec<Visited>>, loop_wall: Vec<Vec<bool>>) {
        for (y, line) in visited.iter().enumerate() {
            for (x, v) in line.iter().enumerate() {
                let tile = self.get_tile(&Coord::new(x, y));
                let tile = format!("{}", tile);
                // if current.x == x && current.y == y {
                // print!("X");
                // } else
                if loop_wall[y][x] {
                    print!("{}", tile.green());
                } else if v.visited() {
                    print!("{}", v);
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
        println!();
    }
    fn debug_2(&self, current: &Coord, visited: &Vec<Vec<Visited>>) {
        for (y, line) in visited.iter().enumerate() {
            for (x, v) in line.iter().enumerate() {
                let tile = self.get_tile(&Coord::new(x, y));
                let tile = format!("{}", tile);
                if current.x == x && current.y == y {
                    print!("X");
                } else if v.visited() {
                    print!("{}", tile.red());
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
        println!();
    }
}

fn part_1(input: &str) -> u32 {
    let lab = Lab::from_input(input);

    let start = lab.get_start();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; lab.width]; lab.height];

    let mut direction = Direction::North;
    let mut current = start;

    loop {
        visited[current.y][current.x] = true;
        // check if exit
        if lab.is_exit(&current, &direction) {
            break;
        }
        // find next tile in same direction
        let next = lab.next(direction, &current);

        if let Some(next) = next {
            // if wall or obsticle then turn
            current = next;
        } else {
            direction = direction.right();
        }
    }

    let visited = visited.iter().flatten().filter(|v| **v);

    visited.count() as u32
}

#[derive(Debug, Clone)]
struct Visited {
    east: bool,
    west: bool,
    north: bool,
    south: bool,
}
impl fmt::Display for Visited {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
impl Visited {
    fn init() -> Self {
        Self {
            east: false,
            west: false,
            north: false,
            south: false,
        }
    }
    fn visited(&self) -> bool {
        self.east || self.west || self.north || self.south
    }
    fn set(&mut self, direction: &Direction) {
        match direction {
            Direction::East => self.east = true,
            Direction::West => self.west = true,
            Direction::North => self.north = true,
            Direction::South => self.south = true,
            _ => panic!(),
        }
    }
    fn get(&self, direction: &Direction) -> bool {
        match direction {
            Direction::East => self.east,
            Direction::West => self.west,
            Direction::North => self.north,
            Direction::South => self.south,
            _ => panic!(),
        }
    }
    fn to_char(&self) -> char {
        match (self.east, self.west, self.south, self.north) {
            (false, false, false, false) => ' ', // No connections

            (true, false, false, false) => '─', // Line to the east
            (false, true, false, false) => '─', // Line to the west
            (true, true, false, false) => '─',  // Horizontal line

            (false, false, true, false) => '│', // Line to the north
            (false, false, false, true) => '│', // Line to the south
            (false, false, true, true) => '│',  // Vertical line

            // (true, false, true, false) => '└', // Corner: east + north
            // (true, false, false, true) => '┌', // Corner: east + south
            // (false, true, true, false) => '┘', // Corner: west + north
            // (false, true, false, true) => '┐', // Corner: west + south

            // (true, true, true, false) => '┴', // T-shape: horizontal + north
            // (true, true, false, true) => '┬', // T-shape: horizontal + south
            // (true, false, true, true) => '├', // T-shape: vertical + east
            // (false, true, true, true) => '┤', // T-shape: vertical + west
            _ => '┼', // Cross: all directions
        }
    }
}

fn part_2(input: &str) -> u32 {
    let lab = Lab::from_input(input);

    let start = lab.get_start();

    let mut visited = vec![vec![Visited::init(); lab.width]; lab.height];
    let mut wall = vec![vec![false; lab.width]; lab.height];

    let mut direction = Direction::North;
    let mut current = start;

    let mut part_2_answer = 0;

    loop {
        if !wall[current.y][current.x] && lab.get_tile(&current) == &Tile::Floor {
            let add_wall = lab.clone_with_wall(&current);

            if add_wall.check_loop(&current, &visited) {
                part_2_answer += 1;
                wall[current.y][current.x] = true;
            }
        }

        if visited[current.y][current.x].get(&direction) {
            panic!("in a loop");
        }
        visited[current.y][current.x].set(&direction);
        // check if exit
        if lab.is_exit(&current, &direction) {
            break;
        }
        // find next tile in same direction
        let next = lab.next(direction, &current);

        if let Some(next) = next {
            // if wall or obsticle then turn
            current = next;
        } else {
            direction = direction.right();
        }
    }

    lab.debug(&current, &visited, wall.clone());

    let visited = wall.iter().flatten().filter(|v| **v);

    let visited = visited.count() as u32;
    println!("Wall: {:}", visited);

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
