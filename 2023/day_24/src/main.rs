use itertools::Itertools;
use library::grid::Vec3;
use library::input::{Day, InputType};
use nalgebra::{Matrix6, Matrix6x1, RowVector6};

#[derive(Debug, Clone, PartialEq, Eq)]
struct HailStone {
    position: Vec3<i128>,
    velocity: Vec3<i128>,
}
impl HailStone {
    fn from_line(line: &str) -> Self {
        let (position_str, velocity_str) = line.split_once(" @ ").unwrap();
        let position: Vec3<i128> = Vec3::parse(position_str).unwrap();
        let velocity: Vec3<i128> = Vec3::parse(velocity_str).unwrap();
        Self { position, velocity }
    }
    fn get_equation(&self) -> (i128, i128, i128) {
        // new test
        let a = -self.velocity.x;
        let b = self.velocity.y;
        let c = self.velocity.x * self.position.y - self.velocity.y * self.position.x;

        (a, b, c)
    }

    fn check_intersection_xy(&self, other: &Self) -> Option<(f64, f64)> {
        let (a1, b1, c1) = self.get_equation();
        let (a2, b2, c2) = other.get_equation();

        let y0 = (b1 * c2 - b2 * c1) as f64 / (a1 * b2 - a2 * b1) as f64;
        let x0 = (c1 * a2 - c2 * a1) as f64 / (b2 * a1 - a2 * b1) as f64;
        if y0.is_infinite() || x0.is_infinite() {
            return None;
        }
        Some((x0, y0))
    }

    fn future_intersection_xy(&self, (x0, y0): (f64, f64)) -> bool {
        // check if the intersection has already happened
        let (dx, dy) = (x0 - self.position.x as f64, y0 - self.position.y as f64);

        if (dy.is_sign_positive() != self.velocity.y.is_positive())
            || (dx.is_sign_positive() != self.velocity.x.is_positive())
        {
            return false;
        }

        true
    }
}

#[derive(Debug)]
struct HailStorm {
    hail_stones: Vec<HailStone>,
}
impl HailStorm {
    fn from_str(input: &str) -> Self {
        let hail_stones: Vec<HailStone> = input.lines().map(HailStone::from_line).collect();

        Self { hail_stones }
    }
}

#[derive(Clone)]
struct Day24 {
    range: (f64, f64),
}
const DAY: Day24 = Day24 {
    range: (200000000000000.0, 400000000000000.0),
};
impl Day<u64> for Day24 {
    fn part_1(&self, input: &str) -> u64 {
        let storm = HailStorm::from_str(input);

        let mut score = 0;

        let (xy_min, xy_max) = self.range;

        for hail_stones in storm
            .hail_stones
            .iter()
            .combinations(2)
            .filter(|hs| hs[0] != hs[1])
        {
            let hail_stone = hail_stones[0];
            let hail_stone2 = hail_stones[1];

            let (x0, y0) = match hail_stone.check_intersection_xy(hail_stone2) {
                None => continue,
                Some(xy0) => xy0,
            };

            if !hail_stone.future_intersection_xy((x0, y0)) {
                continue;
            }

            if !hail_stone2.future_intersection_xy((x0, y0)) {
                continue;
            }

            // check if intersection is within area
            if (x0 < xy_min || x0 > xy_max) || (y0 < xy_min || y0 > xy_max) {
                continue;
            }
            score += 1;
        }
        score
    }
    // todo improve answer for any input
    fn part_2(&mut self, input: &str) -> u64 {
        let storm = HailStorm::from_str(input);

        let p0: Vec3<f64> = storm.hail_stones[0].position.into();
        let p1: Vec3<f64> = storm.hail_stones[1].position.into();
        let p2: Vec3<f64> = storm.hail_stones[6].position.into();
        let v0: Vec3<f64> = storm.hail_stones[0].velocity.into();
        let v1: Vec3<f64> = storm.hail_stones[1].velocity.into();
        let v2: Vec3<f64> = storm.hail_stones[6].velocity.into();

        let b = Matrix6x1::from_row_slice(&[
            ((p0.y as i128 * v0.x as i128 - p1.y as i128 * v1.x as i128)
                - (p0.x as i128 * v0.y as i128 - p1.x as i128 * v1.y as i128)) as f64,
            ((p0.y as i128 * v0.x as i128 - p2.y as i128 * v2.x as i128)
                - (p0.x as i128 * v0.y as i128 - p2.x as i128 * v2.y as i128)) as f64,
            ((p0.z as i128 * v0.x as i128 - p1.z as i128 * v1.x as i128)
                - (p0.x as i128 * v0.z as i128 - p1.x as i128 * v1.z as i128)) as f64,
            ((p0.z as i128 * v0.x as i128 - p2.z as i128 * v2.x as i128)
                - (p0.x as i128 * v0.z as i128 - p2.x as i128 * v2.z as i128)) as f64,
            ((p0.z as i128 * v0.y as i128 - p1.z as i128 * v1.y as i128)
                - (p0.y as i128 * v0.z as i128 - p1.y as i128 * v1.z as i128)) as f64,
            ((p0.z as i128 * v0.y as i128 - p2.z as i128 * v2.y as i128)
                - (p0.y as i128 * v0.z as i128 - p2.y as i128 * v2.z as i128)) as f64,
        ]);

        let a = Matrix6::from_rows(&[
            RowVector6::new(v1.y - v0.y, v0.x - v1.x, 0.0, p0.y - p1.y, p1.x - p0.x, 0.0),
            RowVector6::new(v2.y - v0.y, v0.x - v2.x, 0.0, p0.y - p2.y, p2.x - p0.x, 0.0),
            RowVector6::new(v1.z - v0.z, 0.0, v0.x - v1.x, p0.z - p1.z, 0.0, p1.x - p0.x),
            RowVector6::new(v2.z - v0.z, 0.0, v0.x - v2.x, p0.z - p2.z, 0.0, p2.x - p0.x),
            RowVector6::new(0.0, v1.z - v0.z, v0.y - v1.y, 0.0, p0.z - p1.z, p1.y - p0.y),
            RowVector6::new(0.0, v2.z - v0.z, v0.y - v2.y, 0.0, p0.z - p2.z, p2.y - p0.y),
        ]);

        let r = a.lu().solve(&b).unwrap();
        let answer: f64 = r[0] + r[1] + r[2];
        answer.round() as u64
    }
}

fn main() -> std::io::Result<()> {
    DAY.clone().run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use crate::{Day24, HailStone};
    use library::input::Day;
    const DAY: Day24 = Day24 { range: (7.0, 27.0) };

    #[test]
    fn test1() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[1]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((14.0 + 1. / 3., 15. + 1. / 3.)));
    }
    #[test]
    fn test2() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[2]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((11.0 + 2. / 3., 16. + 2. / 3.)));
    }
    #[test]
    fn test3() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((6.2, 19.4)));
    }
    #[test]
    fn test4() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[1]);
        let hail_stone2 = HailStone::from_line(lines[2]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, None);
    }
    #[test]
    fn test5() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[1]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((-6., -5.)));
    }
    #[test]
    fn test6() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[2]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((-2., 3.)));
    }
    #[test]
    fn test7() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[2]);
        let hail_stone2 = HailStone::from_line(lines[4]);

        let result = hail_stone.check_intersection_xy(&hail_stone2);
        assert_eq!(result, Some((19., 24.)));
    }
    #[test]
    fn test_part_1_example() {
        let input = include_str!("../example.txt");
        let result = DAY.part_1(input);

        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_2_example() {
        let input = include_str!("../example.txt");
        let result = DAY.part_2(input);

        assert_eq!(result, 47);
    }
}
