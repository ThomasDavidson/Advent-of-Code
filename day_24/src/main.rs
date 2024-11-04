use library::grid::Vec3;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
struct HailStone {
    position: Vec3<i128>,
    velocity: Vec3<i128>,
}
impl HailStone {
    fn from_line(line: &str) -> Self {
        let (position_str, velocity_str) = line.split_once(" @ ").unwrap();
        let position: Vec3<i128> = Vec3::from_str(position_str).unwrap();
        let velocity: Vec3<i128> = Vec3::from_str(velocity_str).unwrap();
        Self { position, velocity }
    }
    fn get_equation(&self) -> (i128, i128, i128) {
        // new test
        let a = -self.velocity.x;
        let b = self.velocity.y;
        let c = self.velocity.x * self.position.y - self.velocity.y * self.position.x;

        (a, b, c)
    }

    fn check_intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let (a1, b1, c1) = self.get_equation();
        let (a2, b2, c2) = other.get_equation();

        let y0 = (b1 * c2 - b2 * c1) as f64 / (a1 * b2 - a2 * b1) as f64;
        let x0 = (c1 * a2 - c2 * a1) as f64 / (b2 * a1 - a2 * b1) as f64;
        if y0.is_infinite() || x0.is_infinite() {
            return None;
        }
        Some((x0, y0))
    }
}

#[derive(Debug)]
struct HailStorm {
    hail_stones: Vec<HailStone>,
}
impl HailStorm {
    fn from_str(input: &str) -> Self {
        let hail_stones: Vec<HailStone> = input
            .lines()
            .map(|line| HailStone::from_line(line))
            .collect();

        Self { hail_stones }
    }
}
fn part_1(input: &str) -> u64 {
    let storm = HailStorm::from_str(input);

    let mut score = 0;

    let xy_min = 200000000000000.0;
    let xy_max = 400000000000000.0;

    let mut min_j = 0;
    for hail_stone in storm.hail_stones.iter() {
        min_j += 1;
        for (_j, hail_stone2) in storm
            .hail_stones
            .iter()
            .enumerate()
            .filter(|hs| hs.1 != hail_stone && min_j <= hs.0)
        {
            let (x0, y0) = match hail_stone.check_intersection(hail_stone2) {
                None => continue,
                Some(xy0) => xy0,
            };

            // check if the intersection has already happened
            let (dx, dy) = (
                x0 - hail_stone.position.x as f64,
                y0 - hail_stone.position.y as f64,
            );

            if (dy.is_sign_positive() != hail_stone.velocity.y.is_positive())
                || (dx.is_sign_positive() != hail_stone.velocity.x.is_positive())
            {
                continue;
            }

            let (dx2, dy2) = (
                x0 - hail_stone2.position.x as f64,
                y0 - hail_stone2.position.y as f64,
            );

            if (dy2.is_sign_positive() != hail_stone2.velocity.y.is_positive())
                || (dx2.is_sign_positive() != hail_stone2.velocity.x.is_positive())
            {
                continue;
            }

            // check if intersection is within area
            if (x0 < xy_min || x0 > xy_max) || (y0 < xy_min || y0 > xy_max) {
                continue;
            }
            score += 1;
        }
    }
    score
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::HailStone;

    #[test]
    fn test1() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[1]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((14.0 + 1. / 3., 15. + 1. / 3.)));
    }
    #[test]
    fn test2() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[2]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((11.0 + 2. / 3., 16. + 2. / 3.)));
    }
    #[test]
    fn test3() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[0]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((6.2, 19.4)));
    }
    #[test]
    fn test4() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[1]);
        let hail_stone2 = HailStone::from_line(lines[2]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, None);
    }
    #[test]
    fn test5() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[1]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((-6., -5.)));
    }
    #[test]
    fn test6() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[2]);
        let hail_stone2 = HailStone::from_line(lines[3]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((-2., 3.)));
    }
    #[test]
    fn test7() {
        let input = include_str!("../example.txt");
        let lines: Vec<&str> = input.lines().collect();
        let hail_stone = HailStone::from_line(lines[2]);
        let hail_stone2 = HailStone::from_line(lines[4]);

        let result = hail_stone.check_intersection(&hail_stone2);
        assert_eq!(result, Some((19., 24.)));
    }
}
