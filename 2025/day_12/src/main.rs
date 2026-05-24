use library::grid::Vec2;
use library::input::{Day, InputType};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Present {
    area: Space,
    width: usize,
    height: usize,
    index: usize,
    occupied: usize,
}
impl Present {
    fn parse(lines: &[&str]) -> Self {
        let index = lines[0].chars().next().unwrap();
        let index = index as usize - '0' as usize;

        let mut space: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
        for i in 0..3 {
            let line = &lines[i + 1].chars().map(|c| c == '#').collect::<Vec<_>>();
            space[i][..3].copy_from_slice(&line[..3]);
        }
        let occupied = space
            .iter()
            .map(|line| line.iter().map(|t| if *t { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>();

        Self {
            index,
            area: Space { space },
            width: 3,
            height: 3,
            occupied,
        }
    }
}
impl fmt::Display for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}:", self.index)?;
        for line in &self.area.space {
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
#[derive(Debug, Copy, Clone)]
struct Area {
    height: usize,
    width: usize,
    required_shapes: [u8; 6],
}
impl Area {
    fn parse(line: &str) -> Self {
        let (size, required_shapes) = line.split_once(':').unwrap();

        let (width, height) = size.split_once('x').unwrap();
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
    fn solve_area(&self, area: Area, space: Space) -> Option<Space> {
        // eprintln!("{:?}", area);

        if area.required_shapes.iter().all(|req| req == &0) {
            eprintln!("{space:?}");
            return Some(space);
        }

        for (present, _) in area
            .required_shapes
            .iter()
            .enumerate()
            .filter(|(_, s)| **s != 0)
        {
            // eprintln!("{space:?}");
            let mut space_enum = Vec2::enumerate(&space.space);
            space_enum.sort_by_key(|(coord, _)| -(coord.y as isize));

            for (coord, _) in space_enum {
                let x = coord.x;
                let y = coord.y;
                if coord.x > area.width - 3 || coord.y > area.height - 3 {
                    continue;
                }

                for rot in 0..4 {
                    let mut area = area;
                    let mut space = space.clone();

                    area.required_shapes[present] -= 1;
                    if !space.place(x, y, rot, &self.presents[present]) {
                        continue;
                    }

                    if let Some(solution) = self.solve_area(area, space) {
                        return Some(solution);
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone)]
struct Space {
    space: Vec<Vec<bool>>,
}
impl Space {
    fn place(&mut self, x: usize, y: usize, rotation: usize, present: &Present) -> bool {
        let space = self;

        let (x_iter, y_iter) = match rotation {
            0 => (0, 0),
            1 => (present.width - 1, 0),
            2 => (present.width - 1, present.height - 1),
            3 => (0, present.height - 1),
            _ => panic!(),
        };

        if y != 0 && !(space[(x, y - 1)] || space[(x + 1, y - 1)] || space[(x + 2, y - 1)]) {
            return false;
        }

        for y_offset in 0..present.height {
            let x_present = (y_iter as i16 - y_offset as i16).unsigned_abs() as usize;
            for x_offset in 0..present.width {
                let y_present = (x_iter as i16 - x_offset as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => present.area[(y_present, x_present)],
                    1 | 3 => present.area[(x_present, y_present)],
                    _ => panic!(),
                };

                if space[(x + x_offset, y + y_offset)] && cpy {
                    return false;
                }
            }
        }

        for i in 0..present.height {
            let k = (y_iter as i16 - i as i16).unsigned_abs() as usize;
            for j in 0..present.width {
                let l = (x_iter as i16 - j as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => present.area[(l, k)],
                    1 | 3 => present.area[(k, l)],
                    _ => panic!(),
                };

                space[(x + j, y + i)] |= cpy;
            }
        }
        true
    }
}
impl fmt::Debug for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, y) in self.space.iter().enumerate().rev() {
            write!(f, "{:?}: ", i)?;
            for x in y {
                if *x {
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

impl Index<(usize, usize)> for Space {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.space[y][x]
    }
}
impl IndexMut<(usize, usize)> for Space {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.space[y][x]
    }
}

struct Day12;
const DAY: Day12 = Day12;

impl Day<u64> for Day12 {
    fn part_1(&self, input: &str) -> u64 {
        let tree = Tree::parse(input);

        let mut part_one_answer = 0;
        for area in tree.areas {
            let total_area = area.width * area.height;
            let required_area = area
                .required_shapes
                .iter()
                .enumerate()
                .map(|(i, s)| tree.presents[i].occupied * *s as usize)
                .sum::<usize>();

            if (required_area as f32 / total_area as f32) < 0.999 {
                part_one_answer += 1;
            }
        }

        part_one_answer
    }
    fn part_2(&self, input: &str) -> u64 {
        0
    }
}
fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
