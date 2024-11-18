use itertools::Itertools;
use num::{one, zero, One, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
    usize,
};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectionFilter {
    Forward,
    Turn,
    Stop,
    Backwards,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
        let mut ret: Vec<Direction> = Vec::new();
        if filters.contains(&DirectionFilter::Forward) {
            ret.push(*self);
        }
        if filters.contains(&DirectionFilter::Turn) {
            ret.push(self.left());
            ret.push(self.right());
        }
        if filters.contains(&DirectionFilter::Stop) {
            ret.push(Direction::None);
        }
        if filters.contains(&DirectionFilter::Backwards) {
            ret.push(self.inverse());
        }
        ret
    }
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
        let new_coord = match self.coords + self.direction {
            Ok(c) => c,
            Err(_) => return false,
        };
        return !new_coord.check_bounds(width, height);
                }
            }

impl Add<Direction> for GridState {
    type Output = Result<GridState, ()>;

    fn add(self, rhs: Direction) -> Self::Output {
        let coords = match self.coords + rhs {
            Ok(c) => c,
            Err(_) => return Err(()),
        };
        Ok(Self { coords, ..self })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        let result = Self {
            x: self.x + offset_x,
            y: self.y + offset_y,
        };

        result
    }
}
impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Self::Output {
        let result = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T: FromStr> Vec3<T> {
    pub fn from_str(str: &str) -> Option<Self>
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
