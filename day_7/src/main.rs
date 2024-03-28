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

#[derive(Debug, Clone, Copy)]
struct CardStat {
    count: u8,
    face: char,
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    hand_array: [char; 5],    // card array ex. KK677
    card_stat: [CardStat; 5], // card count descending ex. 22100
    kind: HandType,
}

fn hand_text_to_hand_struct(hand_line: &str) -> Hand {
    let mut hand = Hand {
        bid: 0,
        hand_array: ['0'; 5],
        card_stat: [CardStat {
            count: 0,
            face: '0',
        }; 5],
        kind: HandType::HighCard,
    };

    let mut cards_count: [u8; 5] = [0; 5];

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
        for i in &mut cards_count {
            if *i == 0 {
                *i = card_count as u8;
                break;
            }
        }
        for i in &mut hand.card_stat {
            if i.count == 0 {
                i.count = card_count as u8;
                i.face = c;
                break;
            }
        }
    }
    cards_count.sort_by(|a, b| b.cmp(a));
    hand.card_stat.sort_by(|a, b| b.count.cmp(&a.count));

    // hand kind
    hand.kind = match cards_count {
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

fn maximize_card_score_with_joker(hand: &Hand) -> HandType {
    let joker_num = hand.hand_array.iter().filter(|&a| *a == 'J').count();
    let mut hightest_non_joker: u8 = 0;
    let mut second_hightest_non_joker: u8 = 0;

    for i in 0..3 {
        if hand.card_stat[i].face == 'J' {
            continue;
        }
        if hightest_non_joker == 0 {
            hightest_non_joker = hand.card_stat[i].count;
        } else if second_hightest_non_joker == 0 {
            second_hightest_non_joker = hand.card_stat[i].count;
        }
    }

    // joker, highest non joker, second highest non joker
    let match_tuple = (joker_num, hightest_non_joker, second_hightest_non_joker);

    match match_tuple {
        (0, ..) => hand.kind,
        (5, ..) => HandType::FiveOfaKind,
        (1, 4, ..) => HandType::FiveOfaKind,
        (2, 3, ..) => HandType::FiveOfaKind,
        (3, 2, ..) => HandType::FiveOfaKind,
        (4, 1, ..) => HandType::FiveOfaKind,
        (1, 3, ..) => HandType::FourOfaKind,
        (2, 2, ..) => HandType::FourOfaKind,
        (3, 1, ..) => HandType::FourOfaKind,
        (1, 2, 2) => HandType::FullHouse,
        (1, 2, ..) => HandType::ThreeOfaKind,
        (2, 1, 1) => HandType::ThreeOfaKind,
        (1, 1, 1) => HandType::OnePair,
        _ => panic!("Above condition should match all {:?}", hand.hand_array),
    }
}

fn calculate_answer(hands: &mut Vec<Hand>, card_order: &HashMap<char, i32>) -> u64 {
    // reverse sort to easily calculate score
    hands.sort_by(|a, b| {
        let mut sort = b.kind.cmp(&a.kind);

        for i in 0..5 {
            let a_score = card_order.get(&a.hand_array[i]).unwrap();
            let b_score = card_order.get(&b.hand_array[i]).unwrap();
            sort = sort.then(a_score.cmp(b_score));
        }
        sort
    });

    let mut score: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let hand_score = (i + 1) as u64 * hand.bid;
        score += hand_score;
    }
    score
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

    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        hands.push(hand_text_to_hand_struct(line));
    }

    let day_1_score = calculate_answer(&mut hands, &day_1_card_cmp);
    println!("day 1 score: {}", day_1_score);

    let mut day_2_card_cmp = day_1_card_cmp.clone();
    day_2_card_cmp.insert('J', 1);

    for hand in &mut hands {
        let new_kind = maximize_card_score_with_joker(hand);

        let mut print_sort = hand.hand_array.clone();
        print_sort.sort();
        if hand.kind != new_kind {
            hand.kind = new_kind;
        }
    }
    let day_2_score = calculate_answer(&mut hands, &day_2_card_cmp);

    println!("day 2 score: {}", day_2_score);
}
