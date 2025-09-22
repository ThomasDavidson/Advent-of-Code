use library::input::{Day, InputType};
use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;

#[derive(Clone)]
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

    fn reverse(self) -> Lookup {
        let start = self.range.start + self.modifier;
        let end = self.range.end + self.modifier;
        Self {
            modifier: -self.modifier,
            range: start..end,
        }
    }
}

impl fmt::Display for Lookup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}..{}->{}..{}",
            self.range.start,
            self.range.end,
            self.range.start + self.modifier,
            self.range.end + self.modifier
        )
    }
}

#[derive(Clone)]
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

    fn reverse(&self) -> Map {
        let maps = self.clone().maps.into_iter().map(Lookup::reverse).collect();
        Map { maps }
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, lookup) in self.maps.iter().enumerate() {
            write!(f, "{}", lookup)?;
            if i != self.maps.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone)]
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

    fn get_seed_location_layer(&self, seed: i64, layer: usize) -> i64 {
        if layer > self.maps.len() {
            panic!("Layer out of bounds");
        }

        let mut location = seed;
        for map in &self.maps[layer..] {
            location = map.lookup(location).unwrap_or(location);
        }
        location
    }

    fn lowest_lookup(&self) -> Vec<(usize, i64)> {
        let maps: Vec<(usize, &Map)> = self.maps.iter().enumerate().collect();

        let lookups: Vec<(usize, &Lookup)> = maps
            .iter()
            .flat_map(|(i, map)| map.maps.iter().map(move |lookup| (*i, lookup)))
            .collect();

        lookups
            .into_iter()
            .map(|(i, l)| (i, l.range.start + l.modifier))
            .collect()
    }
}

struct ReverseAlmanac(Almanac);
impl ReverseAlmanac {
    fn new(Almanac { seeds, maps }: Almanac) -> ReverseAlmanac {
        let reverse = Almanac {
            seeds,
            maps: maps.iter().rev().map(Map::reverse).collect(),
        };
        ReverseAlmanac(reverse)
    }

    fn get_seed_from_location(&self, location: i64) -> i64 {
        let mut seed = location;

        for map in &self.0.maps {
            seed = map.lookup(seed).unwrap_or(seed);
        }
        seed
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

    fn part_2(&self, input: &str) -> i64 {
        let mut part_2_answer = i64::MAX;

        let almanac = Almanac::parse(input);

        let day_2_seed_chunk = almanac.seeds.chunks(2);
        let day_2_seed_range: Vec<Range<i64>> = day_2_seed_chunk
            .map(|seed_chunk| seed_chunk[0]..(seed_chunk[0] + seed_chunk[1]))
            .collect();

        let low_seed_nums: Vec<(usize, i64)> = day_2_seed_range
            .iter()
            .map(|seed_range| (0, seed_range.start))
            .collect();

        // get the lowest number from each range
        let lowest_lookups: Vec<(usize, i64)> = almanac
            .lowest_lookup()
            .into_iter()
            // increment layer
            .map(|(l, s)| (l + 1, s))
            .collect();

        let reverse = ReverseAlmanac::new(almanac.clone());

        // test location/layer
        for (layer, location) in lowest_lookups.into_iter().chain(low_seed_nums) {
            // get final location
            let final_location = almanac.get_seed_location_layer(location, layer);

            //  keep lower number
            if part_2_answer < final_location {
                continue;
            }

            // check if it represents a valid seed
            let seed = reverse.get_seed_from_location(final_location);

            // check if the seed is part of the input
            if !day_2_seed_range.iter().any(|range| range.contains(&seed)) {
                continue;
            }

            // final check to see if the checking the location from the starting seed is possible
            if almanac.get_seed_location(seed) != final_location {
                continue;
            }

            part_2_answer = final_location;
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
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
    #[test]
    fn check_almanac() {
        let input = include_str!("../example.txt");
        let almanac = Almanac::parse(input);

        for (input, expected) in [(79, 82), (14, 43), (55, 86), (13, 35)] {
            let result = almanac.get_seed_location(input);
            assert_eq!(result, expected);
        }

        let result = almanac.get_seed_location(13);
        assert_eq!(result, 35);

        let result = almanac.get_seed_location(88);
        assert_eq!(result, 52);

        let result = almanac.get_seed_location_layer(13, 0);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(13, 1);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(52, 2);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(41, 3);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(34, 4);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(34, 5);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(35, 6);
        assert_eq!(result, 35);
        let result = almanac.get_seed_location_layer(35, 7);
        assert_eq!(result, 35);
    }

    #[test]
    fn check_reverse() {
        let input = include_str!("../example.txt");
        let almanac = Almanac::parse(input);

        let reverse = ReverseAlmanac::new(almanac);
        let result = reverse.get_seed_from_location(35);
        assert_eq!(result, 13);

        let result = reverse.get_seed_from_location(46);
        assert_eq!(result, 82);
    }
}
