use crate::Tile::{GardenPlot, Rocks, Start};
use colored::Colorize;
use library::grid::Direction;
use library::math::{round_to, sawtooth};
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

    fn get_steps_range(
        &self,
        y_max: i64,
        y_min: i64,
        x_max: i64,
        x_min: i64,
    ) -> impl Iterator<Item = &u32> {
        let entries = self
            .steps
            .iter()
            .filter(move |(k, _v)| k.1 < y_max && k.1 >= y_min && k.0 < x_max && k.0 >= x_min);

        entries.map(|x| x.1)
    }
    fn print_steps_range(&self, y_max: i64, y_min: i64, x_max: i64, x_min: i64) {
        let entries: Vec<((i64, i64), u32)> = self
            .steps
            .clone()
            .drain()
            .filter(move |(k, _v)| k.1 < y_max && k.1 >= y_min && k.0 < x_max && k.0 >= x_min)
            .collect();

        println!("{:?}", entries)
    }

    fn calculate_score_for_each_universe(&self, max_steps: u32) -> Vec<Vec<u64>> {
        let width = self.width();
        let height = self.height();

        let steps: Vec<&(i64, i64)> = self.steps.keys().collect::<Vec<_>>();

        let min_x_visited = steps.iter().min_by_key(|f| f.0).unwrap().0;
        let min_y_visited = steps.iter().min_by_key(|f| f.1).unwrap().1;
        let max_x_visited = steps.iter().max_by_key(|f| f.0).unwrap().0;
        let max_y_visited = steps.iter().max_by_key(|f| f.1).unwrap().1;

        let min_x_step = round_to(min_x_visited, -(width as i64));
        let min_y_step = round_to(min_y_visited, -(height as i64));
        let max_x_step = round_to(max_x_visited, width as i64);
        let max_y_step = round_to(max_y_visited, height as i64);

        for x in (min_x_step..max_x_step).step_by(width) {
            for y in (min_y_step..max_y_step).step_by(height) {
                let steps = self.get_steps_range(y + height as i64, y as i64, x + width as i64, x);
                let score = self.calculate_score_from_iter(steps, max_steps);
                print!("{score}\t");
            }
            println!()
        }

        let us: Vec<Vec<u64>> = (min_x_step..max_x_step)
            .step_by(width)
            .map(|x| {
                (min_y_step..max_y_step)
                    .step_by(height)
                    .map(|y| {
                        let steps =
                            self.get_steps_range(y + height as i64, y as i64, x + width as i64, x);
                        self.calculate_score_from_iter(steps, max_steps)
                    })
                    .collect()
            })
            .collect();

        us
    }

    fn calculate_score_from_iter<'a>(
        &self,
        iter: impl Iterator<Item = &'a u32>,
        max_steps: u32,
    ) -> u64 {
        iter.map(|step| {
            // even coordinates can only be reached with an even amount of steps an vise versa with odd steps
            let even_coord = step % 2 == 0;
            let even_max_step = max_steps % 2 == 0;
            match even_coord == even_max_step {
                true => 1,
                false => 0,
            }
        })
        .fold(0, |acc, x| acc + x)
    }

    fn calculate_score(&self, max_steps: u32) -> u64 {
        self.get_step_coords()
            .map(|step| {
                // even coordinates can only be reached with an even amount of steps an vise versa with odd steps
                let even_coord = step % 2 == 0;
                let even_max_step = max_steps % 2 == 0;
                match even_coord == even_max_step {
                    true => 1,
                    false => 0,
                }
            })
            .fold(0, |acc, x| acc + x)
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
                        format!("E")
                    } else {
                        format!("O")
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
    garden.calculate_score(max_steps)
}

fn part_2(input: &str, max_steps: u32) -> u64 {
    let mut garden = Garden::from_string(input);

    let width = garden.width() as i64;
    let height = garden.height() as i64;
    if width != height {
        panic!("Not sqare");
    }

    let reduced_max_steps = if max_steps > width as u32 * 4 {
        (max_steps as i64 % width + width * 2) as u32
    } else {
        println!("Using part 1 method due to size");
        return part_1(input, max_steps);
    };

    println!("steps: {max_steps}, reduced steps: {reduced_max_steps}");

    let gardener = Elf {
        max_steps: reduced_max_steps,
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

    let scores = garden.calculate_score_for_each_universe(max_steps);

    let origin = if scores.len() == 5 { 2 } else { 3 };

    println!("Origin: {origin}");

    let e1 = scores[origin][origin];
    let e2 = scores[origin - 1][origin];

    println!("e: {e1}");
    println!("e2: {e2}");

    let a1 = scores[origin - 1][origin - 1];
    let a2 = scores[origin - 1][origin + 1];
    let a3 = scores[origin + 1][origin + 1];
    let a4 = scores[origin + 1][origin - 1];
    let a = a1 + a2 + a3 + a4;

    let t1 = scores[origin - 2][origin];
    let t2 = scores[origin][origin - 2];
    let t3 = scores[origin + 2][origin];
    let t4 = scores[origin][origin + 2];

    // set 0 if d doesn't exist
    let (d1, d2) = if scores.len() == 5 {
        (0, 0)
    } else {
        let d1: u64 = scores[origin - 3][origin];
        let d2 = scores[origin][origin - 3];
        (d1, d2)
    };

    let (d3, d4) = if origin + 3 > scores.len() {
        let d3 = scores[origin + 3][origin];
        let d4 = scores[origin][origin + 3];
        (d3, d4)
    } else {
        (0, 0)
    };

    let d = d1 + d2 + d3 + d4;

    let t = t1 + t2 + t3 + t4;

    let b1 = scores[origin - 1][origin - 2];
    let b2 = scores[origin - 2][origin + 1];
    let b3 = scores[origin + 1][origin - 2];
    let b4 = scores[origin + 1][origin + 2];
    let b = b1 + b2 + b3 + b4;

    println!("a1: {a1}, a2: {a2}, a3: {a3}, a4: {a4}");
    println!("t1: {t1}, t2: {t2}, t3: {t3}, t4: {t4}");
    println!("b1: {b1}, b2: {b2}, b3: {b3}, b4: {b4}");
    println!("d1: {d1}, d2: {d2}, d3: {d3}, d4: {d4}");

    println!("a: {a}, b: {b}, t: {t}, d {d}");

    let s = (width - 1) / 2;
    let n = ((max_steps as i64 - s) / width) as u64;

    println!("s: {s}, n: {n}");

    (n - 1).pow(2) * e1 + n.pow(2) * e2 + (n - 1) * a + n * b + t + d
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input, 64);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input, 1000);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
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
    fn test2_4() {
        let input = include_str!("../example.txt");
        let result = part_2(input, 100);
        assert_eq!(result, 6536);
    }
    #[test]
    fn test2_5() {
        let input = include_str!("../example.txt");
        let result = part_2(input, 500);
        assert_eq!(result, 167004);
    }
    #[test]
    fn test2_6() {
        let input = include_str!("../example.txt");
        let result = part_2(input, 1000);
        assert_eq!(result, 668697);
    }
    #[test]
    fn test2_7() {
        let input = include_str!("../example.txt");
        let result = part_2(input, 5000);
        assert_eq!(result, 16733044);
    }
}
