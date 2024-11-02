use itertools::Itertools;

#[derive(Debug)]
struct Vec3D {
    x: i32,
    y: i32,
    z: i32,
}
impl Vec3D {
    fn from_str(str: &str) -> Self {
        let (x, y, z) = str
            .split(",")
            .map(|str| str.split_whitespace().collect::<String>())
            .map(|str| str.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self { x, y, z }
    }
}

#[derive(Debug)]
struct HailStone {
    position: Vec3D,
    velocity: Vec3D,
}
impl HailStone {
    fn from_line(line: &str) -> Self {
        let (position_str, velocity_str) = line.split_once(" @ ").unwrap();
        let pat = ", ";
        let position = Vec3D::from_str(position_str);
        let velocity = Vec3D::from_str(velocity_str);
        Self { position, velocity }
    }
}

#[derive(Debug)]
struct HailStorm {
    hail_stones: Vec<HailStone>,
}
impl HailStorm {
    fn from_str(input: &str) -> Self {
        let hail_stones: Vec<HailStone> = input
            .lines()
            .map(|line| HailStone::from_line(line))
            .collect();

        Self { hail_stones }
    }
}

fn main() {
    let input = include_str!("../example.txt");
    println!("{input}");

    let storm = HailStorm::from_str(input);

    for hail_stone in storm.hail_stones {
        println!("{:?}", hail_stone);
    }
}
