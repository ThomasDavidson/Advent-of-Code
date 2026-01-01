use itertools::Itertools;
use library::grid::Vec2;
use library::input::{Day, InputType};
use std::collections::{HashMap, HashSet};
use std::ops::Rem;

type Coord = Vec2<usize>;

struct TheaterHitbox {
    hitboxes: HashMap<usize, Hitbox>,
}
impl TheaterHitbox {
    fn is_inside(&self, coord: &Coord) -> bool {
        let Some(hitbox) = &self.hitboxes.get(&coord.y) else {
            return false;
        };

        hitbox.is_inside(coord.x)
    }
    fn from_border(border: &HashSet<Coord>) -> Self {
        let max_y = border.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

        let mut hitboxes = HashMap::with_capacity(max_y);

        let border = border.iter().sorted_by(|a, b| a.y.cmp(&b.y));

        let mut y = 0;
        let mut min_x = usize::MAX;
        let mut max_x = 0;
        for coords in border {
            if coords.y != y {
                if !(max_x == 0) {
                    hitboxes.insert(y, Hitbox { min_x, max_x });
                }
                y = coords.y;
                min_x = coords.x;
                max_x = coords.x;
                continue;
            }
            min_x = coords.x.min(min_x);
            max_x = coords.x.max(max_x);
        }

        Self { hitboxes }
    }
}
struct Hitbox {
    max_x: usize,
    min_x: usize,
}
impl Hitbox {
    fn is_inside(&self, x: usize) -> bool {
        self.min_x <= x && self.max_x >= x || self.min_x > x && self.max_x < x
    }
}

fn create_border(corners: &Vec<Coord>) -> HashSet<Coord> {
    let mut grid: HashSet<Coord> = HashSet::new();

    for i in 0..corners.len() {
        let coord_1 = &corners[i];
        let coord_2 = &corners[(i + 1).rem(corners.len())];

        let y_min = coord_1.y.min(coord_2.y);
        let y_max = coord_1.y.max(coord_2.y);

        let x_min = coord_1.x.min(coord_2.x);
        let x_max = coord_1.x.max(coord_2.x);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                grid.insert(Coord { x, y });
            }
        }
    }

    grid
}

fn check_rectangle(corners: &Vec<Coord>, theater_hitbox: &TheaterHitbox) -> bool {
    for i in 0..corners.len() {
        let coord_1 = &corners[i];
        let coord_2 = &corners[(i + 1).rem(corners.len())];

        let y_min = coord_1.y.min(coord_2.y);
        let y_max = coord_1.y.max(coord_2.y);

        let x_min = coord_1.x.min(coord_2.x);
        let x_max = coord_1.x.max(coord_2.x);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let coord = Coord { x, y };
                if !theater_hitbox.is_inside(&coord) {
                    return false;
                }
            }
        }
    }

    true
}

struct Theater {
    red_tiles: Vec<Coord>,
}

impl Theater {
    fn parse(input: &str) -> Self {
        let red_tiles: Vec<Coord> = input
            .lines()
            .map(|line| line.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .map(|(x, y)| Coord { x, y })
            .collect();

        Self { red_tiles }
    }

    fn create_border(&self) -> HashSet<Coord> {
        create_border(&self.red_tiles)
    }
}

fn rectangle_size(coord1: &Coord, coord2: &Coord) -> u64 {
    let x_diff = (coord1.x.abs_diff(coord2.x) + 1) as u64;
    let y_diff = (coord1.y.abs_diff(coord2.y) + 1) as u64;
    x_diff * y_diff
}

struct Day9;
const DAY: Day9 = Day9;
impl Day<u64> for Day9 {
    fn part_1(&self, input: &str) -> u64 {
        Theater::parse(input)
            .red_tiles
            .iter()
            .combinations(2)
            .map(|tile| rectangle_size(tile[0], tile[1]))
            .max()
            .unwrap()
    }
    fn part_2(&self, input: &str) -> u64 {
        let theater = Theater::parse(input);
        let mut part_2_answer = 0;

        let red_tile_border = theater.create_border();

        let theater_hitbox = TheaterHitbox::from_border(&red_tile_border);

        let combinations = theater
            .red_tiles
            .iter()
            .combinations(2)
            .sorted_by(|b, a| rectangle_size(a[0], a[1]).cmp(&rectangle_size(b[0], b[1])));

        for red_tile in combinations {
            let tile1 = red_tile[0];
            let tile2 = red_tile[1];
            let rect_size = rectangle_size(tile1, tile2);

            if check_rectangle(
                &vec![
                    tile1.to_owned(),
                    Coord::new(tile1.x, tile2.y),
                    tile2.to_owned(),
                    Coord::new(tile2.x, tile1.y),
                ],
                &theater_hitbox,
            ) {
                part_2_answer = part_2_answer.max(rect_size);
                break;
            }
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
