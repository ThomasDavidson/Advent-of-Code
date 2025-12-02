use library::input::{Day, InputType};
use std::iter;

struct List {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}
impl List {
    fn parse(input: &str) -> Self {
        let (left_list, right_list): (Vec<u32>, Vec<u32>) = input
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .collect();

        Self {
            left_list,
            right_list,
        }
    }
}

struct Day1;
const DAY: Day1 = Day1;
impl Day<u32> for Day1 {
    fn part_1(&self, input: &str) -> u32 {
        let mut list = List::parse(input);

        list.left_list.sort();
        list.right_list.sort();

        iter::zip(list.left_list, list.right_list).fold(0, |acc, (l, r)| acc + l.abs_diff(r))
    }
    fn part_2(&self, input: &str) -> u32 {
        let mut list = List::parse(input);

        list.left_list.sort();
        list.right_list.sort();

        list.left_list
            .into_iter()
            .map(|left| {
                list.right_list
                    .iter()
                    .filter(|right| right == &&left)
                    .count() as u32
                    * left
            })
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
