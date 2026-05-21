use library::input::{Day, InputType};
use std::fmt;
use std::fmt::Formatter;

fn debug_space(space: &[Vec<bool>]) {
    for y in space {
        for x in y {
            if *x {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    eprintln!();
}
#[derive(Debug)]
struct Present {
    area: [[bool; 3]; 3],
    index: usize,
}
impl Present {
    fn parse(lines: &[&str]) -> Self {
        eprintln!("{:?}", lines);
        let index = lines[0].chars().next().unwrap();
        let index = index as usize - '0' as usize;

        let mut area = [[false; 3]; 3];
        for i in 0..3 {
            let line = &lines[i + 1].chars().map(|c| c == '#').collect::<Vec<_>>();
            for j in 0..3 {
                let space = line[j];
                area[i][j] = space;
            }
        }

        Self { index, area }
    }
    fn place(&self, x: usize, y: usize, rotation: usize, space: &mut [Vec<bool>]) -> bool {
        let (x_iter, y_iter) = match rotation {
            0 => (0, 0),
            1 => (2, 0),
            2 => (2, 2),
            3 => (0, 2),
            _ => panic!(),
        };

        for i in 0..3usize {
            let k = (y_iter - i as i16).unsigned_abs() as usize;
            for j in 0..3usize {
                let l = (x_iter - j as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => self.area[k][l],
                    1 | 3 => self.area[l][k],
                    _ => panic!(),
                };

                if space[y + i][x + j] && cpy {
                    return false;
                }
            }
        }

        for i in 0..3usize {
            let k = (y_iter - i as i16).unsigned_abs() as usize;
            for j in 0..3usize {
                let l = (x_iter - j as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => self.area[k][l],
                    1 | 3 => self.area[l][k],
                    _ => panic!(),
                };

                space[y + i][x + j] |= cpy;
            }
        }
        true
    }
}
impl fmt::Display for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.index)?;
        for line in &self.area {
            for space in line {
                if *space {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
struct Area {
    height: usize,
    width: usize,
    required_shapes: [u8; 6],
}
impl Area {
    fn parse(line: &str) -> Self {
        let (size, required_shapes) = line.split_once(':').unwrap();

        let (height, width) = size.split_once('x').unwrap();
        let height = height.parse().unwrap();
        let width = width.parse().unwrap();

        let required_shapes: Vec<u8> = required_shapes
            .split(' ')
            .filter_map(|str| str.parse().ok())
            .collect();
        let required_shapes = required_shapes.try_into().unwrap();

        Self {
            height,
            width,
            required_shapes,
        }
    }
}

struct Tree {
    presents: Vec<Present>,
    areas: Vec<Area>,
}
impl Tree {
    fn parse(input: &str) -> Self {
        let mut presents: Vec<Present> = Vec::new();
        let mut areas: Vec<Area> = Vec::new();

        let mut temp: Vec<&str> = Vec::new();

        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            if line.len() == 2 {
                temp.push(line);
                for _ in 0..4 {
                    temp.push(lines.next().unwrap());
                }
                let present = Present::parse(&temp);
                presents.push(present);
                temp.clear();
            } else {
                let area = Area::parse(line);
                areas.push(area);
            }
        }

        Self { areas, presents }
    }
    fn solve_area(&self, index: usize) -> bool {
        let area = &self.areas[index];
        let mut space = vec![vec![false; area.height]; area.width];

        self.presents[4].place(0, 0, 1, &mut space);
        debug_space(&space);
        self.presents[4].place(1, 1, 3, &mut space);
        debug_space(&space);

        false
    }
}

struct Day12;
const DAY: Day12 = Day12;

impl Day<u64> for Day12 {
    fn part_1(&self, input: &str) -> u64 {
        let tree = Tree::parse(input);

        tree.solve_area(0);
        0
    }
    fn part_2(&self, input: &str) -> u64 {
        0
    }
}
fn main() -> std::io::Result<()> {
    DAY.run(InputType::Example)
}
