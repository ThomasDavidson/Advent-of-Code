use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    FiveOfaKind,
    FourOfaKind,
    FullHouse,
    ThreeOfaKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    hand_array: [char; 5],  // card array ex. KK677
    card_count: [usize; 5], // card count descending ex. 22100
    kind: HandType,
    hand_score: u64,
}

fn hand_text_to_hand_struct(hand_line: &str, card_order: &HashMap<char, i32>) -> Hand {
    let mut hand = Hand {
        bid: 0,
        hand_array: ['0'; 5],
        card_count: [0; 5],
        kind: HandType::HighCard,
        hand_score: 0,
    };

    let split_hand_vec: Vec<&str> = hand_line.split(' ').collect();

    if split_hand_vec.len() != 2 {
        panic!("Hand should only split in 2");
    }
    let split_hand_array: [&str; 2] = split_hand_vec.try_into().unwrap();

    // bid amount
    hand.bid = match &split_hand_array[1].parse::<u64>() {
        Ok(bid) => *bid,
        Err(err) => panic!("{:?}", err),
    };

    // hand char array
    let hand_vec: Vec<char> = split_hand_array[0].chars().collect();
    hand.hand_array = hand_vec.clone().try_into().unwrap();

    // card count
    let mut unique_cards = hand_vec.clone();
    unique_cards.sort();
    // println!("{:?}", unique_cards);
    unique_cards.dedup();
    // println!("{:?}", unique_cards);
    for c in unique_cards {
        // print!("{} ", c);
        let card_count: usize = hand_vec.iter().filter(|&a| *a == c).count();
        // println!("{} {}", c, card_count);
        for i in &mut hand.card_count {
            if *i == 0 {
                *i = card_count;
                break;
            }
        }
    }
    hand.card_count.sort_by(|a, b| b.cmp(a));

    // hashed value of each card
    for i in 0..5 {
        let card_number_value = match card_order.get(&hand.hand_array[i]) {
            Some(a) => *a as u64,
            _ => panic!("Value not matched {}", hand.hand_array[i]),
        };

        hand.hand_score += card_number_value * 10_u64.pow(((4 - i) * 2).try_into().unwrap());
        // print!("{} ", card_number_value);
    }

    // hand kind
    hand.kind = match hand.card_count {
        [5, ..] => HandType::FiveOfaKind,
        [4, ..] => HandType::FourOfaKind,
        [3, 2, ..] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfaKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("Above condition should match all"),
    };
    hand
}

fn maximize_card_score_with_joker(hand: Hand) -> HandType {

    let joker_num = hand.hand_array.iter().filter(|&a| *a == 'J').count();
    println!("Hand: {:?}, Js: {}", hand.hand_array, joker_num);

    match joker_num {
        0 => hand,
        1 => hand,
        2 => hand,
        3 => hand,
        4 => hand,
        5 => hand,
        _ => panic!("Above condition should match all"),
    };

    HandType::FiveOfaKind
    
}

fn main() {
    let input = include_str!("../input.txt");

    let day_1_card_cmp = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    let mut day_1_score: u64 = 0;

    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        hands.push(hand_text_to_hand_struct(line, &day_1_card_cmp));
    }

    // reverse sort to easily calculate score
    hands.sort_by(|a, b| {
        let first = b.kind.cmp(&a.kind);
        let second = a.hand_score.cmp(&b.hand_score);
        first.then(second)
    });

    for (i, hand) in hands.iter().enumerate() {
        let hand_score = (i + 1) as u64 * hand.bid;
        day_1_score += hand_score;
        // println!(
        //     "Sum: {}, Hand Score: {} place: {} Hand: {:?}",
        //     day_1_score,
        //     hand_score,
        //     i + 1,
        //     hand
        // );
    }

    println!("day 1 score: {}", day_1_score);
    
    let mut day_2_card_cmp = day_1_card_cmp.clone();
    day_2_card_cmp.insert('J', 1);
    
    
    for hand in hands {
        // println!("{:?}", hand);
        maximize_card_score_with_joker(hand);
    }

}
