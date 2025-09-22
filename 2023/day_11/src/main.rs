use itertools::Itertools;
use library::grid::Vec2;
use library::input::{Day, InputType};

#[derive(Copy, Clone)]
struct Coords(Vec2<usize>);
impl Coords {
    fn distance(&self, coord: &Self) -> usize {
        let x_diff = self.0.x.abs_diff(coord.0.x);
        let y_diff = self.0.y.abs_diff(coord.0.y);

        x_diff + y_diff
    }
}

struct Space {
    space: Vec<Vec<char>>,
}
impl Space {
    fn parse(input: &str) -> Self {
        let space = input.lines().map(|line| line.chars().collect()).collect();
        Self { space }
    }
    fn calculate_expanded_galaxies(&self, expand_multiplier: usize) -> Vec<Coords> {
        let space_enum: Vec<(Coords, char)> = Vec2::enumerate(&self.space)
            .into_iter()
            .map(|(coord, c)| (Coords(coord), c))
            .collect();

        let mut galaxies: Vec<Coords> = space_enum
            .iter()
            .filter(|&a| a.1 == '#')
            .map(|a| a.0)
            .collect();

        let empty_y = self.find_empty_space_vertical();

        for y in empty_y.iter().rev() {
            // filter for galaxies bellow of the empty space
            for galaxy in galaxies.iter_mut().filter(|a| a.0.y > *y) {
                galaxy.0.y += expand_multiplier - 1;
            }
        }

        let empty_x = self.find_empty_space_horizontal();
        for x in empty_x.iter().rev() {
            // filter for galaxies bellow of the empty space
            for galaxy in galaxies.iter_mut().filter(|a| a.0.x > *x) {
                galaxy.0.x += expand_multiplier - 1;
            }
        }

        galaxies
    }
    fn find_empty_space_vertical(&self) -> Vec<usize> {
        let mut ret: Vec<usize> = self
            .space
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|a| a == &'.'))
            .map(|(i, _)| i)
            .collect();

        ret.sort();
        ret
    }

    fn find_empty_space_horizontal(&self) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();

        // iterate through columns in reverse so the column indexes don't get mixed up when adding columns
        for i in (0..self.space.first().unwrap().len()).rev() {
            let empty_columns: Vec<bool> = self
                .space
                .clone()
                .into_iter()
                .map(|line| line.get(i).unwrap() == &'.')
                .collect();

            let empty_column = empty_columns.iter().all(|&a| a);
            // println!("{}: {:?}", i, empty_column);
            if empty_column {
                ret.push(i);
            }
        }

        ret.sort();

        ret
    }
}

struct Day11;
const DAY: Day11 = Day11;
impl Day<usize> for Day11 {
    fn part_1(&self, input: &str) -> usize {
        let space = Space::parse(input);
        let galaxies = space.calculate_expanded_galaxies(2);

        galaxies
            .iter()
            .combinations(2)
            .fold(0, |acc, comb| acc + comb[0].distance(comb[1]))
    }
    fn part_2(&self, input: &str) -> usize {
        let space = Space::parse(input);
        let galaxies: Vec<Coords> = space.calculate_expanded_galaxies(1000000);

        galaxies
            .iter()
            .combinations(2)
            .fold(0, |acc, comb| acc + comb[0].distance(comb[1]))
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}

#[cfg(test)]
mod tests {
    use crate::Day11;
    use library::input::Day;
    const DAY: Day11 = Day11;

    #[test]
    fn test_part_1() {
        let input = include_str!("../example.txt");

        let res = DAY.part_1(input);

        assert_eq!(res, 374);
    }
    #[test]
    fn test_part_2() {
        let input = include_str!("../example.txt");

        let res = DAY.part_2(input);

        assert_eq!(res, 8410);
    }
}
