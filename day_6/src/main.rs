use roots::find_roots_quadratic;
use roots::Roots;
use std::iter::zip;

struct Race {
    time: i32,
    distance: i32,
}

fn calculate_time_margin(race: &Race) -> u64 {
    let a: f32 = -1f32;
    let b: f32 = race.time as f32;
    let c: f32 = -race.distance as f32;
    let root = find_roots_quadratic(a, b, c);

    println!("{:?}", root);

    let roots: [f32;2] = match root {
        Roots::Two([x1, x2]) => [x1, x2],
        _ => panic!("Should not be none"),
    };
    let mut ret = 0;

    let min_iter = roots[0].floor() as i32;
    let max_iter = roots[1].ceil() as i32;

    for i in min_iter..max_iter {
        if (i as f32) > roots[0] && (i as f32) < roots[1] {
            ret = ret + 1;
        }
    }

    return ret;
}

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", input);

    let mut times: Vec<i32> = vec![];
    let mut distances: Vec<i32> = vec![];

    let mut day_1_races: Vec<Race> = vec![];

    for (i, line) in input.lines().enumerate() {
        for day_1_races in line.split_whitespace() {
            let val = day_1_races.parse::<i32>();

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

    let mut day_1_answer: u64 = 1;

    for race in day_1_races {
        let margin: u64 = calculate_time_margin(&race);
        println!("distance: {} time: {} margin: {}", race.distance, race.time, margin);

        day_1_answer = day_1_answer * margin;
    }
    println!("day_1_answer: {}", day_1_answer);
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
        let margin: u64 = calculate_time_margin(&race);

        assert_eq!(margin, 4);
    }

    #[test]
    fn check_margin_2() {
        let race = Race {
            time: 15,
            distance: 40,
        };
        let margin: u64 = calculate_time_margin(&race);

        assert_eq!(margin, 8);
    }

    #[test]
    fn check_margin_3() {
        let race = Race {
            time: 30,
            distance: 200,
        };
        let margin: u64 = calculate_time_margin(&race);

        assert_eq!(margin, 9);
    }
}
