fn expand_space_vertical(image: Vec<String>) -> Vec<String> {
    let mut expanded_space: Vec<String> = Vec::new();
    for line in image {
        if line.chars().all(|a| a == '.') {
            expanded_space.push(line.to_string());
        }
        expanded_space.push(line.to_string());
    }

    expanded_space
}

fn expand_space_horizontal(image: Vec<&str>) -> Vec<String> {
    let mut expanded_space: Vec<String> = image
        .clone()
        .into_iter()
        .map(|line| line.to_string())
        .collect();

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
            for line in &mut expanded_space {
                line.insert(i, '.');
            }
        }
    }

    expanded_space
}

fn expand_space(image: Vec<&str>) -> Vec<String> {
    let horizontal_expanded_image = expand_space_horizontal(image);
    expand_space_vertical(horizontal_expanded_image)
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

fn part_1(input: &str) {
    let image: Vec<&str> = input.lines().collect();
    println!(
        "Start width: {} height: {}",
        image.get(0).unwrap().len(),
        image.len()
    );
    let expanded_map = expand_space(image);

    println!(
        "Expanded width: {} height: {}",
        expanded_map.get(0).unwrap().len(),
        expanded_map.len()
    );

    for line in &expanded_map {
        println!("{:?}", line);
    }

    let mut galaxies: Vec<Coord> = Vec::new();

    for (y, line) in expanded_map.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coord { x: x, y: y });
            }
        }
    }

    println!("Galaxiers: {}", galaxies.len());

    print_distances(&galaxies);
}

fn print_distances(galaxies: &Vec<Coord>) {
    for galaxy in galaxies {
        print!("\t{}, {}", galaxy.x, galaxy.y);
    }
    println!("");

    for coord1 in galaxies {
        print!("{}, {}", coord1.x, coord1.y);
        for coord2 in galaxies {
            let dist = get_distance(coord1, coord2);
            print!("\t{}", dist);
            if dist == 0 {
                break;
            }
        }
        println!("");
    }
}

fn get_distance(coord1: &Coord, coord2: &Coord) -> usize {
    let x_diff = coord1.x.abs_diff(coord2.x);
    let y_diff = coord1.y.abs_diff(coord2.y);

    x_diff + y_diff
}

fn calculate_closest_coord(coord: &Coord, coord_list: &Vec<Coord>) -> usize {
    let res = coord_list
        .iter()
        .map(|&a| get_distance(&a, &coord))
        .enumerate()
        .min_by_key(|&(_, item)| item)
        .unwrap();

    res.0
}

fn main() {
    let input = include_str!("../example.txt");

    part_1(input);
}
