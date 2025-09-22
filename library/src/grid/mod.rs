use itertools::Itertools;
use num::{one, zero, One, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectionFilter {
    Forward,
    Turn,
    Stop,
    Backwards,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    None,
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::None => 'o',
        }
    }

    pub fn get_translation<T>(self) -> (T, T)
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            Direction::North => (zero(), -one::<T>()),
            Direction::East => (one(), zero()),
            Direction::South => (zero(), one()),
            Direction::West => (-one::<T>(), zero()),
            Direction::None => (zero(), zero()),
        }
    }
    pub const fn inverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::None => Direction::None,
        }
    }
    pub const fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::None => Direction::None,
        }
    }

    pub const fn left(self) -> Self {
        self.right().inverse()
    }

    pub const ALL: [Direction; 5] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::None,
    ];
    pub const MOVE: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    // should be constant
    pub fn next(&self, filters: Vec<DirectionFilter>) -> Vec<Direction> {
        let mut ret = Vec::with_capacity(filters.len() * 2); // worst case: Turn adds 2

        for f in filters {
            match f {
                DirectionFilter::Forward => {
                    ret.push(*self);
                }
                DirectionFilter::Turn => {
                    ret.push(self.left());
                    ret.push(self.right());
                }
                DirectionFilter::Stop => {
                    ret.push(Direction::None);
                }
                DirectionFilter::Backwards => {
                    ret.push(self.inverse());
                }
            }
        }

        ret
    }

    pub const fn next_fixed(&self, filters: u8) -> [Option<Direction>; 5] {
        // bitmask flags: 0b0001 = Forward, 0b0010 = Turn, etc.
        let mut out = [None; 5];
        let mut i = 0;

        if filters & 0b0001 != 0 {
            out[i] = Some(*self);
            i += 1;
        }
        if filters & 0b0010 != 0 {
            out[i] = Some(self.left());
            i += 1;
            out[i] = Some(self.right());
            i += 1;
        }
        if filters & 0b0100 != 0 {
            out[i] = Some(Direction::None);
            i += 1;
        }
        if filters & 0b1000 != 0 {
            out[i] = Some(self.inverse());
        }

        out
    }
}

impl<T: Zero + One + Clone + Neg<Output = T>> Mul<T> for Direction {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        let (x, y): (T, T) = self.get_translation();

        Vec2 {
            x: x * rhs.clone(),
            y: y * rhs,
        }
    }
}

#[macro_export]
macro_rules! next_directions {
    ($dir:expr, [ $($f:ident),* $(,)? ]) => {
        {
            let mut ret = [Direction::None; 5];
            let mut i = 0;
            $(
                match DirectionFilter::$f {
                    DirectionFilter::Forward => { ret[i] = $dir; i += 1; }
                    DirectionFilter::Turn => {
                        ret[i] = $dir.left(); i += 1;
                        ret[i] = $dir.right(); i += 1;
                    }
                    DirectionFilter::Stop => { ret[i] = Direction::None; i += 1; }
                    DirectionFilter::Backwards => { ret[i] = $dir.inverse(); i += 1; }
                }
            )*
            (ret, i)
        }
    };
}
#[test]
fn test_next() {
    let (dirs, n) = next_directions!(Direction::North, [Forward]);

    assert!(dirs[..n].contains(&Direction::North));
    assert!(!dirs[..n].contains(&Direction::East));
    assert!(!dirs[..n].contains(&Direction::West));
    assert!(!dirs[..n].contains(&Direction::South));
    assert!(!dirs[..n].contains(&Direction::None));
}

#[macro_export]
macro_rules! filter_direction {
    ([ $(DirectionFilter::$f:ident),* $(,)? ]) => {
        {
            let mut filter = 0;

            $(
                match DirectionFilter::$f {
                    DirectionFilter::Forward => { filter+= 0b0001 },
                    DirectionFilter::Turn => { filter += 0b0010; }
                    DirectionFilter::Stop => { filter += 0b0100; }
                    DirectionFilter::Backwards => { filter += 0b1000; }
                }
            )*
            filter
        }
    };
}
#[test]
fn test_filter() {
    let f = filter_direction!([
        DirectionFilter::Forward,
        DirectionFilter::Turn,
        DirectionFilter::Stop,
        DirectionFilter::Backwards
    ]);

    println!("{:?}", f);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridState {
    pub direction: Direction,
    pub coords: UVec2<usize>,
}

impl GridState {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        GridState {
            coords: UVec2::new(x, y),
            direction,
        }
    }
    pub fn check_bounds(&self, width: usize, height: usize) -> bool {
        !self.coords.check_bounds(width, height)
        // let new_coord = match self.coords + self.direction {
        //     Ok(c) => c,
        //     Err(_) => return false,
        // };
        // !new_coord.check_bounds(width, height)
    }
}

// impl Add<Direction> for GridState {
//     type Output = Result<GridState, ()>;
//
//     fn add(self, rhs: Direction) -> Self::Output {
//         let coords = match self.coords + rhs {
//             Ok(c) => c,
//             Err(_) => return Err(()),
//         };
//         Ok(Self { coords, ..self })
//     }
// }

impl Add<Vec2<i32>> for GridState {
    type Output = Result<GridState, ()>;
    fn add(self, rhs: Vec2<i32>) -> Self::Output {
        // print!("{rhs:?} + {self:?}");
        let x = self.coords.x as i32 + rhs.x;
        let y = self.coords.y as i32 + rhs.y;
        // println!("-> {x:?}, {y:?}");
        let Ok(x) = usize::try_from(x) else {
            return Err(());
        };
        let Ok(y) = usize::try_from(y) else {
            return Err(());
        };

        Ok(Self {
            direction: self.direction,
            coords: UVec2 { x, y },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UVec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialOrd> UVec2<T> {
    pub fn new(x: T, y: T) -> Self {
        UVec2 { x, y }
    }
    pub fn check_bounds(&self, width: T, height: T) -> bool {
        width <= self.x || height <= self.y
    }
}

impl Add<Direction> for UVec2<usize> {
    type Output = Result<UVec2<usize>, &'static str>;

    fn add(self, direction: Direction) -> Self::Output {
        let Ok(x) = isize::try_from(self.x) else {
            return Err("error");
        };
        let Ok(y) = isize::try_from(self.y) else {
            return Err("error");
        };

        let (offset_x, offset_y): (isize, isize) = Direction::get_translation(direction);

        let Ok(new_x) = usize::try_from(x + offset_x) else {
            return Err("error");
        };
        let Ok(new_y) = usize::try_from(y + offset_y) else {
            return Err("error");
        };

        let result = Self { x: new_x, y: new_y };

        Ok(result)
    }
}

pub type Coord = UVec2<usize>;

pub fn find_in_coord<T>(map: &[Vec<T>], find: &T) -> Vec<Coord>
where
    T: PartialEq<T>,
{
    let found: Vec<(Coord, &T)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, needle)| (Coord::new(x, y), needle))
                .collect::<Vec<(Coord, &T)>>()
        })
        .collect();

    found
        .iter()
        .filter(|(_, needle)| needle == &find)
        .map(|(coord, _)| *coord)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialOrd> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
    pub fn check_bounds(&self, width: T, height: T) -> bool {
        width <= self.x || height <= self.y
    }
}

impl<T: Neg<Output = T> + Add + Zero + One> Add<Direction> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, direction: Direction) -> Self::Output {
        let (offset_x, offset_y): (T, T) = Direction::get_translation(direction);

        Self {
            x: self.x + offset_x,
            y: self.y + offset_y,
        }
    }
}
impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Vec2<usize> {
    pub fn enumerate<T: Clone>(two_dim_array: &[Vec<T>]) -> Vec<(Vec2<usize>, T)> {
        two_dim_array
            .iter()
            .enumerate()
            .flat_map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .map(move |(x, v)| (Vec2 { x, y }, v.clone()))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T: FromStr> Vec3<T> {
    pub fn parse(str: &str) -> Option<Self>
    where
        <T as FromStr>::Err: Debug,
    {
        let (x, y, z) = str
            .split(",")
            .map(|str| str.split_whitespace().collect::<String>())
            .map(|str| str.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Some(Self { x, y, z })
    }
}
impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    // Required method
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    // Required method
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<Vec3<i128>> for Vec3<f64> {
    fn from(value: Vec3<i128>) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            z: value.z as f64,
        }
    }
}
