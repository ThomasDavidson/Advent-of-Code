use std::{ops::Rem, time::Instant};

fn hash_algorithm(string: &str) -> usize {
    string
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17).rem(256))
}

fn part_1(strings: &Vec<&str>) -> usize {
    strings
        .iter()
        .fold(0, |acc, string| acc + hash_algorithm(string))
}

fn main() {
    let input = include_str!("../example.txt");

    let strings: Vec<&str> = input.split(",").collect();

    let start: Instant = Instant::now();

    let part_1_answer = part_1(&strings);

}
