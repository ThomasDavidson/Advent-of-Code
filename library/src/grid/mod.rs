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

    pub const fn get_translation(self) -> (i16, i16) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::None => (0, 0),
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
    pub x: usize,
    pub y: usize,
}

impl GridState {
    pub const fn check_bounds(&self, width: usize, height: usize) -> bool {
        match self.direction {
            Direction::South => {
                if self.y + 1 == height {
                    false
                } else {
                    true
                }
            }
            Direction::East => {
                if self.x + 1 == width {
                    false
                } else {
                    true
                }
            }
            Direction::North => {
                if self.y == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::West => {
                if self.x == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::None => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialOrd> Coords<T> {
    pub fn new(x: T, y: T) -> Self {
        Coords { x, y }
    }
    pub fn check_bounds(&self, width: T, height: T) -> bool {
        width <= self.x || height <= self.y
    }
}

impl Add<Direction> for Coords<usize> {
    type Output = Result<Coords<usize>, &'static str>;

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
