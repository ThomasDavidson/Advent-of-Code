use core::panic;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Record {
    row: Vec<char>,
    damaged: Vec<usize>,
}
impl Record {
    fn valid(&self) -> bool {
        if self.is_damged() {
            return false;
        }
        let found_damaged_areas: Vec<usize> = self
            .row
            .iter()
            .collect::<String>()
            .split('.')
            .map(|a| a.len())
            .filter(|&a| a != 0)
            .collect();

        found_damaged_areas == self.damaged
    }
    fn partial_compare(&self, length: usize) -> Option<Record> {
        let limit = if self.row.len() < length {
            self.row.len()
        } else {
            length
        };

        let mut count = 0;
        let mut cmp_rec: Vec<usize> = self.damaged.clone();
        cmp_rec.reverse();

        for c in self.row.iter().take(limit) {
            match c {
                '.' => match count {
                    0 => continue,
                    _ => {
                        let cmp_damaged = match cmp_rec.pop() {
                            Some(a) => a,
                            None => return None,
                        };

                        if count != cmp_damaged {
                            return None;
                        }

                        count = 0;
                    }
                },
                '#' => count = count + 1,
                _ => panic!("unexpected char {}", c),
            }
        }

        if count != 0 {
            // last match is doesn't account for the next characters
            let last_cmp = match cmp_rec.last() {
                Some(a) => a,
                None => return None,
            };

            if count > *last_cmp {
                return None;
            }
        }

        let first_dot_before_check = self
            .row
            .iter()
            .take(limit)
            .enumerate()
            .rev()
            .find(|(_, &a)| a == '.');

        // return the same if it cannot find a dot
        let index = match first_dot_before_check {
            None | Some((0, _)) => return Some(self.clone()),
            Some(a) => a.0,
        };

        let row_removed: Vec<char> = self.row.clone().drain(index..).collect();

        // unreverse to use
        cmp_rec.reverse();

        Some(Record {
            damaged: cmp_rec,
            row: row_removed,
        })
    }

    fn is_damged(&self) -> bool {
        self.row.iter().any(|&a| a == '?')
    }
}

fn parse_record(input: &str) -> Option<Record> {
    let split_input = match input.split_once(" ") {
        Some(a) => a,
        None => return None,
    };

    if !split_input
        .0
        .chars()
        .all(|a| a == '.' || a == '#' || a == '?')
    {
        return None;
    }

    let damaged: Vec<usize> = split_input
        .1
        .split(',')
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    Some(Record {
        row: split_input.0.chars().collect(),
        damaged: damaged,
    })
}

fn get_record_variations(record: &Record) -> usize {
    let mut result = 0;

    if !record.is_damged() {
        match record.valid() {
            true => return 1,
            false => return 0,
        }
    }

    let mut mut_row: Vec<char> = record.row.to_owned();

    let f = mut_row.iter().enumerate().find(|(_, a)| **a == '?');
    let i = match f {
        Some(i) => i.0,
        None => panic!("Should not get here without ? in the string"),
    };

    mut_row[i] = '.';

    let new_record = Record {
        damaged: record.damaged.to_owned(),
        row: mut_row.to_owned(),
    };

    result += match new_record.partial_compare(i + 1) {
        Some(a) => get_record_variations(&a),
        None => 0,
    };

    mut_row[i] = '#';

    let new_record = Record {
        damaged: record.damaged.to_owned(),
        row: mut_row,
    };

    result += match new_record.partial_compare(i + 1) {
        Some(a) => get_record_variations(&a),
        None => 0,
    };

    result
}

fn part_1(records: Vec<Record>) -> usize {
    let mut part_1_answer = 0;

    for record in records {
        if record.is_damged() {
            let res = get_record_variations(&record);
            // println!("Result: {}", res);
            part_1_answer += res;
        }
    }

    part_1_answer
}

fn unfold_record(record: Record) -> Record {
    let mut new_row: Vec<char> = Vec::new();
    let mut new_damged: Vec<usize> = Vec::new();

    for i in 0..5 {
        let mut append_record: Vec<char> = record.row.to_owned();

        new_row.append(&mut append_record);
        // don't add values between
        if i != 4 {
            new_row.push('?');
        }

        new_damged.append(&mut record.damaged.clone());
    }

    Record {
        row: new_row,
        damaged: new_damged,
    }
}

fn part_2(records: Vec<Record>) -> usize {
    let mut part_2_answer = 0;

    let start = Instant::now();
    let p2_records = records.iter().map(|record| unfold_record(record.clone()));
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    for record in p2_records {
        println!(
            "Record: {:?} {:?}",
            record.row.iter().collect::<String>(),
            record.damaged
        );

        let start: Instant = Instant::now();

        let res = get_record_variations(&record);
        let duration = start.elapsed();
        print!("Time elapsed is: {:?}", duration);
        println!(" per result: {:?}", duration / (res.max(1) as u32));

        println!("Result: {}", res);
        println!("");
        part_2_answer += res;
    }

    part_2_answer
}

fn main() {
    let input = include_str!("../example_damaged.txt");

    let records: Vec<Record> = input
        .lines()
        .map(|line| parse_record(line).unwrap())
        .collect();

    let start: Instant = Instant::now();

    let part_1_answer = part_1(records.clone());
    let duration = start.elapsed();
    println!("Part 1 time is: {:?}", duration);

    println!("Part 1 answer: {}", part_1_answer);

    let part_2_answer = part_2(records.clone());
    println!("Part 2 anwer: {}", part_2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{get_record_variations, Record};

    #[test]
    fn test_variations() {
        let record = Record {
            damaged: vec![3, 2, 1],
            row: "?###????????".chars().collect(),
        };
        let res = get_record_variations(&record);

        assert_eq!(res, 10);
    }

    #[test]
    fn test_valid_1() {
        let records: Vec<Record> = vec![
            Record {
                damaged: vec![3, 2, 1],
                row: ".###.##.#...".chars().collect(),
            },
            Record {
                damaged: vec![3, 2, 1],
                row: ".###.##..#..".chars().collect(),
            },
            Record {
                damaged: vec![3, 2, 1],
                row: ".###..##.#..".chars().collect(),
            },
        ];

        for record in records {
            let res = record.valid();
            assert_eq!(res, true, "record {:?}", record);

            for i in 1..record.row.len() {
                let res = record.partial_compare(i);
                assert_eq!(res, true, "record {:?} iter: {}", record, i);
            }
        }
    }
}
