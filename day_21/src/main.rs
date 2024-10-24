use crate::Tile::{GardenPlot, Rocks, Start};
use colored::Colorize;
use library::grid::Direction;
use library::math::sawtooth;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    time::Instant,
};

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
    fn to_char(&self) -> char {
        match self {
            Start => 'S',
            GardenPlot => '.',
            Rocks => '#',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<Tile>>,
    steps: HashMap<(i64, i64), u32>,
}

impl Garden {
    pub const DEFAULT_STEP: u32 = u32::MAX;

    fn from_string(input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
            .collect();

        Self {
            grid,
            steps: HashMap::new(),
        }
    }
    fn find_start(&self) -> (i64, i64) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if *tile == Start {
                    return (x as i64, y as i64);
                }
            }
        }
        panic!("Cannot find start")
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_tile(&self, coords: (i64, i64)) -> (usize, usize) {
        let width = self.width();
        let height = self.height();
        let (x, y) = coords;

        // uses sawtooth to get tile in infinitly expanding map
        let index_x = sawtooth(x, width as i64);
        let index_y = sawtooth(y, height as i64);
        (index_y as usize, index_x as usize)
    }

    fn visited(&self, elf: &Elf) -> bool {
        // checks if spot has been visited before with fewer steps
        let steps = match self.steps.get(&elf.coords) {
            None => Garden::DEFAULT_STEP,
            Some(step) => *step,
        };

        steps <= elf.steps
    }

    fn visit(&mut self, elf: &Elf) {
        self.steps.insert(elf.coords, elf.steps);
    }

    fn get_step_coords(&self) -> impl Iterator<Item = &u32> {
        self.steps.values()
    }

    fn get_color(&self, x: i64, y: i64) -> u8 {
        let width = self.width() as i64;
        let height = self.width() as i64;
        match ((x + width * 10) / width % 2, (y + height * 10) / height % 2) {
            (0, 0) => 0,
            (_, 0) => 1,
            (0, _) => 2,
            (_, _) => 3,
        }
    }

    fn find_next(&mut self, elf: Elf) -> VecDeque<Elf> {
        let (x, y) = elf.coords;

        if elf.steps == elf.max_steps {
            return VecDeque::new();
        }

        let directions = Direction::MOVE;

        let mut coords = VecDeque::new();

        for direction in directions {
            let (x_offset, y_offset) = direction.get_translation();
            let (next_x, next_y) = ((x + x_offset as i64), (y + y_offset as i64));

            let (index_x, index_y) = self.get_tile((next_x, next_y));
            let next_tile = &self.grid[index_x][index_y];

            if *next_tile == Rocks {
                continue;
            }

            let new_elf = Elf {
                coords: (next_x, next_y),
                steps: elf.steps + 1,
                ..elf
            };

            if self.visited(&new_elf) {
                continue;
            }

            coords.push_front(new_elf);
        }
        coords
    }
}

impl fmt::Display for Garden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let steps: Vec<&(i64, i64)> = self.steps.keys().collect::<Vec<_>>();
        let min_x_visited = steps.iter().min_by_key(|f| f.0).unwrap().0 - 10;
        let min_y_visited = steps.iter().min_by_key(|f| f.1).unwrap().1 - 10;
        let max_x_visited = steps.iter().max_by_key(|f| f.0).unwrap().0 + 10;
        let max_y_visited = steps.iter().max_by_key(|f| f.1).unwrap().1 + 10;

        for y in min_y_visited..max_y_visited {
            for x in min_x_visited..max_x_visited {
                let (index_x, index_y) = self.get_tile((x, y));
                let tile = &self.grid[index_x][index_y];
                let elf = Elf {
                    max_steps: 0,
                    steps: 0,
                    coords: (x as i64, y as i64),
                };

                let s = if let Some(steps) = self.steps.get(&elf.coords) {
                        if steps % 2 == 0 {
                        format!("O")
                        } else {
                        format!("{}", tile)
                    }
                } else {
                    format!("{}", tile)
                };

                let cs = match self.get_color(x, y) {
                    0 => format!("{}", s).red(),
                    1 => format!("{}", s).blue(),
                    2 => format!("{}", s).magenta(),
                    3 => format!("{}", s).green(),
                    _ => panic!(),
                };

                write!(f, "{}", cs)?;
            }
            writeln!(f)?; // Add a newline at the end of each row
        }

        Ok(())
    }
}

struct Elf {
    max_steps: u32,
    steps: u32,
    coords: (i64, i64),
}

fn part_1(input: &str, max_steps: u32) -> i32 {
    let mut garden = Garden::from_string(input);

    let gardener = Elf {
        max_steps,
        steps: 0,
        coords: garden.find_start(),
    };

    let mut gardeners = VecDeque::from([gardener]);

    while let Some(gardener) = gardeners.pop_front() {
        if garden.visited(&gardener) {
            continue;
        }

        garden.visit(&gardener);
        let mut next = garden.find_next(gardener);
        gardeners.append(&mut next);
    }

    println!("{}", &garden);

    garden
        .get_step_coords()
        .map(|step| {
            // check if visited from step count
            if max_steps % 2 == 0 {
                if step % 2 == 0 {
                    1
                } else {
                    0
                }
            } else {
                if step % 2 == 0 {
                    0
                } else {
                    1
                }
            }
        })
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input, 64);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn test1() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 6);
        assert_eq!(result, 16);
    }
    #[test]
    fn test2() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 10);
        assert_eq!(result, 50);
    }
    #[test]
    fn test3() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 50);
        assert_eq!(result, 1594);
    }
    #[test]
    fn test4() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 100);
        assert_eq!(result, 6536);
    }
    #[test]
    fn test5() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 500);
        assert_eq!(result, 167004);
    }
    #[test]
    fn test6() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 1000);
        assert_eq!(result, 668697);
    }
    #[test]
    fn test7() {
        let input = include_str!("../example.txt");
        let result = part_1(input, 5000);
        assert_eq!(result, 16733044);
    }
}
