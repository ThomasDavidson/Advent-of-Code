use library::input::{Day, InputType};
use regex::Regex;

#[derive(Debug)]
enum Toggle {
    Do,
    Dont,
}
impl Toggle {
    fn parse(str: &str) -> Option<Self> {
        let Some(t) = str.split_once("(") else {
            return None;
        };

        let num_arguments = t.1.chars().filter(|c| *c == ',').count();

        let instruction_name = if str.contains("don't") {
            "don't"
        } else if str.contains("do") {
            "do"
        } else {
            return None;
        };

        let argument_regex = match num_arguments {
            0 => String::new(),
            1 => "([0-9]{1,}),([0-9]{1,})".to_string(),
            _ => return None,
        };
        let regex = format!(r"{instruction_name}\({argument_regex}\)");

         let regex = Regex::new(&regex).unwrap();
        if regex.captures_len() != num_arguments + 1 {
            return None;
        }

        Some(match instruction_name {
            "do" => Self::Do,
            "don't" => Self::Dont,
            _ => panic!(),
        })
    }
}
struct Multiply(u32, u32);
impl Multiply {
    fn parse(segment: &str) -> Option<Self> {
        let regex = Regex::new(r"mul\(([0-9]{1,}),([0-9]{1,})\)").unwrap();

        let captures = regex.captures(segment)?;

        let len = captures.len();

        if len != 3 {
            panic!(
                "Number of matches does not match expected amount {len}\n{:?}",
                captures
            );
        }

        // will always work since capture group only matches numbers
        let a: u32 = captures[1].parse().unwrap();

        let b: u32 = captures[2].parse().unwrap();

        Some(Self(a, b))
    }

    fn compute(self) -> u32 {
        self.0 * self.1
    }
}

#[derive(Clone)]
struct Day3;
const DAY: Day3 = Day3;
impl Day<u32> for Day3 {
    fn part_1(&self, input: &str) -> u32 {
        input
            .split_inclusive(")")
            .filter_map(Multiply::parse)
            .map(Multiply::compute)
            .sum()
    }
    fn part_2(&mut self, input: &str) -> u32 {
        let mut active = true;
        let mut part_2_answer = 0;

        let input: String = input
            .chars()
            .filter(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
            .collect();

        for segment in input.split_inclusive(')') {
            if let Some(instruction) = Toggle::parse(segment) {
                match instruction {
                    Toggle::Dont => active = false,
                    Toggle::Do => active = true,
                }
            } else if active {
                match Multiply::parse(segment) {
                    None => (),
                    Some(mul) => {
                        part_2_answer += mul.compute();
                        continue;
                    }
                }
            }
        }
        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.clone().run(InputType::UserInput)
}
