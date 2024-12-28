use std::{iter, time::Instant};

fn part_1(input: &str) -> u32 {
    let (left_list, right_list): (Vec<&str>, Vec<&str>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect();

    let mut left_list: Vec<u32> = left_list.into_iter().map(|s| s.parse().unwrap()).collect();
    let mut right_list: Vec<u32> = right_list.into_iter().map(|s| s.parse().unwrap()).collect();

    left_list.sort();
    right_list.sort();

    iter::zip(left_list, right_list).fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn part_2(input: &str) -> u32 {
    let (left_list, right_list): (Vec<&str>, Vec<&str>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect();

    let mut left_list: Vec<u32> = left_list.into_iter().map(|s| s.parse().unwrap()).collect();
    let mut right_list: Vec<u32> = right_list.into_iter().map(|s| s.parse().unwrap()).collect();

    left_list.sort();
    right_list.sort();

    let mut part_2_answer: u32 = 0;

    for left in left_list {
        let right_count = right_list.iter().filter(|right|right == &&left).count();

        part_2_answer += right_count as u32 * left;
    }

    part_2_answer
}

fn main() {
    let input: &str = include_str!("../example.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}