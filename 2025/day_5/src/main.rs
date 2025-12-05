use library::input::{Day, InputType};
use std::collections::VecDeque;
use std::ops::Range;

type Id = u64;
struct Database {
    fresh_id_ranges: Vec<Range<Id>>,
    ingredient_ids: Vec<Id>,
}
impl Database {
    fn parse(text: &str) -> Self {
        let lines: Vec<_> = text.lines().collect();
        let mut sections = lines.split(|line| line.is_empty());

        let ids = sections.next().unwrap();

        let fresh_id_ranges = ids
            .iter()
            .map(|id| {
                let (start, end) = id.split_once("-").unwrap();

                start.parse().unwrap()..(end.parse::<Id>().unwrap() + 1)
            })
            .collect();

        let ingredients = sections.next().unwrap();
        let ingredient_ids = ingredients
            .iter()
            .filter_map(|arg0: &&str| arg0.parse::<Id>().ok())
            .collect();

        Self {
            fresh_id_ranges,
            ingredient_ids,
        }
    }
    fn count_fresh_ingredients(&self) -> u32 {
        let mut fresh_ingredient_count = 0;
        for ingredient in &self.ingredient_ids {
            if self
                .fresh_id_ranges
                .iter()
                .any(|range| range.contains(ingredient))
            {
                fresh_ingredient_count += 1;
            }
        }

        fresh_ingredient_count
    }

    fn fresh_ids_in_range(&self) -> Id {
        let mut ranges: VecDeque<_> = self.fresh_id_ranges.clone().try_into().unwrap();

        for _ in 0..self.fresh_id_ranges.len() * 2 {
            let range1 = ranges.pop_front().unwrap();

            if let Some(position) = ranges
                .iter()
                .position(|range| range.contains(&range1.start) || range.contains(&range1.end))
            {
                let range2 = ranges.swap_remove_front(position).unwrap();

                let start = range1.start.min(range2.start);
                let end = range1.end.max(range2.end);

                ranges.push_back(start..end);
            } else {
                ranges.push_back(range1)
            }
        }

        ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum::<Id>()
    }
}

struct Day1;
const DAY: Day1 = Day1;
impl Day<Id> for Day1 {
    fn part_1(&self, input: &str) -> Id {
        Database::parse(input).count_fresh_ingredients() as Id
    }
    fn part_2(&self, input: &str) -> Id {
        Database::parse(input).fresh_ids_in_range()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
