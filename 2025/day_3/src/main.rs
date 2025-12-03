use library::input::{Day, InputType};
use std::collections::VecDeque;

#[derive(Debug)]
struct BatteryBank {
    banks: Vec<Batteries>,
}
impl BatteryBank {
    fn parse(text: &str) -> Self {
        let banks = text.lines().map(Batteries::parse).collect();
        Self { banks }
    }
}

#[derive(Debug)]
struct Batteries {
    batteries: Vec<u8>,
}
impl Batteries {
    fn parse(text: &str) -> Self {
        let batteries = text
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        Self { batteries }
    }

    fn largest_voltage(&self, size: usize) -> u64 {
        largest_numbers_in_order(&self.batteries, size)
            .iter()
            .rev()
            .enumerate()
            .map(|(a, b)| *b as u64 * 10_u64.pow(a as u32))
            .sum()
    }
}

fn largest_numbers_in_order(slice: &[u8], size: usize) -> VecDeque<u8> {
    if size == 0 {
        return VecDeque::new();
    }

    let len = slice.len();

    let (max_first_pos, max_first) = &slice[0..(len - size + 1)]
        .iter()
        .enumerate()
        // reverse so max returns the first largest number
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let new_slice = &slice[(max_first_pos + 1)..];

    let mut result = largest_numbers_in_order(new_slice, size - 1);

    result.insert(0, **max_first);

    result
}

struct Day3;
const DAY: Day3 = Day3;
impl Day<u64> for Day3 {
    fn part_1(&self, input: &str) -> u64 {
        let battery_bank = BatteryBank::parse(input);

        battery_bank
            .banks
            .iter()
            .map(|bank| bank.largest_voltage(2))
            .sum()
    }
    fn part_2(&self, input: &str) -> u64 {
        let battery_bank = BatteryBank::parse(input);

        battery_bank
            .banks
            .iter()
            .map(|bank| bank.largest_voltage(12))
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
