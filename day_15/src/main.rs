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

fn part_2(strings: &Vec<&str>) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

    for string in strings {
        let (label, instr, focal_length_str): (&str, &str, Option<&str>) =
            match string.split_at(string.chars().position(|a| a == '=' || a == '-').unwrap()) {
                (label, "-") => (label, "-", None),
                (label, instr_lens_comb) => {
                    let (instr, lens) = instr_lens_comb.split_at(1);
                    (label, instr, Some(lens))
                }
            };

        let focal_length = match focal_length_str {
            Some(a) => match a.parse() {
                Ok(a) => a,
                Err(a) => panic!("{:?} {:?}", a, focal_length_str),
            },
            None => 0,
        };

        let label_char_vec: Vec<char> = label.chars().take(2).collect();
        let label_char_array: [char; 2] = label_char_vec.try_into().unwrap();

        let box_num = hash_algorithm(label);

        // println!("{} {} {} {:?}", box_num, label, instr, focal_length);

        match instr {
            "-" => boxes[box_num]
                .iter()
                .position(|&a| a.label == label_char_array)
                .map(|e| boxes[box_num].remove(e))
                .is_some(),
            "=" => {
                match boxes[box_num]
                    .iter()
                    .position(|&a| a.label == label_char_array)
                {
                    None => boxes[box_num].push(Lens {
                        label: label_char_array,
                        focal_length: focal_length,
                    }),
                    Some(a) => boxes[box_num][a].focal_length = focal_length,
                };
                true
            }
            _ => panic!("Invalid instruction"),
        };

    }
    let mut day_2_answer = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (j, lens) in box_.iter().enumerate() {
            let lens_sum = (i + 1) * (j + 1) * lens.focal_length;
            day_2_answer += lens_sum;
            // println!("lens answer: {}", lens_sum);
        }
    }
    day_2_answer
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Lens {
    label: [char; 2],
    focal_length: usize,
}

fn main() {
    let input = include_str!("../input.txt");

    let strings: Vec<&str> = input.split(",").collect();

    let start: Instant = Instant::now();

    let part_1_answer = part_1(&strings);
    let duration = start.elapsed();
    println!("Part 1 anwer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();

    let part_2_answer = part_2(&strings);
    let duration = start.elapsed();
    println!("Part 2 anwer: {}, time: {:?}", part_2_answer, duration);
}
