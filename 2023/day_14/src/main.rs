use std::{collections::VecDeque, ops::Rem, time::Instant};
use library::input::Day;

fn calculate_weight(lines: &[Vec<char>]) -> usize {
    let res = lines
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|&&c| c == 'O').count() * (i + 1));

    res.sum()
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn flip(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.clone().into_iter().rev().collect())
        .collect()
}

fn roll_rocks(lines: Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    // rotate to make it easier to move the rocks

    let rotated = match direction {
        Direction::North => transpose(lines),
        Direction::West => lines,
        Direction::South => flip(transpose(lines)),
        Direction::East => flip(lines),
    };

    for mut line in rotated {
        let mut empty_positions: VecDeque<usize> = VecDeque::new();

        for i in 0..line.len() {
            let c = line.get(i).unwrap();
            match c {
                '.' => empty_positions.push_back(i),
                '#' => empty_positions.clear(),
                // take furthest right empty spot
                'O' => match empty_positions.pop_front() {
                    None => (),
                    Some(pos) => {
                        line.swap(pos, i);
                        // add now empty spot
                        empty_positions.push_back(i);
                    }
                },
                _ => panic!("Unaccounted for char {}", c),
            }
        }
        res.push(line);
    }

    match direction {
        Direction::North => transpose(res),
        Direction::West => res,
        Direction::South => transpose(flip(res)),
        Direction::East => flip(res),
    }
}

fn string_to_char_vec_vec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> usize {
    let formated_input: Vec<Vec<char>> = string_to_char_vec_vec(input);
    let rolled = roll_rocks(formated_input.clone(), Direction::North);

    calculate_weight(&rolled)
}

fn part_2(input: &str) -> usize {
    let formated_input: Vec<Vec<char>> = string_to_char_vec_vec(input);

    let mut records: Vec<(usize, Vec<Vec<char>>)> = Vec::new();
    let mut rolled = formated_input.clone();

    let mut cleared = false;

    let iterations = 1000000000;
    for i in 0..iterations {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            rolled = roll_rocks(rolled, direction);
        }

        // try to find patterns in each cycle
        match records.iter().find(|(_, a)| *a == rolled) {
            Some(_) => {
                if cleared {
                    break;
                } else {
                    // remove records from before pattern settled
                    records.clear();
                    cleared = true;
                }
            }
            None => records.push((i + 1, rolled.clone())),
        }
    }

    // calculate which repeating pattern will occur for the billionth cycle
    let remaining_cycles = iterations - records.iter().map(|a| a.0).min().unwrap();

    let remainder = remaining_cycles.rem(records.iter().len());

    let final_rotation = &records.get(remainder).unwrap().1;

    calculate_weight(final_rotation)
}

struct Day14;
const DAY: Day14 = Day14;
impl Day<usize> for Day14 {
    fn part_1(&self, input: &str) -> usize {
        let space = Space::parse(input);
        let galaxies = space.calculate_expanded_galaxies(2);

        galaxies
            .iter()
            .combinations(2)
            .fold(0, |acc, comb| acc + comb[0].distance(comb[1]))
    }
    fn part_2(&self, input: &str) -> usize {
        let space = Space::parse(input);
        let galaxies: Vec<Coords> = space.calculate_expanded_galaxies(1000000);

        galaxies
            .iter()
            .combinations(2)
            .fold(0, |acc, comb| acc + comb[0].distance(comb[1]))
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(input);
    let duration = start.elapsed();
    println!("Part 1 anwer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(input);
    let duration = start.elapsed();
    println!("Part 2 anwer: {}, time: {:?}", part_2_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_weight, roll_rocks, string_to_char_vec_vec, Direction};

    #[test]
    fn test_weight_calc() {
        let input = string_to_char_vec_vec(include_str!("../example_rolled_north.txt"));

        assert_eq!(136, calculate_weight(&input));
    }
    #[test]
    fn test_rolling_north() {
        let input = string_to_char_vec_vec(include_str!("../example.txt"));
        let expected = string_to_char_vec_vec(include_str!("../example_rolled_north.txt"));

        assert_eq!(expected, roll_rocks(input, Direction::North));
    }
    #[test]
    fn test_rolling_south() {
        let input = string_to_char_vec_vec(include_str!("../example.txt"));
        let expected = string_to_char_vec_vec(include_str!("../example_rolled_south.txt"));

        let result = roll_rocks(input, Direction::South);
        assert_eq!(expected, result);
    }
}
