use library::input::{Day, InputType};
use roots::find_roots_quadratic;
use roots::Roots;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn calculate_time_margin(race: &Race) -> i64 {
    let a: f64 = -1f64;
    let b: f64 = race.time as f64;
    let c: f64 = -race.distance as f64;
    let root: Roots<f64> = find_roots_quadratic(a, b, c);

    let roots: [f64; 2] = match root {
        Roots::Two([x1, x2]) => [x1, x2],
        _ => panic!("Should not be none"),
    };

    let min_iter = roots[0].ceil() as i64;
    let max_iter = roots[1].floor() as i64;

    max_iter - min_iter + 1
}

#[derive(Debug)]
enum Races {
    Part1(Vec<Race>),
    Part2(Race),
}
impl Races {
    fn parse_part_1(input: &str) -> Self {
        let mut lines = input.lines();

        let times: Vec<i64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let distances: Vec<i64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let races = distances
            .into_iter()
            .zip(times)
            .map(|(distance, time)| Race { distance, time })
            .collect();

        Self::Part1(races)
    }

    fn parse_part_2(input: &str) -> Self {
        let mut lines = input.lines();

        let time = lines
            .next()
            .unwrap()
            .chars()
            .filter(|arg0: &char| char::is_numeric(*arg0))
            .collect::<String>()
            .parse()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .chars()
            .filter(|arg0: &char| char::is_numeric(*arg0))
            .collect::<String>()
            .parse()
            .unwrap();

        Self::Part2(Race { time, distance })
    }
}

struct Day6;
const DAY: Day6 = Day6;
impl Day<i64> for Day6 {
    fn part_1(&self, input: &str) -> i64 {
        let Races::Part1(races) = Races::parse_part_1(input) else {
            panic!()
        };

        let mut part_1_answer: i64 = 1;

        for race in &races {
            let margin: i64 = calculate_time_margin(race);
            part_1_answer *= margin;
        }

        part_1_answer
    }
    fn part_2(&self, input: &str) -> i64 {
        let Races::Part2(race) = Races::parse_part_2(input) else {
            panic!()
        };

        calculate_time_margin(&race)
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_margin_1() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        let margin: i64 = calculate_time_margin(&race);

        assert_eq!(margin, 4);
    }

    #[test]
    fn check_margin_2() {
        let race = Race {
            time: 15,
            distance: 40,
        };
        let margin: i64 = calculate_time_margin(&race);

        assert_eq!(margin, 8);
    }

    #[test]
    fn check_margin_3() {
        let race = Race {
            time: 30,
            distance: 200,
        };
        let margin: i64 = calculate_time_margin(&race);

        assert_eq!(margin, 11);
    }
    #[test]
    fn check_margin_4_precision_test() {
        let race = Race {
            time: 40000000,
            distance: 50000000000,
        };
        let margin: i64 = calculate_time_margin(&race);

        assert_eq!(margin, 39997499);
    }
}
