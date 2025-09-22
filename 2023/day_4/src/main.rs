use library::input::{Day, InputType};
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

fn get_part_1_score(num_match: u32) -> usize {
    match num_match {
        0 => 0,
        num => 2_usize.pow(num - 1),
    }
}

struct ScratchCard {
    your_numbers: HashSet<i32>,
    winning_numbers: HashSet<i32>,
}
impl ScratchCard {
    fn parse(str: &str) -> Self {
        let cards: &str = match str.split(":").nth(1) {
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

        Self {
            winning_numbers,
            your_numbers,
        }
    }

    fn matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .count()
    }
}

struct Day4;
const DAY: Day4 = Day4;
impl Day<usize> for Day4 {
    fn part_1(&self, str: &str) -> usize {
        let mut part_1_answer: usize = 0;

        for line in str.lines() {
            let scratch_card = ScratchCard::parse(line);

            let matching_numbers = scratch_card.matches();

            if matching_numbers > 0 {
                part_1_answer += get_part_1_score(matching_numbers as u32);
            }
        }

        part_1_answer
    }
    fn part_2(&self, str: &str) -> usize {
        let mut card_count: Vec<usize> = vec![1; str.lines().count()];

        for (i, line) in str.lines().enumerate() {
            let scratch_card = ScratchCard::parse(line);

            let matching_numbers = scratch_card.matches();

            let card_dup = match card_count.get(i) {
                Some(n) => *n,
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

        card_count.iter().sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
