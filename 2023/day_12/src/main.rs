use core::panic;
use library::input::{Day, InputType};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Record {
    row: Vec<char>,
    damaged: Vec<usize>,
}
impl Record {
    fn valid(&self) -> bool {
        if self.is_damaged() {
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
                        let cmp_damaged = cmp_rec.pop()?;

                        if count != cmp_damaged {
                            return None;
                        }

                        count = 0;
                    }
                },
                '#' => count += 1,
                _ => panic!("unexpected char {}", c),
            }
        }

        if count != 0 {
            // last match doesn't account for the next characters
            let last_cmp = cmp_rec.last()?;

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

    fn is_damaged(&self) -> bool {
        self.row.contains(&'?')
    }
    fn parse_record(input: &str) -> Option<Record> {
        let split_input = input.split_once(" ")?;

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
            damaged,
        })
    }
    fn get_record_variations(self) -> usize {
        let mut cache: HashMap<Record, usize> = HashMap::new();

        self.memoized_get_record_variations(&mut cache)
    }
    fn memoized_get_record_variations(&self, cache: &mut HashMap<Record, usize>) -> usize {
        let mut result = 0;

        if !self.is_damaged() {
            return match self.valid() {
                true => 1,
                false => 0,
            };
        }

        if let Some(a) = cache.get(self) {
            return *a;
        };

        let mut mut_row: Vec<char> = self.row.to_owned();

        let f = mut_row.iter().enumerate().find(|(_, a)| **a == '?');
        let i = match f {
            Some(i) => i.0,
            None => panic!("Should not get here without ? in the string"),
        };

        mut_row[i] = '.';

        let new_record = Record {
            damaged: self.damaged.to_owned(),
            row: mut_row.to_owned(),
        };

        result += match new_record.partial_compare(i + 1) {
            Some(a) => Record::memoized_get_record_variations(&a, cache),
            None => 0,
        };

        mut_row[i] = '#';

        let new_record = Record {
            damaged: self.damaged.to_owned(),
            row: mut_row,
        };

        result += match new_record.partial_compare(i + 1) {
            Some(a) => Record::memoized_get_record_variations(&a, cache),
            None => 0,
        };
        cache.insert(self.clone(), result);

        result
    }
    fn unfold_record(self) -> Record {
        let mut new_row: Vec<char> = Vec::new();
        let mut new_damaged: Vec<usize> = Vec::new();

        for i in 0..5 {
            let mut append_record: Vec<char> = self.row.to_owned();

            new_row.append(&mut append_record);
            // don't add values between
            if i != 4 {
                new_row.push('?');
            }

            new_damaged.append(&mut self.damaged.clone());
        }

        Record {
            row: new_row,
            damaged: new_damaged,
        }
    }
}

struct Day12;
const DAY: Day12 = Day12;
impl Day<usize> for Day12 {
    fn part_1(&self, input: &str) -> usize {
        let records: Vec<Record> = input.lines().filter_map(Record::parse_record).collect();

        records
            .into_iter()
            .filter(Record::is_damaged)
            .map(Record::get_record_variations)
            .sum()
    }
    fn part_2(&self, input: &str) -> usize {
        input
            .lines()
            .filter_map(Record::parse_record)
            .map(Record::unfold_record)
            .map(Record::get_record_variations)
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use crate::Record;

    #[test]
    fn test_variations() {
        let record = Record {
            damaged: vec![3, 2, 1],
            row: "?###????????".chars().collect(),
        };
        let res = Record::get_record_variations(record);

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
                assert!(res.is_some(), "record {:?} iter: {}", record, i);
            }
        }
    }
}
