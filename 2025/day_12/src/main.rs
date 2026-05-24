use itertools::Itertools;
use library::grid::Vec2;
use library::input::{Day, InputType};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
enum PresentIndex {
    One(usize),
    Two(usize, usize),
}
#[derive(Debug)]
struct Present {
    area: [[bool; 3]; 3],
    index: PresentIndex,
    space: usize,
}
impl Present {
    fn parse(lines: &[&str]) -> Self {
        let index = lines[0].chars().next().unwrap();
        let index = PresentIndex::One(index as usize - '0' as usize);

        let mut area = [[false; 3]; 3];
        for i in 0..3 {
            let line = &lines[i + 1].chars().map(|c| c == '#').collect::<Vec<_>>();
            for j in 0..3 {
                let space = line[j];
                area[i][j] = space;
            }
        }

        let space = area
            .iter()
            .map(|line| line.iter().map(|t| if *t { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>();

        Self { index, area, space }
    }
}
impl fmt::Display for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}:", self.index)?;
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
    fn solve_area(&self, area: Area, space: Space) -> bool {
        // eprintln!("{:?}", area);

        if area.required_shapes.iter().all(|req| req == &0) {
            eprintln!("{space:?}");
            return true;
        }

        for (present, _) in area
            .required_shapes
            .iter()
            .enumerate()
            .filter(|(_, s)| **s != 0)
        {
            let mut space_enum = Vec2::enumerate(&space.space);
            space_enum.sort_by_key(|(coord, _)| -(coord.y as isize));

            for (coord, _) in space_enum {
                let x = coord.x;
                let y = coord.y;
                if coord.x > area.width - 3 || coord.y > area.height - 3 {
                    continue;
                }

                for rot in 0..4 {
                    let mut area = area.clone();
                    let mut space = space.clone();

                    area.required_shapes[present] -= 1;
                    if !space.place(x, y, rot, &self.presents[present]) {
                        continue;
                    }

                    if self.solve_area(area.clone(), space) {
                        return true;
                    }
                }
            }
        }

        false
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
            1 => (2, 0),
            2 => (2, 2),
            3 => (0, 2),
            _ => panic!(),
        };

        if y != 0 && !(space[(x, y - 1)] || space[(x + 1, y - 1)] || space[(x + 2, y - 1)]) {
            return false;
        }

        for y_offset in 0..3usize {
            let x_present = (y_iter - y_offset as i16).unsigned_abs() as usize;
            for x_offset in 0..3usize {
                let y_present = (x_iter - x_offset as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => present.area[x_present][y_present],
                    1 | 3 => present.area[y_present][x_present],
                    _ => panic!(),
                };

                if space[(x + x_offset, y + y_offset)] && cpy {
                    return false;
                }
            }
        }

        for i in 0..3usize {
            let k = (y_iter - i as i16).unsigned_abs() as usize;
            for j in 0..3usize {
                let l = (x_iter - j as i16).unsigned_abs() as usize;

                let cpy = match rotation {
                    0 | 2 => present.area[k][l],
                    1 | 3 => present.area[l][k],
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

        let mut cache: Vec<(usize, usize, (usize, usize))> = Vec::new();

        let combinations: Vec<(usize, usize)> = (0..tree.presents.len())
            .into_iter()
            .combinations(2)
            .map(|combination| (combination[0], combination[1]))
            .collect();
        let same: Vec<(usize, usize)> = (0..tree.presents.len())
            .zip(0..tree.presents.len())
            .collect();

        for (press1, press2) in combinations.iter().chain(same.iter()) {
            'outer: for height in 3..=5 {
                for width in 3..=5 {
                    let mut required_shapes = [0; 6];
                    required_shapes[*press1] += 1;
                    required_shapes[*press2] += 1;

                    let area = Area {
                        height,
                        width,
                        required_shapes,
                    };
                    let space = Space {
                        space: vec![vec![false; area.width]; area.height],
                    };
                    if tree.solve_area(area, space) {
                        cache.push((*press1, *press2, (width, height)));
                        break 'outer;
                    }
                }
            }
        }
        eprintln!("cache: {:?}", cache);

        let mut total = 0;
        let mut completed = 0;
        for pres in (0..tree.presents.len()).into_iter().combinations(2) {
            total += 1;

            let pres1 = pres[0];
            let pres2 = pres[1];

            let mut required_shapes = [0; 6];
            required_shapes[pres1] += 1;
            required_shapes[pres2] += 1;

            let area = Area {
                height: 3,
                width: 5,
                required_shapes,
            };
            let space = Space {
                space: vec![vec![false; area.width]; area.height],
            };
            if tree.solve_area(area, space) {
                completed += 1;
                eprintln!("true: {area:?}");
            }
        }
        eprintln!("{completed}/{total}");

        let mut part_one_answer = 0;
        for area in &tree.areas[..2] {
            let space = Space {
                space: vec![vec![false; area.width]; area.height],
            };
            if tree.solve_area(*area, space) {
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
    DAY.run(InputType::Example)
}
