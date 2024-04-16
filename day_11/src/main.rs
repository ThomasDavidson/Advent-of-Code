#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
struct Coord {
    x: usize,
    y: usize,
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
    let coords = [
        Coord { x: 4, y: 0 },
        Coord { x: 9, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { y: 6, x: 1 },
        Coord { y: 11, x: 5 },
    ]
    .to_vec();

    let mut coord_list = coords.clone();
    let mut coord: Coord = coord_list.pop().unwrap();

    let res = calculate_closest_coord(&coord, &coord_list);

    println!("1: {:?} 2: {:?}", coord, coord_list.get(res));
}
