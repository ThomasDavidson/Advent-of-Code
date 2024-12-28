use std::{fmt, time::Instant};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Content {
    Empty,
    File(usize),
}
impl Content {
    fn format(&self) -> String {
        match self {
            Self::Empty => format!("."),
            Self::File(c) => format!("{c}"),
        }
    }
    fn is_empty(&self) -> bool {
        self == &Self::Empty
    }
}
impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[derive(Debug, Copy, Clone)]
struct File {
    content: Content,
    len: u8,
}

impl File {
    fn new(content: Content, len: u8) -> Self {
        Self { content, len }
    }
}

#[derive(Debug, Clone)]
struct DiskMap {
    layout: Vec<File>,
}
impl DiskMap {
    fn from_input(input: &str) -> Self {
        let layout = input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let Some(num) = c.to_string().parse().ok() else {
                    panic!("Cannot parse number");
                };
                let content = match (i % 2) == 0 {
                    false => Content::Empty,
                    true => {
                        let content = i.div_ceil(2);
                        Content::File(content)
                    }
                };

                File::new(content, num)
            })
            .collect();
        Self { layout }
    }

    fn print(&self) {
        for file in &self.layout {
            for _ in 0..file.len {
                print!("{}", file.content);
            }
        }
        println!();
    }

    fn get_files(&self) -> Vec<(usize, &File)> {
        self.layout
            .iter()
            .enumerate()
            .filter(|(_, file)| !file.content.is_empty())
            // .map(|(i, file)| (i, file.len))
            .collect()
    }

    fn get_empty_spaces(&self) -> Vec<(usize, u8)> {
        self.layout
            .iter()
            .enumerate()
            .filter(|(_, file)| file.content.is_empty())
            .map(|(i, file)| (i, file.len))
            .collect()
    }

    fn move_file_to_empty(&mut self, to: usize, from_file: File) -> Option<File> {
        // println!("Move: {} {} -> {}", from_file.len, from_file.content, to);
        let to_file = &mut self.layout[to];
        if !to_file.content.is_empty() {
            panic!("File should be empty");
        }

        if to_file.len == from_file.len {
            *to_file = from_file;
            return None;
        } else if to_file.len < from_file.len {
            // return remaining segment if file is larger than empty segment
            to_file.content = from_file.content;
            let mut ret = from_file;
            ret.len -= to_file.len;
            return Some(ret);
        } else if to_file.len > from_file.len {
            // Add new empty segment if empty space is larger then file
            let diff = to_file.len - from_file.len;
            to_file.len = from_file.len;
            to_file.content = from_file.content;

            let new_empty = File::new(Content::Empty, diff);
            // println!("Insert: {} {}", new_empty.len, new_empty.content);
            self.layout.insert(to + 1, new_empty);
            return None;
        }

        return None;
    }
    fn pop_file(&mut self) -> File {
        while let Some(file) = self.layout.pop() {
            if file.content.is_empty() {
                continue;
            }
            return file;
        }
        panic!("File Should not be empty")
    }
    fn hash(&self) -> u64 {
        let mut hash = 0;
        let mut pos = 0;
        for f in &self.layout {
            let v: usize = match f.content {
                Content::Empty => {
                    pos += f.len as u64;
                    continue;
                }
                Content::File(v) => v,
            };

            for _ in 0..(f.len) {
                let h = pos * v as u64;
                // println!("{pos} {v} => {h}");
                hash += h;
                pos += 1;
            }
        }
        hash
    }
}

fn part_1(input: &str) -> u64 {
    let mut disk_map = DiskMap::from_input(input);

    let mut i = 0;
    loop {
        if i + 1 >= disk_map.layout.len() {
            break;
        }
        if !disk_map.layout[i].content.is_empty() {
            i += 1;
            continue;
        }

        let from_file = disk_map.pop_file();

        let remainder_file = disk_map.move_file_to_empty(i, from_file);

        match remainder_file {
            None => (),
            Some(f) => disk_map.layout.push(f),
        }

        i += 1;
    }

    disk_map.hash()
}

fn part_2(input: &str) -> u64 {
    let mut disk_map = DiskMap::from_input(input);

    let mut curr_id: Option<usize> = None;

    for i in (1..disk_map.layout.len()).rev() {
        let move_file = disk_map.layout[i].clone();
        let move_id = match move_file.content {
            Content::Empty => continue,
            Content::File(id) => id,
        };

        if curr_id == Some(0) {
            break;
        }

        let id = match curr_id {
            Some(id) => id,
            None => move_id,
        };

        if id != move_id {
            continue;
        }
        curr_id = Some(id - 1);

        let (first, _second) = disk_map.layout.split_at_mut(i);

        let mut fit = first
            .iter()
            .enumerate()
            .filter(|(_i, f)| f.len >= move_file.len && f.content.is_empty());

        let Some((fit_idx, _empty_file)) = fit.next() else {
            continue;
        };

        disk_map.layout[i].content = Content::Empty;
        disk_map.move_file_to_empty(fit_idx, move_file);
    }

    disk_map.hash()
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}
