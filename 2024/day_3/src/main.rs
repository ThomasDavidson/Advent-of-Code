use std::time::Instant;

use regex::Regex;

fn try_parse_multiply(segment: &str) -> Option<(u32, u32)> {
    let regex = Regex::new(r"mul\(([0-9]{1,}),([0-9]{1,})\)").unwrap();

    let captures = regex.captures(&segment)?;

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

    Some((a, b))
}

fn part_1(input: &str) -> u32 {
    let split = input.split_inclusive(")");

    let mut part_1_answer = 0;
    for segment in split {
        let Some((a, b)) = try_parse_multiply(segment) else {
            continue;
        };
        part_1_answer += a * b;
    }
    part_1_answer
}

#[derive(Debug)]
struct Instruction {
    name: String,
    arguments: Vec<u32>,
}

fn try_parse_instruction(
    str: &str,
    instruction_name: &str,
    num_arguments: u32,
) -> Option<Instruction> {
    let argument_regex = match num_arguments {
        0 => String::new(),
        2 => format!("([0-9]{{1,}}),([0-9]{{1,}})"),
        _ => panic!(),
    };
    let regex = format!(r"{instruction_name}\({argument_regex}\)");

    let regex = Regex::new(&regex).unwrap();
    if regex.captures_len() != num_arguments as usize + 1 {
        return None;
    }

    let captures = regex.captures(&str)?;

    let arguments: Vec<u32> = if num_arguments > 0 {
        captures
            .iter()
            .filter_map(|capture| {
                let c = capture.unwrap();
                let str = c.as_str();

                str.parse::<u32>().ok()
            })
            .collect()
    } else {
        Vec::new()
    };

    Some(Instruction {
        name: instruction_name.to_string(),
        arguments,
    })
}

fn part_2(input: &str) -> u32 {
    let mut active = true;
    let mut part_2_answer = 0;

    for segment in input.split_inclusive(")") {
        match try_parse_instruction(segment, "don't", 0) {
            None => (),
            Some(_) => {
                active = false;
                continue;
            }
        }

        match try_parse_instruction(segment, "do", 0) {
            None => (),
            Some(_) => {
                active = true;
                continue;
            }
        }

        if active {
            match try_parse_instruction(segment, "mul", 2) {
                None => (),
                Some(instr) => {
                    part_2_answer += instr.arguments[0] * instr.arguments[1];
                    continue;
                }
            }
        }
    }
    part_2_answer
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
