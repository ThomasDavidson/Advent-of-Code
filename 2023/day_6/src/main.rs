use roots::find_roots_quadratic;
use roots::Roots;
use std::iter::zip;

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

    println!("{:?}", root);

    let roots: [f64; 2] = match root {
        Roots::Two([x1, x2]) => [x1, x2],
        _ => panic!("Should not be none"),
    };

    let min_iter = roots[0].ceil() as i64;
    let max_iter = roots[1].floor() as i64;

    println!("Win max: {} min: {}", max_iter, min_iter);

    let diff = max_iter - min_iter + 1;

    // for i in min_iter..max_iter {
    //     if (i as f64) > roots[0] && (i as f64) < roots[1] {
    //         ret = ret + 1;
    //     }
    // }

    return diff;
}

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", input);

    let mut times: Vec<i64> = vec![];
    let mut distances: Vec<i64> = vec![];

    let mut day_1_races: Vec<Race> = vec![];

    for (i, line) in input.lines().enumerate() {
        for day_1_races in line.split_whitespace() {
            let val = day_1_races.parse::<i64>();

            match val {
                Err(_e) => (),

                Ok(num) => {
                    if i == 0 {
                        times.push(num);
                    } else {
                        distances.push(num);
                    }
                }
            };
        }
    }

    for (time, distance) in zip(times.iter(), distances.iter()) {
        let race = Race {
            distance: distance.clone(),
            time: time.clone(),
        };
        day_1_races.push(race);
    }

    let mut day_1_answer: i64 = 1;

    for race in &day_1_races {
        let margin: i64 = calculate_time_margin(&race);
        // println!("distance: {} time: {} margin: {}", race.distance, race.time, margin);

        day_1_answer = day_1_answer * margin;
    }
    println!("day_1_answer: {}", day_1_answer);

    let mut day_2_time_str: String = Default::default();
    let mut day_2_distance_str: String = Default::default();
    for race in &day_1_races {
        day_2_time_str.push_str(&race.time.to_string());
        day_2_distance_str.push_str(&race.distance.to_string());
    }

    let day_2_race = Race {
        time: day_2_time_str.parse::<i64>().unwrap(),
        distance: day_2_distance_str.parse::<i64>().unwrap(),
    };
    println!("{:?}", day_2_race);

    let day_2_answer: i64 = calculate_time_margin(&day_2_race);
    println!("Day 2 margin: {}", day_2_answer);
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
