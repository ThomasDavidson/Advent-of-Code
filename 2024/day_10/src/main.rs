use std::time::Instant;

use library::grid::{find_in_coord, Direction, UVec2};
type Coord = UVec2<usize>;

struct TopologicalMap {
    map: Vec<Vec<u32>>,
}
impl TopologicalMap {
    fn from_input(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap_or(99))
                    .collect()
            })
            .collect();

        Self { map }
    }

    fn get(&self, coord: &Coord) -> u32 {
        self.map[coord.y][coord.x]
    }

    fn get_start(&self) -> Vec<Coord> {
        find_in_coord(&self.map, &0)
    }

    fn next(&self, coord: &Coord) -> Vec<Coord> {
        let height = self.map.len();
        let width = self.map[0].len();
        let curr_height = self.get(coord);

        let mut next_coords = Vec::new();
        for d in Direction::MOVE.iter() {
            let next_coord = match *coord + *d {
                Err(_) => continue,
                Ok(c) => c,
            };

            if next_coord.check_bounds(width, height) {
                continue;
            }

            let next_height = self.get(&next_coord);

            if next_height != curr_height + 1 {
                continue;
            }
            next_coords.push(next_coord);
        }

        next_coords
    }

    fn traverse(&self, curr: &Coord, visited: &mut Option<Vec<Vec<bool>>>) -> u64 {
        if let Some(visited) = visited {
            if visited[curr.y][curr.x] == true {
                return 0;
            }

            visited[curr.y][curr.x] = true;
        }

        let curr_height = self.get(&curr);

        if curr_height == 9 {
            return 1;
        }

        let mut score = 0;
        for next in self.next(curr) {
            score += self.traverse(&next, visited);
        }

        return score;
    }
}

fn part_1(input: &str) -> u64 {
    let map = TopologicalMap::from_input(input);

    let starting_points = map.get_start();

    let mut part_1_answer: u64 = 0;

    let height = map.map.len();
    let width = map.map[0].len();

    for start in &starting_points {
        let visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let score = map.traverse(start, &mut Some(visited));
        part_1_answer += score;
    }

    part_1_answer
}

fn part_2(input: &str) -> u64 {
    let map = TopologicalMap::from_input(input);

    let starting_points = map.get_start();

    let mut part_1_answer: u64 = 0;

    for start in &starting_points {
        // the existance of this line of code is unsettling
        let score = map.traverse(start, &mut None);
        part_1_answer += score;
    }

    part_1_answer
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
    use crate::part_1;

    #[test]
    fn test1() {
        let input = include_str!("../example3.txt");
        let result = part_1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let input = include_str!("../example4.txt");
        let result = part_1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let input = include_str!("../example5.txt");
        let result = part_1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let input = include_str!("../example2.txt");
        let result = part_1(input);
        assert_eq!(result, 36);
    }
}
