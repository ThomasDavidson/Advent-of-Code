use std::time::Instant;

use library::grid::{Coord, UVec2};

#[derive(Debug)]
struct Robot {
    position: Coord,
    velocity: UVec2<i32>,
}
impl Robot {
    fn from_line(line: &str) -> Self {
        let (position_str, velocity_str) = line.split_once(" ").unwrap();

        let position_xy_str = position_str.split("=").nth(1).unwrap();

        let (pos_x, pos_y) = position_xy_str.split_once(",").unwrap();

        let Some(pos_x) = pos_x.parse().ok() else {
            panic!("Cannot parse pos x '{}'", pos_x);
        };

        let Some(pos_y) = pos_y.parse().ok() else {
            panic!("Cannot parse pos y '{}'", pos_y);
        };

        let position = Coord::new(pos_x, pos_y);

        let velocity_xy_str = velocity_str.split("=").nth(1).unwrap();

        let (vel_x, vel_y) = velocity_xy_str.split_once(",").unwrap();

        let Some(vel_x) = vel_x.parse().ok() else {
            panic!("Cannot parse pos x '{}'", vel_x);
        };

        let Some(vel_y) = vel_y.parse().ok() else {
            panic!("Cannot parse pos y '{}'", vel_y);
        };

        let velocity = UVec2::new(vel_x, vel_y);

        Self { position, velocity }
    }

    fn simulate(&mut self, width: usize, height: usize) {
        let mut new_x = self.position.x as i32 + self.velocity.x;
        if new_x < 0 {
            new_x += width as i32;
        } else if new_x >= width as i32 {
            new_x %= width as i32;
        }
        self.position.x = new_x as usize;

        let mut new_y = self.position.y as i32 + self.velocity.y;
        if new_y < 0 {
            new_y += height as i32;
        } else if new_y >= height as i32 {
            new_y %= height as i32;
        }
        self.position.y = new_y as usize;
    }
}

struct Bathroom {
    height: usize,
    width: usize,
    robots: Vec<Robot>,
}
impl Bathroom {
    fn from_input(input: &str, width: usize, height: usize) -> Self {
        let robots = input.lines().map(|line| Robot::from_line(line)).collect();

        Self {
            height,
            width,
            robots,
        }
    }
    fn simulate(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.simulate(self.width, self.height);
        }
    }

    fn simulate_n(&mut self, n: usize) {
        for _ in 0..n {
            self.simulate();
        }
    }

    fn num_robots_in_area(&self, x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> usize {
        let inside = self.robots.iter().filter(|robot| {
            robot.position.y >= y_min
                && robot.position.y < y_max
                && robot.position.x >= x_min
                && robot.position.x < x_max
        });

        inside.count()
    }
    fn debug(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self
                    .robots
                    .iter()
                    .filter(|robot| robot.position.x == x && robot.position.y == y)
                    .count();
                if count == 0 {
                    print!(".")
                } else {
                    print!("{count}");
                }
            }
            println!();
        }
    }
}

fn part_1(input: &str) -> u64 {
    let mut bathroom = Bathroom::from_input(input, 101, 103);

    bathroom.simulate_n(100);

    let quarter_width = bathroom.width.div_euclid(2);
    let quarter_2_start_x = bathroom.width.div_ceil(2);

    let quarter_height = bathroom.height.div_euclid(2);
    let quarter_2_start_y = bathroom.height.div_ceil(2);

    let mut part_1_answer = 1;

    for (y_min, y_max) in [(0, quarter_height), (quarter_2_start_y, bathroom.height)] {
        for (x_min, x_max) in [(0, quarter_width), (quarter_2_start_x, bathroom.width)] {
            let quarter_count = bathroom.num_robots_in_area(x_min, x_max, y_min, y_max);
            part_1_answer *= quarter_count;
        }
    }

    part_1_answer as u64
}

fn part_2(input: &str) -> u64 {
    let mut bathroom = Bathroom::from_input(input, 101, 103);

    bathroom.simulate_n(100);

    let quarter_width = bathroom.width.div_euclid(2);
    let q2_start_x = bathroom.width.div_ceil(2);

    for i in 0..1000 {
        for y in 0..bathroom.height {
            let left_robots = bathroom.num_robots_in_area(0, quarter_width, y, y + 1);
            let right_robots = bathroom.num_robots_in_area(q2_start_x, bathroom.width, y, y + 1);
            // println!("left: {left_robots}, right: {right_robots}");
            if left_robots != right_robots {
                break;
            }
            bathroom.debug();
            return i;
        }
        bathroom.simulate();
    }

    bathroom.debug();
    0
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
