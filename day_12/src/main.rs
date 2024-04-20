use core::panic;

#[derive(Debug, Clone)]
struct Record {
    row: String,
    damaged: Vec<usize>,
}
impl Record {
    fn valid(&self) -> bool {
        if self.row.contains('?') {
            return false;
        }
        let found_damaged_areas: Vec<usize> = self
            .row
            .split('.')
            .map(|a| a.len())
            .filter(|&a| a != 0)
            .collect();

        found_damaged_areas == self.damaged
    }
    fn partial_compare(&self, length: usize) -> bool {
        let limit = if self.row.len() < length {
            self.row.len()
        } else {
            length
        };

        let mut count = 0;
        let mut cmp_rec: Vec<usize> = self.damaged.clone();
        cmp_rec.reverse();

        for c in self.row.chars().take(limit) {
            match c {
                '.' => match count {
                    0 => continue,
                    _ => {
                        let cmp_damaged = match cmp_rec.pop() {
                            Some(a) => a,
                            None => return false,
                        };

                        if count != cmp_damaged {
                            return false;
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
            let last_cmp = match cmp_rec.pop() {
                Some(a) => a,
                None => return false,
            };

            if count > last_cmp {
                return false;
            }
        }

        true
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
        row: split_input.0.to_string(),
        damaged: damaged,
    })
}

fn get_record_variations(record: &Record) -> usize {
    let mut result = 0;

    if !record.row.contains('?') {
        match record.valid() {
            true => return 1,
            false => return 0,
        }
    }

    let mut mut_row: Vec<char> = record.row.chars().collect();

    let f = mut_row.iter().enumerate().find(|(_, a)| **a == '?');
    let i = match f {
        Some(i) => i.0,
        None => panic!("Should not get here without ? in the string"),
    };

    mut_row[i] = '.';

    let new_record = Record {
        damaged: record.damaged.to_owned(),
        row: mut_row.iter().collect(),
    };

    if new_record.partial_compare(i + 1) {
        result += get_record_variations(&new_record);
    }

    mut_row[i] = '#';

    let new_record = Record {
        damaged: record.damaged.to_owned(),
        row: mut_row.iter().collect(),
    };

    if new_record.partial_compare(i + 1) {
        result += get_record_variations(&new_record);
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");

    let records: Vec<Record> = input
        .lines()
        .map(|line| parse_record(line).unwrap())
        .collect();

    let mut part_1_answer = 0;

    for record in records {
        if record.row.contains("?") {
            let res = get_record_variations(&record);
            println!("Result: {}", res);
            part_1_answer += res;
        } else {
            println!("{:?} valid: {}", record, record.valid());
        }
    }
    println!("Part 1 anwer: {}", part_1_answer);
}

#[cfg(test)]
mod tests {
    use crate::{get_record_variations, Record};

    #[test]
    fn test_variations() {
        let record = Record {
            damaged: vec![3, 2, 1],
            row: "?###????????".to_string(),
        };
        let res = get_record_variations(&record);

        assert_eq!(res, 7);
    }

    #[test]
    fn test_valid_1() {
        let records: Vec<Record> = vec![
            Record {
                damaged: vec![3, 2, 1],
                row: ".###.##.#...".to_string(),
            },
            Record {
                damaged: vec![3, 2, 1],
                row: ".###.##..#..".to_string(),
            },
            Record {
                damaged: vec![3, 2, 1],
                row: ".###..##.#..".to_string(),
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
