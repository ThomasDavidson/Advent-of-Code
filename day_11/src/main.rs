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

fn print_distances(galaxies: &Vec<Coord>) {
    for galaxy in galaxies {
        print!("\t{}, {}", galaxy.x, galaxy.y);
    }
    println!("");

    for coord1 in galaxies {
        print!("{}, {}", coord1.x, coord1.y);
        for coord2 in galaxies {
            let dist = coord1.distance(coord2);
            print!("\t{}", dist);
            if dist == 0 {
                break;
            }
        }
        println!("");
    }
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


    let mut galaxies: Vec<Coord> = Vec::new();

    for (y, line) in expanded_map.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coord { x: x, y: y });
            }
        }
    }

    println!("Galaxiers: {}", galaxies.len());

    // print_distances(&galaxies);

    let mut answer: usize = 0;

    for (i, galaxy1) in galaxies.iter().enumerate(){
        for (j, galaxy2) in galaxies.iter().enumerate() {
            if j >= i {
                break;
            }
            answer += galaxy1.distance(galaxy2);
        }
    }


    println!("Part one asnwer: {}", answer);
}


fn main() {
    let input = include_str!("../example.txt");

    part_1(input);
}
