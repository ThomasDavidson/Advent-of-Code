use std::collections::VecDeque;

fn calculate_weight(lines: &Vec<Vec<char>>) -> usize {
    let res = lines
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|&&c| c == 'O').count() * (i + 1));

    res.fold(0, |acc, i| acc + i)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn rotate(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = 0..lines.first().unwrap().len();

    width
        .map(|x| {
            lines
                .iter()
                .map(|row| row.iter().nth(x).unwrap().clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn roll_rocks(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    println!("w: {} h: {}", lines.first().unwrap().len(), lines.len());
    let mut res: Vec<Vec<char>> = Vec::new();
    // rotate to make it easier to move the rocks
    let rotated = rotate(lines);
    println!("w: {} h: {}", rotated.first().unwrap().len(), rotated.len());

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

    rotate(res)
}

fn string_to_char_vec_vec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> usize {
    let formated_input: Vec<Vec<char>> = string_to_char_vec_vec(input);
    let rolled = roll_rocks(formated_input.clone());

    for i in &rolled {
        println!("{}", i.iter().collect::<String>());
    }
    calculate_weight(&rolled)
}

fn main() {
    let input = include_str!("../input.txt");

    let part_1_answer = part_1(input);
    println!("part 1 answer: {}", part_1_answer);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_weight, rotate, string_to_char_vec_vec};

    #[test]
    fn test_weight_calc() {
        let input = string_to_char_vec_vec(include_str!("../example_rolled.txt"));

        assert_eq!(136, calculate_weight(&input));
    }
    #[test]
    fn test_rolling() {
        let input = string_to_char_vec_vec(include_str!("../example_rolled.txt"));
        let result = string_to_char_vec_vec(include_str!("../example_rolled.txt"));

        assert_eq!(result, rotate(input));
    }
}
