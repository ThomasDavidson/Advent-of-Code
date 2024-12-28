use std::time::Instant;

use library::grid::{Direction, UVec2};

type Coord = UVec2<usize>;

struct WordSearch {
    text: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
impl WordSearch {
    fn from_input(input: &str) -> Self {
        let text: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self {
            width: text[0].len(),
            height: text.len(),
            text,
        }
    }

    fn find_letter(&self, find: char) -> Vec<Coord> {
        self.text
            .iter()
            .enumerate()
            .map(|(y, lines)| {
                lines
                    .iter()
                    .enumerate()
                    .map(move |(x, char)| (char, (x, y)))
            })
            .flatten()
            .filter(|(char, _)| char == &&find)
            .map(|(_char, (x, y))| Coord::new(x, y))
            .collect()
    }

    fn find_word_count(&self, word: &str) -> u32 {
        if word.len() == 0 {
            return 0;
        }
        let mut find = word.chars();

        let start = find.next().unwrap();

        let starting_points: Vec<Coord> = self.find_letter(start);
        let directions = Direction::DIAGINALS;

        let mut count = 0;
        for point in starting_points {
            for direction in &directions {
                if self.check_word("XMAS", point, direction) {
                    count += 1;
                }
            }
        }

        count
    }

    fn check_word(&self, word: &str, start: Coord, directions: &[Direction; 2]) -> bool {
        let mut coord = start;

        let [dir1, dir2] = *directions;

        for c in word.chars() {
            if c != self.text[coord.y][coord.x] {
                return false;
            }

            let first_add = match coord + dir1 {
                Ok(c) => c,
                Err(_) => continue,
            };

            let next_coord_check = match first_add + dir2 {
                Ok(c) => c,
                Err(_) => continue,
            };

            if next_coord_check.check_bounds(self.width, self.height) {
                continue;
            }

            coord = next_coord_check;
        }

        true
    }

    fn find_sub_string(&self, sub_string: Vec<Vec<char>>) -> u32 {


        let width = sub_string.len();
        let height = sub_string[0].len();

        let mut count = 0;

        // check with slices from top to bottom then left to right
        for x in 0..=(self.width - width) {
            for y in 0..=(self.height - height) {
                // get slices of y
                let y_slices = &self.text[y..(y + height)];
                'outer: for (i, y_slice) in y_slices.into_iter().enumerate() {
                    let x_slice = &y_slice[x..(x + width)];

                    for (a, b) in x_slice.iter().zip(sub_string[i].iter()) {
                        if b == &'.' {
                            continue;
                        }
                        if a != b {
                            break 'outer;
                        }
                    }

                    if (i + 1) == sub_string.len() {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone)]
struct Array2D {
    array: Vec<Vec<char>>,
}
impl Array2D {
    fn new(array: Vec<Vec<char>>) -> Self {
        Self { array }
    }
    fn from_lines(lines: Vec<&str>) -> Self {
        let array = lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();
        Self { array }
    }

    fn reverse_y(mut self) -> Self {
        self.array.reverse();
        self
    }

    fn reverse_x(mut self) -> Self {
        for l in &mut self.array {
            l.reverse();
        }
        self
    }

    fn rotate(self) -> Self {
        let rotated_sub_string: Vec<Vec<char>> = (0..self.array.len())
            .map(|y| (0..self.array[0].len()).map(|x| self.array[x][y]).collect())
            .collect();
        Self::new(rotated_sub_string)
    }
}

fn part_1(input: &str) -> u32 {
    let word_search = WordSearch::from_input(input);

    word_search.find_word_count("XMAS")
}

fn part_2(input: &str) -> u32 {
    let word_search = WordSearch::from_input(input);
    let mut part_2_answer = 0;

    let sub_string: Vec<Vec<char>> = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    let sub_string = Array2D::new(sub_string);

    part_2_answer += word_search.find_sub_string(sub_string.array.clone());

    let y_reversed_sub_string = sub_string.clone().reverse_x();

    part_2_answer += word_search.find_sub_string(y_reversed_sub_string.array);

    let rotated_sub_string = sub_string.clone().rotate();

    part_2_answer += word_search.find_sub_string(rotated_sub_string.clone().array);

    let y_reversed_rotated_sub_string = rotated_sub_string.clone().reverse_y();

    part_2_answer += word_search.find_sub_string(y_reversed_rotated_sub_string.array);

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
