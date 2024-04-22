use std::char;

#[derive(Debug)]
struct Valley {
    land: Vec<char>,
}
// -> Vec<Valley>
fn parse_input(input: &str) -> Vec<Valley> {
    let lines: Vec<&str> = input.lines().collect();

    let mut valleys: Vec<Valley> = Vec::new();

    for valley in lines.split(|&a| a == "") {
        valleys.push(Valley {
            land: valley.iter().flat_map(|&s| s.chars()).collect(),
        });
    }
    valleys
}
fn main() {
    let input = include_str!("../example.txt");

    for line in input.lines() {
        println!("{:?}", line);
    }

    let valleys = parse_input(input);
}
