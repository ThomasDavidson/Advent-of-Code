use std::collections::HashSet;

fn turn_string_into_number_vect(input: &str) -> HashSet<i32> {
    let mut ret: HashSet<i32> = HashSet::new();
    let str_numbers: std::str::Split<'_, &str> = input.split(" ");

    for str_number in str_numbers {
        match str_number.parse() {
            Ok(num) => ret.insert(num),
            Err(_e) => true,
        };
    }

    ret
}

fn get_day_1_score(num_match: u32) -> usize {
    match num_match {
        0 => 0,
        num => 2_usize.pow(num - 1),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut part_1_answer: usize = 0;

    let mut card_count: Vec<usize> = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let cards: &str = match line.split(":").nth(1) {
            None => panic!("Should not be none"),
            Some(i) => i,
        };

        let mut hands = cards.split("|");

        let winning_hand = match hands.nth(0) {
            None => panic!("winning_hand is none"),
            Some(i) => i,
        };

        let your_hand = match hands.nth(0) {
            None => panic!("your_hand is none"),
            Some(i) => i,
        };

        let winning_numbers: HashSet<i32> = turn_string_into_number_vect(winning_hand);
        let your_numbers: HashSet<i32> = turn_string_into_number_vect(your_hand);

        let matching_numbers = winning_numbers.intersection(&your_numbers).count();

        if matching_numbers > 0 {
            part_1_answer = part_1_answer + get_day_1_score(matching_numbers as u32);
        }

        let card_dup = match card_count.get(i) {
            Some(n) => n.clone(),
            None => 0,
        };

        for j in 0..matching_numbers {
            let index = j + i + 1;
            match card_count.get(index) {
                Some(curr) => card_count[index] = curr + card_dup,
                None => card_count.push(1),
            };
        }
    }
    println!("day 4");
    println!("part 1 answer: {}", part_1_answer);

    let part_2_answer: usize = card_count.iter().sum();
    println!("part 2 answer: {:?}", part_2_answer);
}
