use std::time::Instant;

fn get_number_lists(lines: &str) -> Vec<[i64; 3]> {
    let mut maps: Vec<Vec<i64>> = vec![];
    let mut array_maps: Vec<[i64; 3]> = Vec::new();
    let mut array_map: [i64; 3];

    for line in lines.lines() {
        let map: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        if map.len() > 0 {
            maps.push(map);
        }
    }

    // println!("Maps");
    for map in &maps {
        array_map = map.clone().try_into().unwrap();
        array_maps.push(array_map)
    }
    // println!("{:?}", array_maps);

    array_maps
}

fn calculate_map_value(value: i64, maps: &Vec<[i64; 3]>) -> i64 {
    for map in maps {
        let lower_check = map[1];
        let upper_check = lower_check + map[2] - 1;
        let offset = map[0];
        // println!("value: {}, upper: {} lower: {} offset: {}", value, upper_check, lower_check, offset);
        if value <= upper_check && value >= lower_check {
            return value - lower_check + offset;
        }
    }
    value
}

fn get_location_from_seed(seed: i64, maps: &Vec<Vec<[i64; 3]>>) -> i64 {
    let mut res = seed;
    for map in maps {
        // print!(" {}", res);
        res = calculate_map_value(res, map);
    }
    // println!(" {}", res);
    res
}

fn part_1(input: &str) -> i64 {
    let mut sections = input.split("\r\n\r\n");

    let seed_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };

    let seed_nums: Vec<i64> = seed_str
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let seed_map_maps: Vec<Vec<[i64; 3]>> = sections
        .into_iter()
        .map(|section| get_number_lists(section))
        .collect();

    let mut part_1_answer: Vec<i64> = vec![];
    for seed in seed_nums.clone() {
        let location = get_location_from_seed(seed, &seed_map_maps);

        part_1_answer.push(location);
    }

    *part_1_answer.iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut sections = input.split("\r\n\r\n");

    let seed_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };

    let seed_nums: Vec<i64> = seed_str
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let seed_map_maps: Vec<Vec<[i64; 3]>> = sections
        .into_iter()
        .map(|section| get_number_lists(section))
        .collect();

    let day_2_seed_chunk = seed_nums.chunks(2);
    let day_2_seed_chunk_len = day_2_seed_chunk.len();

    let mut part_2_answers: Vec<i64> = vec![];

    for (i, day_2_seed_pair) in day_2_seed_chunk.into_iter().enumerate() {
        println!("{} out of {}", i, day_2_seed_chunk_len);

        let seed_start = match day_2_seed_pair.get(0) {
            None => panic!("0 Should not be none"),
            Some(val) => val.clone(),
        };
        let length = match day_2_seed_pair.get(1) {
            None => panic!("1 Should not be none"),
            Some(val) => val.clone(),
        };

        let seed_range = seed_start..seed_start + length;

        for day_2_seed in seed_range {
            let location = get_location_from_seed(day_2_seed.clone(), &seed_map_maps);
            part_2_answers.push(location);
        }
    }
    *part_2_answers.iter().min().unwrap()
}

fn main() {
    let input: &str = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_map() {
        let seed_to_soil: Vec<[i64; 3]> = [[50, 98, 2], [52, 50, 48]].to_vec();
        let result = calculate_map_value(98, &seed_to_soil);
        assert_eq!(result, 50);
        let result = calculate_map_value(79, &seed_to_soil);
        assert_eq!(result, 81);
        let result = calculate_map_value(14, &seed_to_soil);
        assert_eq!(result, 14);
        let result = calculate_map_value(55, &seed_to_soil);
        assert_eq!(result, 57);
        let result = calculate_map_value(13, &seed_to_soil);
        assert_eq!(result, 13);
    }
}
