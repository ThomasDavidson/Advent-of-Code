#[derive(Debug, Clone, PartialEq)]
struct Valley {
    land: Vec<Vec<char>>,
}

impl Valley {
    fn rotate(&self) -> Valley {
        let width = 0..self.land.first().unwrap().len();

        Valley {
            land: width
                .map(|x| {
                    self.land
                        .iter()
                        .map(|row| row.iter().nth(x).unwrap().clone())
                        .collect::<Vec<char>>()
                })
                .collect(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Valley> {
    let lines: Vec<&str> = input.lines().collect();

    let mut valleys: Vec<Valley> = Vec::new();

    for valley in lines.split(|&a| a == "") {
        valleys.push(Valley {
            land: valley.iter().map(|&s| s.chars().collect()).collect(),
        });
    }
    valleys
}

fn row_diff(row_1: &Vec<char>, row_2: &Vec<char>) -> usize {
    row_1
        .iter()
        .zip(row_2.iter())
        .fold(0, |acc: usize, (char_1, char_2)| {
            acc + if char_1 == char_2 { 0 } else { 1 }
        })
}

fn find_reflection(valley: &Valley, smudges: usize) -> Option<usize> {
    let mut last: Vec<char> = Vec::new();

    let mut mirrors: Vec<usize> = Vec::new();

    for (i, row) in valley.land.iter().enumerate() {
        // skip first comparison if empty
        if !last.is_empty() {
            let diff = row_diff(row, &last);
            if diff == 0 || diff == smudges {
                mirrors.push(i);
            }
        }

        last = row.clone();
    }

    if mirrors.is_empty() {
        return None;
    }

    for mirror in mirrors {
        let mut total_smudes = 0;

        let iter = 0..mirror;
        for (i, j) in iter.rev().zip(mirror..(mirror * 2)) {
            let cmp_row_1 = match valley.land.get(i) {
                None => break,
                Some(a) => a,
            };

            let cmp_row_2 = match valley.land.get(j) {
                None => break,
                Some(a) => a,
            };

            let diff = row_diff(cmp_row_1, cmp_row_2);

            total_smudes += diff;
        }

        if total_smudes == smudges {
            return Some(mirror);
        }
    }

    None
}
fn part_1(valleys: Vec<Valley>) -> usize {
    let mut answer = 0;
    for (i, valley) in valleys.iter().enumerate() {
        let res = match find_reflection(&valley, 0) {
            Some(a) => a * 100,
            None => match find_reflection(&valley.rotate(), 0) {
                Some(a) => a,
                None => panic!("No perfect mirror {}", i),
            },
        };
        answer += res;
    }
    answer
}

fn part_2(valleys: Vec<Valley>) -> usize {
    let mut answer = 0;
    for (i, valley) in valleys.iter().enumerate() {
        let res = match find_reflection(&valley, 1) {
            Some(a) => a * 100,
            None => match find_reflection(&valley.rotate(), 1) {
                Some(a) => a,
                None => panic!("No perfect mirror {}", i),
            },
        };
        answer += res;
    }
    answer
}
fn main() {
    let input = include_str!("../input.txt");

    let valleys: Vec<Valley> = parse_input(input);

    let part_1_answer = part_1(valleys.clone());
    println!("part 1 answer: {}", part_1_answer);

    let part_2_answer = part_2(valleys.clone());
    println!("part 2 answer: {}", part_2_answer);
}

#[cfg(test)]
mod tests {
    use crate::Valley;

    #[test]
    fn test_rotate() {
        let valley = Valley {
            land: vec![
                "#.##..##.".chars().collect(),
                "..#.##.#.".chars().collect(),
                "##......#".chars().collect(),
                "##......#".chars().collect(),
                "..#.##.#.".chars().collect(),
                "..##..##.".chars().collect(),
                "#.#.##.#.".chars().collect(),
            ],
        };

        let rotated = Valley {
            land: vec![
                "#.##..#".chars().collect(),
                "..##...".chars().collect(),
                "##..###".chars().collect(),
                "#....#.".chars().collect(),
                ".#..#.#".chars().collect(),
                ".#..#.#".chars().collect(),
                "#....#.".chars().collect(),
                "##..###".chars().collect(),
                "..##...".chars().collect(),
            ],
        };

        assert_eq!(rotated, valley.rotate());
    }
}
