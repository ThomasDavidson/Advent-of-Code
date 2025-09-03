use library::input::Day;
use std::ops::Range;

struct Lookup {
    range: Range<i64>,
    modifier: i64,
}
impl Lookup {
    fn parse(line: &str) -> Option<Self> {
        let nums = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        if nums.len() < 2 {
            return None;
        }

        let dest_start = nums[0];
        let source_start = nums[1];
        let length = nums[2];

        let range = source_start..(source_start + length);

        let modifier = dest_start - source_start;

        Some(Self { range, modifier })
    }
}

struct Map {
    maps: Vec<Lookup>,
}
impl Map {
    fn parse(section: &str) -> Self {
        let lookups = section.lines().filter_map(Lookup::parse).collect();

        Self { maps: lookups }
    }

    fn lookup(&self, seed: i64) -> Option<i64> {
        let result: Vec<_> = self
            .maps
            .iter()
            .filter(|lookup| lookup.range.contains(&seed))
            .collect();

        if result.is_empty() {
            None
        } else if result.len() == 1 {
            Some(seed + result[0].modifier)
        } else {
            panic!("Each value should map to one lookup")
        }
    }
}

struct Almanac {
    maps: Vec<Map>,
    seeds: Vec<i64>,
}
impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut sections = input.split("map:");

        let seed_str = match sections.next() {
            None => panic!("Should not be none"),
            Some(i) => i,
        };

        let seeds: Vec<i64> = seed_str
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        let maps: Vec<Map> = sections.map(Map::parse).collect();

        Self { maps, seeds }
    }

    fn get_seed_location(&self, seed: i64) -> i64 {
        let mut location = seed;
        for map in &self.maps {
            location = map.lookup(location).unwrap_or(location);
        }
        location
    }
}

struct Day5;
const DAY: Day5 = Day5;
impl Day<i64> for Day5 {
    fn part_1(&self, input: &str) -> i64 {
        let almanac = Almanac::parse(input);

        almanac
            .seeds
            .iter()
            .map(|seed| almanac.get_seed_location(*seed))
            .min()
            .unwrap()
    }

    fn part_2(&self, _input: &str) -> i64 {
        todo!()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_map() {
        let seed_to_soil = Map {
            maps: vec![
                Lookup::parse("50 98 2").unwrap(),
                Lookup::parse("52 50 48").unwrap(),
            ],
        };

        // Tests
        let result = seed_to_soil.lookup(98);
        assert_eq!(result, Some(50));

        let Some(result) = seed_to_soil.lookup(79) else {
            panic!()
        };
        assert_eq!(result, 81);

        let result = seed_to_soil.lookup(14).is_none();
        assert_eq!(result, true);

        let Some(result) = seed_to_soil.lookup(55) else {
            panic!()
        };
        assert_eq!(result, 57);

        let result = seed_to_soil.lookup(13).is_none();
        assert_eq!(result, true);
    }
}
