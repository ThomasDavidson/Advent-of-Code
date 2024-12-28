use std::ops::Range;

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
        } else if new_x > width as i32 {
            new_x %= width as i32;
        }
        self.position.x = new_x as usize;

        let mut new_y = self.position.y as i32 + self.velocity.y;
        if new_y < 0 {
            new_y += height as i32;
        } else if new_y > height as i32 {
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

    fn num_robots_in_area(&self, x_range: Range<usize>, y_range: Range<usize>) {
        
    }
}

fn main() {
    let input = include_str!("../example.txt");

    let mut bathroom = Bathroom::from_input(input, 11, 7);

    bathroom.simulate_n(100);
}
