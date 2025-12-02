use library::input::{Day, InputType};

const DIAL_SIZE: u8 = 100;
#[derive(Debug)]
struct Dial {
    position: u8,
}
impl Dial {
    fn new(position: u8) -> Dial {
        Dial { position }
    }

    fn rotate(&mut self, rotation: Rotate) -> Click {
        // count whole rotations of dial
        let mut clicks: u8 = (rotation.to_amount().abs() / DIAL_SIZE as i32) as u8;

        // remove already counted rotations
        let partial_rotation = self.position as i32 + rotation.to_amount() % 100;

        // get new position of dial
        let mut new_position = partial_rotation % DIAL_SIZE as i32;
        new_position += DIAL_SIZE as i32;
        new_position %= DIAL_SIZE as i32;

        // check if the partial rotation caused it to click
        // by checking if it ended on zero or if the modulo operation caused the position to change
        if new_position == 0 || self.position != 0 && (new_position != partial_rotation) {
            clicks += 1;
        }

        self.position = new_position as u8;

        Click {
            part_1: self.position == 0,
            part_2: clicks,
        }
    }
}

#[derive(Debug)]
enum Rotate {
    Left(u32),
    Right(u32),
}
impl Rotate {
    fn parse_line(line: &str) -> Self {
        let amount = line[1..].parse::<u32>().unwrap();

        match line.chars().next().unwrap() {
            'L' => Rotate::Left(amount),
            'R' => Rotate::Right(amount),
            _ => panic!("Invalid rotation direction"),
        }
    }
    fn to_amount(&self) -> i32 {
        match self {
            Self::Left(a) => -(*a as i32),
            Self::Right(a) => *a as i32,
        }
    }
}

#[derive(PartialEq)]
struct Click {
    part_1: bool,
    part_2: u8,
}

struct Day1;
const DAY: Day1 = Day1;
impl Day<u32> for Day1 {
    fn part_1(&self, input: &str) -> u32 {
        let mut part_1_answer = 0;

        let mut dial = Dial::new(50);

        for line in input.lines() {
            let rotate = Rotate::parse_line(line);

            if dial.rotate(rotate).part_1 {
                part_1_answer += 1;
            }
        }

        part_1_answer
    }
    fn part_2(&self, input: &str) -> u32 {
        let mut part_2_answer = 0;

        let mut dial = Dial::new(50);

        for line in input.lines() {
            let rotate = Rotate::parse_line(line);
            part_2_answer += dial.rotate(rotate).part_2 as u32;
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
