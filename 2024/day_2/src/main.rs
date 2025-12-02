use library::input::{Day, InputType};

fn valid_increment(increment: i64, is_ascending: bool) -> bool {
    is_ascending == (increment > 0) && increment.abs() <= 3 && increment.abs() > 0
}

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}
impl Report {
    fn from_line(line: &str) -> Self {
        let levels: Vec<u32> = line
            .split(" ")
            .map(|level_str| level_str.parse().unwrap())
            .collect();

        Self { levels }
    }
    fn is_valid(&self) -> bool {
        let len = self.levels.len();

        let a = &self.levels[1..];
        let b = &self.levels[..(len - 1)];

        let level_difference: Vec<i64> = a
            .iter()
            .zip(b)
            .map(|(a, b)| (*a as i64) - (*b as i64))
            .collect();

        if level_difference.iter().all(|diff| *diff <= 3 && *diff > 0) {
            return true;
        }

        if level_difference.iter().all(|diff| *diff >= -3 && *diff < 0) {
            return true;
        }

        false
    }

    fn is_valid_part_2(&self, is_ascending: bool, reverse_check: bool) -> bool {
        let len = self.levels.len();

        let a = &self.levels[1..];
        let b = &self.levels[..(len - 1)];

        let mut level_difference: Vec<i64> = a
            .iter()
            .zip(b)
            .map(|(a, b)| (*a as i64) - (*b as i64))
            .collect();

        if reverse_check {
            level_difference.reverse();
        }

        let mut adjusted = false;

        let mut i = 0;
        loop {
            // check for if value was remove from level_difference
            if i >= level_difference.len() {
                break;
            }

            if !valid_increment(level_difference[i], is_ascending) {
                // if already adjusted then return false
                if adjusted {
                    return false;
                }
                adjusted = true;

                let invalid_level_diff = level_difference[i];

                level_difference.remove(i);

                // if on edge of diff vec then continue
                if i == level_difference.len() || i == 0 {
                    continue;
                }

                // if adding difference makes it valid then check other numbers
                if valid_increment(invalid_level_diff + level_difference[i], is_ascending) {
                    level_difference[i] += invalid_level_diff;
                    i = 0;
                    continue;
                } else if valid_increment(
                    invalid_level_diff + level_difference[i - 1],
                    is_ascending,
                ) {
                    level_difference[i - 1] += invalid_level_diff;
                    i = 0;
                    continue;
                }
                // if it cannot be fixed then return false
                return false;
            }
            i += 1;
        }

        true
    }
}

type Reports = Vec<Report>;

fn reports_from_str(input: &str) -> Reports {
    input.lines().map(Report::from_line).collect()
}

#[derive(Clone)]
struct Day2;
const DAY: Day2 = Day2;
impl Day<u32> for Day2 {
    fn part_1(&self, input: &str) -> u32 {
        let reports = reports_from_str(input);

        reports
            .iter()
            .map(Report::is_valid)
            .map(|is_valid| if is_valid { 1 } else { 0 })
            .sum()
    }
    fn part_2(&mut self, input: &str) -> u32 {
        let reports = reports_from_str(input);

        reports
            .iter()
            .map(|report| {
                report.is_valid()
                    || report.is_valid_part_2(true, false)
                    || report.is_valid_part_2(false, false)
                    || report.is_valid_part_2(true, true)
                    || report.is_valid_part_2(false, true)
            })
            .map(|is_valid| if is_valid { 1 } else { 0 })
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.clone().run(InputType::UserInput)
}
