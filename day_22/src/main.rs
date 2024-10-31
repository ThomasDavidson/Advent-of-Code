use core::fmt;
use std::{collections::HashMap, fmt::Formatter, hash::Hash};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coords3D {
    x: usize,
    y: usize,
    z: usize,
}
impl Coords3D {
    fn from_str(str: &str) -> Self {
        let split: Vec<usize> = str
            .split(",")
            .map(|split_str| split_str.parse::<usize>().unwrap())
            .collect();
        Self {
            x: split[0],
            y: split[1],
            z: split[2],
        }
    }
    fn get_field(&self, axis3d: &Axis3D) -> usize {
        match axis3d {
            Axis3D::X => self.x,
            Axis3D::Y => self.y,
            Axis3D::Z => self.z,
        }
    }
    fn new_from_cross_section(
        axis_coord: usize,
        cross_section_coords: &(usize, usize),
        orientation: &Axis3D,
    ) -> Self {
        let csc_1 = cross_section_coords.0;
        let csc_2 = cross_section_coords.1;
        match orientation {
            Axis3D::X => Coords3D {
                x: axis_coord,
                y: csc_1,
                z: csc_2,
            },
            Axis3D::Y => Coords3D {
                x: csc_1,
                y: axis_coord,
                z: csc_2,
            },
            Axis3D::Z => Coords3D {
                x: csc_1,
                y: csc_2,
                z: axis_coord,
            },
        }
    }
    fn translate(self, offset: i32, axis3d: &Axis3D) -> Self {
        match axis3d {
            Axis3D::X => Coords3D {
                x: (self.x as i32 + offset) as usize,
                ..self
            },
            Axis3D::Y => Coords3D {
                y: (self.y as i32 + offset) as usize,
                ..self
            },
            Axis3D::Z => Coords3D {
                z: (self.z as i32 + offset) as usize,
                ..self
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum AxisOrientation3D {
    XY,
    YZ,
    XZ,
}
impl AxisOrientation3D {
    fn get_other_field(&self, coords3d: &Coords3D) -> usize {
        match self {
            Self::XY => coords3d.z,
            Self::XZ => coords3d.y,
            Self::YZ => coords3d.x,
        }
    }
    fn from_axis(axis: &Axis3D) -> Self {
        match axis {
            Axis3D::X => Self::YZ,
            Axis3D::Y => Self::XZ,
            Axis3D::Z => Self::XY,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Axis3D {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Block {
    orientation: AxisOrientation3D,
    block_parts: Vec<Coords3D>,
}

impl Block {
    fn from_string(line: &str) -> Self {
        let (start, end) = line.split_once("~").unwrap();
        let start_coord = Coords3D::from_str(start);
        let end_coord = Coords3D::from_str(end);

        let x_range = start_coord.x..(end_coord.x + 1);
        let y_range = start_coord.y..(end_coord.y + 1);
        let z_range = start_coord.z..(end_coord.z + 1);

        let orientation: AxisOrientation3D = match (x_range.len(), y_range.len(), z_range.len()) {
            (1, 1, _) => AxisOrientation3D::XY,
            (_, 1, 1) => AxisOrientation3D::YZ,
            (1, _, 1) => AxisOrientation3D::XZ,
            _ => panic!(""),
        };

        let mut block_parts: Vec<Coords3D> = Vec::new();
        for x in x_range.clone() {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let new_block_part = Coords3D { x, y, z };
                    block_parts.push(new_block_part);
                }
            }
        }
        Self {
            orientation,
            block_parts,
        }
    }
    fn lowest_part(&self, axis: Axis3D) -> usize {
        match axis {
            Axis3D::X => {
                self.block_parts
                    .iter()
                    .min_by(|a, b| a.x.cmp(&b.x))
                    .unwrap()
                    .x
            }
            Axis3D::Y => {
                self.block_parts
                    .iter()
                    .min_by(|a, b| a.y.cmp(&b.y))
                    .unwrap()
                    .y
            }
            Axis3D::Z => {
                self.block_parts
                    .iter()
                    .min_by(|a, b| a.z.cmp(&b.z))
                    .unwrap()
                    .z
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SandStack {
    coords: HashMap<Coords3D, usize>,
    limit: Coords3D,
}

impl SandStack {
    fn from_str(input: &str) -> Self {
        let blocks: Vec<Block> = input.lines().map(|line| Block::from_string(line)).collect();

        let mut coords = HashMap::new();
        let mut limit = Coords3D { x: 0, y: 0, z: 0 };

        let mut block_ident: usize = 0;

        for block in blocks {
            block_ident += 1;
            for block_part in block.block_parts {
                limit.x = limit.x.max(block_part.x);
                limit.y = limit.y.max(block_part.y);
                limit.z = limit.z.max(block_part.z);

                coords.insert(block_part, block_ident);
            }
        }

        SandStack { limit, coords }
    }
    fn settle_block(self, ident: usize, axis3d: &Axis3D) -> Self {
        assert!(*axis3d == Axis3D::Z);
        // get blocks
        let block_move: Vec<Coords3D> = self
            .clone()
            .coords
            .into_iter()
            .filter(|(_, block_ident)| *block_ident == ident)
            .map(|bm| bm.0)
            .collect();

        // find first block that this block would colide with
        let cross_section_coords: Vec<(usize, usize)> =
            block_move.iter().map(|bm| (bm.x, bm.y)).collect();

        // a is the axis that is bing moved
        let Some(a_min) = block_move.iter().map(|c| c.get_field(&axis3d)).min() else {
            return self;
        };

        // scan blocks
        let mut supporting_level = 0;
        'outer: for a in (1..a_min).rev() {
            for cross_section_coord in &cross_section_coords {
                let coords = Coords3D::new_from_cross_section(a, cross_section_coord, &axis3d);
                if self.coords.get(&coords).is_some() {
                    supporting_level = a;
                    break 'outer;
                }
            }
        }
        // check if the block is alread settled
        let offset: i32 = supporting_level as i32 - a_min as i32 + 1;

        if offset == 0 {
            return self;
        }

        // remove old blocks and add where they have settled
        let mut new_stack = self.coords;

        for b in &block_move {
            new_stack.remove(b);
        }

        for b in block_move {
            let new_block = b.translate(offset, &axis3d);
            new_stack.insert(new_block, ident);
        }

        Self {
            coords: new_stack,
            limit: self.limit,
        }
    }

    fn settle_blocks(self, axis3d: &Axis3D) -> Self {
        let limit = match axis3d {
            Axis3D::X => self.limit.x,
            Axis3D::Y => self.limit.y,
            Axis3D::Z => self.limit.z,
        };
        let slice_orientation = AxisOrientation3D::from_axis(axis3d);

        let mut settled = self.clone();

        for a in 0..limit {
            let slice = self.get_slice(&slice_orientation, a);

            let mut idents: Vec<usize> = slice.iter().map(|(_coords, ident)| **ident).collect();
            idents.sort();
            idents.dedup();

            for ident in idents {
                settled = settled.settle_block(ident, &axis3d)
            }
        }

        settled
    }

    fn get_slice(&self, axis: &AxisOrientation3D, coord: usize) -> Vec<(&Coords3D, &usize)> {
        self.coords
            .iter()
            .filter(|(coords3d, _ident)| axis.get_other_field(coords3d) == coord)
            .collect()
    }

    fn get_supported_blocks(
        &self,
        axis: AxisOrientation3D,
        coord: usize,
    ) -> Vec<(usize, Vec<usize>)> {
        assert!(axis == AxisOrientation3D::XY);

        let base_slice = self.get_slice(&axis, coord);

        let mut block_support_relation: HashMap<usize, Vec<usize>> = HashMap::new();

        for (base_coord, base_ident) in base_slice {
            let supported_coord = base_coord.clone().translate(1, &Axis3D::Z);

            let mut new_supported_vec: Vec<usize> = Vec::new();

            // check if block supports somthing else
            if let Some(supported_ident) = self.coords.get(&supported_coord) {
                if base_ident != supported_ident {
                    new_supported_vec.push(*supported_ident);
                } else {
                    // skip if block supports itself
                    continue;
                }
            }

            if let Some(supported_vec) = block_support_relation.get_mut(base_ident) {
                supported_vec.append(&mut new_supported_vec);
            } else {
                block_support_relation.insert(*base_ident, new_supported_vec);
            }
        }
        block_support_relation.into_iter().collect()
    }
}

impl fmt::Display for SandStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for z in (0..=self.limit.z).rev() {
            for y in 0..=self.limit.y {
                for x in 0..=self.limit.x {
                    let coord = Coords3D { x, y, z };
                    let ident = self.coords.get(&coord);
                    if let Some(ident) = ident {
                        let p = format!("i: {ident}");
                        write!(f, "{p: <8}")?;
                    } else {
                        write!(f, "{: <8}", '.')?;
                    }
                }
                write!(f, "\t")?;
            }
            writeln!(f, " {z}")?;
        }

        Ok(())
    }
}

// check how many items are being supported by counting unique identiers
fn get_num_supported(supported: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut supported_list: Vec<usize> = supported.iter().map(|r| r.1.clone()).flatten().collect();
    supported_list.sort();
    supported_list.dedup();
    supported_list.len()
}

fn main() {
    let input = include_str!("../example.txt");

    let sand_stack = SandStack::from_str(input);

    let settled_sand_blocks = sand_stack.settle_blocks(&Axis3D::Z);

    println!("{settled_sand_blocks}");

    let supported_each_level: Vec<Vec<(usize, Vec<usize>)>> = (0..(settled_sand_blocks.limit.z))
        .rev()
        .map(|z| settled_sand_blocks.get_supported_blocks(AxisOrientation3D::XY, z))
        .collect();

    // remove reduncant blocks
    // if block is supported by more than one block then add it to the remove list
    // check if removing a block from the block list
    // start with the block that is supporting the least amount of blocks

    let remove_lists: Vec<Vec<(usize, Vec<usize>)>> = supported_each_level.into_iter().collect();

    let mut blocks_removed: usize = 0;
    for mut remove_list in remove_lists {
        let num_supported = get_num_supported(&remove_list);
        // sort so item that supports the least amount gets removed first
        remove_list.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (i, _remove) in remove_list.iter().enumerate() {
            let mut test_remove_list = remove_list.clone();
            test_remove_list.remove(i);
            if num_supported == get_num_supported(&test_remove_list) {
                blocks_removed += 1;
            }
        }
    }
    println!("Blocks removed: {blocks_removed}");
}
