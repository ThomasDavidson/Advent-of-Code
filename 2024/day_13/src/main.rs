use std::time::Instant;

use library::grid::UVec2;
use regex::Regex;
type Coord = UVec2<usize>;

#[derive(Debug)]
struct ClawMachine {
    prize: Coord,
    button_a: Coord,
    button_b: Coord,
}
impl ClawMachine {
    fn from_str(str: &str) -> Self {
        let mut lines = str.lines();

        let a = lines.next().unwrap();

        let re_a = Regex::new(r"Button A: X\+([0-9]{1,}), Y\+([0-9]{1,})").unwrap();
        let Some(a_capt) = re_a.captures(a) else {
            panic!();
        };
        let Some(a_x) = a_capt[1].parse().ok() else {
            println!("{:?}", a_capt[1].chars());
            panic!();
        };
        let Some(a_y) = a_capt[2].parse().ok() else {
            println!("{:?}", a_capt[2].chars());
            panic!();
        };

        let button_a = Coord::new(a_x, a_y);

        let b = lines.next().unwrap();

        let re_b = Regex::new(r"Button B: X\+([0-9]{1,}), Y\+([0-9]{1,})").unwrap();
        let Some(b_capt) = re_b.captures(b) else {
            panic!();
        };
        let b_x = b_capt[1].parse().unwrap();
        let b_y = b_capt[2].parse().unwrap();
        let button_b = Coord::new(b_x, b_y);

        let prize = lines.next().unwrap();
        let re_prize = Regex::new(r"Prize: X=([0-9]{1,}), Y=([0-9]{1,})").unwrap();
        let Some(prize_capt) = re_prize.captures(prize) else {
            panic!();
        };
        let prize_x = prize_capt[1].parse().unwrap();
        let prize_y = prize_capt[2].parse().unwrap();
        let prize = Coord::new(prize_x, prize_y);

        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn get_button_presses(&self) -> Option<(u64, u64)> {
        let a2 = self.button_b.x as f64;
        let b2 = self.button_a.x as f64;
        let c2 = -(self.prize.x as f64);
        // println!("{a2}x + {b2}y + {c2} = 0");

        let a1 = self.button_b.y as f64;
        let b1 = self.button_a.y as f64;
        let c1 = -(self.prize.y as f64);
        // println!("{a1}x + {b1}y + {c1} = 0");

        let a_count = (a1 * c2 - a2 * c1) / (b1 * a2 - b2 * a1);

        let b_count = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);

        let (a_count, b_count) = if a_count.ceil() != a_count || b_count.ceil() != b_count {
            return None;
        } else {
            (a_count as u64, b_count as u64)
        };

        Some((a_count, b_count))
    }
}

struct Lobby {
    claw_machines: Vec<ClawMachine>,
}
impl Lobby {
    fn from_input(input: &str) -> Self {
        let claw_machines = input
            .split("\r\n\r\n")
            .map(|machine| ClawMachine::from_str(machine))
            .collect();

        Self { claw_machines }
    }
}

fn part_1(input: &str) -> u64 {
    let lobby = Lobby::from_input(input);

    let mut part_1_answer = 0;

    for machine in lobby.claw_machines {
        let Some((a, b)) = machine.get_button_presses() else {
            continue;
        };
        part_1_answer += a * 3 + b;
    }
    part_1_answer
}

fn part_2(input: &str) -> u64 {
    let mut lobby = Lobby::from_input(input);
    for machine in lobby.claw_machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
    }

    let mut part_1_answer = 0;

    for machine in lobby.claw_machines {
        let Some((a, b)) = machine.get_button_presses() else {
            continue;
        };
        part_1_answer += a * 3 + b;
    }
    part_1_answer
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
