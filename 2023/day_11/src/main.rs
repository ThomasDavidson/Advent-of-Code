fn find_empty_space_vertical(image: &Vec<&str>) -> Vec<usize> {
    let mut ret: Vec<usize> = image
        .iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|a| a == '.'))
        .map(|(i, _)| i)
        .collect();

    ret.sort();
    ret
}

fn find_empty_space_horizontal(image: &Vec<&str>) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();

    // iterate through columns in reverse so the column indexes don't get mixed up when adding columns
    for i in (0..image.first().unwrap().len()).rev() {
        let empty_columns: Vec<bool> = image
            .clone()
            .into_iter()
            .map(|line| line.chars().nth(i).unwrap() == '.')
            .collect();

        let empty_column = empty_columns.iter().all(|&a| a == true);
        // println!("{}: {:?}", i, empty_column);
        if empty_column {
            ret.push(i);
        }
    }

    ret.sort();

    ret
}

fn calculate_galaxies(image: Vec<&str>, expand_multiplier: usize) -> Vec<Coord> {
    let mut galaxies: Vec<Coord> = image
        .iter()
        .enumerate()
        .flat_map(|(y, lines)| {
            lines
                .chars()
                .enumerate()
                .map(move |(x, c)| (Coord { x: x, y: y }, c))
        })
        .filter(|&a| a.1 == '#')
        .map(|a| a.0)
        .collect();
    // println!("Galaxies: {:?}", galaxies);

    let empty_y = find_empty_space_vertical(&image);

    for y in empty_y.iter().rev() {
        // print!("y: {}: ", y);
        // filter for galaxies bellow of the empty space
        for galaxy in galaxies.iter_mut().filter(|a| a.y > *y) {
            // print!("{:?} ", galaxy);
            galaxy.y += (expand_multiplier - 1) as usize;
        }
        // println!("");
    }

    let empty_x = find_empty_space_horizontal(&image);
    for x in empty_x.iter().rev() {
        // print!("x: {}: ", x);
        // filter for galaxies bellow of the empty space
        for galaxy in galaxies.iter_mut().filter(|a| a.x > *x) {
            // print!("{:?} ", galaxy);
            galaxy.x += (expand_multiplier - 1) as usize;
        }
        // println!("");
    }

    // println!("Galaxies: {:?}", galaxies);

    galaxies
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn distance(&self, coord: &Coord) -> usize {
        let x_diff = self.x.abs_diff(coord.x);
        let y_diff = self.y.abs_diff(coord.y);

        x_diff + y_diff
    }
}

fn part_1(input: &str) -> usize {
    let image: Vec<&str> = input.lines().collect();
    let galaxies = calculate_galaxies(image, 2);

    let mut answer: usize = 0;

    for (i, galaxy1) in galaxies.iter().enumerate() {
        for (j, galaxy2) in galaxies.iter().enumerate() {
            if j >= i {
                break;
            }
            answer += galaxy1.distance(galaxy2);
        }
    }

    println!("Part one asnwer: {}", answer);
    answer
}

fn part_2(input: &str) -> usize {
    let image: Vec<&str> = input.lines().collect();
    let galaxies = calculate_galaxies(image, 1000000);

    let mut answer: usize = 0;

    for (i, galaxy1) in galaxies.iter().enumerate() {
        for (j, galaxy2) in galaxies.iter().enumerate() {
            if j >= i {
                break;
            }
            answer += galaxy1.distance(galaxy2);
        }
    }

    println!("Part two asnwer: {}", answer);
    answer
}

fn main() {
    let input = include_str!("../input.txt");

    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let input = include_str!("../example.txt");

        let res = part_1(input);

        assert_eq!(res, 374);
    }
    #[test]
    fn test_part_2() {
        let input = include_str!("../example.txt");

        let res = part_2(input);

        assert_eq!(res, 8410);
    }
}
