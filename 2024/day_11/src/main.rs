use std::collections::linked_list::Iter;
use std::collections::{HashMap, LinkedList};
use std::time::Instant;

struct Ull<T> {
    list: LinkedList<T>,
}
impl<T> Ull<T> {
    fn new() -> Self {
        let list = LinkedList::<T>::new();
        Self { list }
    }
    fn push_back(&mut self, value: T) {
        self.list.push_back(value);
    }
    fn insert(&mut self, idx: usize, val: T) {
        let l = &mut self.list;

        let mut tail = l.split_off(idx);
        l.push_back(val);
        l.append(&mut tail);
    }

    fn iter(&self) -> Iter<'_, T> {
        self.list.iter()
    }
    fn len(&self) -> usize {
        self.list.len()
    }
    fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.list.iter_mut().nth(idx)
    }
}

#[derive(Debug)]
enum Blink {
    Set(u64),
    Multiply(u64),
    Split,
}
impl Blink {
    fn apply(&self, other: u64) -> (u64, Option<u64>) {
        // println!("{:?}", self);
        let res = match self {
            Self::Set(v) => (*v, None),
            Self::Multiply(v) => (other * v, None),
            Self::Split => {
                let str = format!("{other}");
                let split_at = str.len().div_ceil(2);

                let (left, right) = str.split_at(split_at);

                let (left, right) = (left.parse().unwrap(), right.parse().unwrap());

                (left, Some(right))
            }
        };
        res
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Stone {
    value: u64,
}
impl Stone {
    fn new(value: u64) -> Self {
        Self { value }
    }
    fn blink(&self) -> Blink {
        if self.value == 0 {
            Blink::Set(1)
        } else if format!("{}", self.value).len() % 2 == 0 {
            Blink::Split
        } else {
            Blink::Multiply(2024)
        }
    }

    fn blink_result(&self, count: u32) -> u64 {
        if count == 0 {
            return 0;
        }

        let mut ret = 0;

        let (value, value2) = self.blink().apply(self.value);

        let stone1 = Stone::new(value);
        ret += stone1.blink_result(count - 1);

        if let Some(value2) = value2 {
            ret += 1;
            let stone2 = Stone::new(value2);
            ret += stone2.blink_result(count - 1);
        }

        ret
    }

    fn blink_result_cache(&self, count: u32, cache: &mut HashMap<(Stone, u32), u64>) -> u64 {
        if let Some(cached) = cache.get(&(*self, count)) {
            return *cached;
        }

        if count == 0 {
            return 0;
        }

        let mut ret = 0;

        let (value, value2) = self.blink().apply(self.value);

        let stone1 = Stone::new(value);
        ret += stone1.blink_result_cache(count - 1, cache);

        if let Some(value2) = value2 {
            ret += 1;
            let stone2 = Stone::new(value2);
            ret += stone2.blink_result_cache(count - 1, cache);
        }

        cache.insert((*self, count), ret);

        ret
    }
}

struct Stones {
    line: Vec<Stone>,
}
impl Stones {
    fn from_input(input: &str) -> Self {
        let mut line = Vec::new();

        for str in input.split(" ") {
            let value = str.parse().unwrap();
            let stone = Stone::new(value);

            line.push(stone);
        }

        Self { line }
    }
    fn blink(&self, count: u32) -> u64 {
        let mut res = 0;
        let mut cache = HashMap::new();
        for (_, stone) in self.line.iter().enumerate() {
            res += stone.blink_result_cache(count, &mut cache) + 1;
        }
        res
    }
}

fn part_1(input: &str) -> u64 {
    let stones = Stones::from_input(input);

    stones.blink(25)
}

fn part_2(input: &str) -> u64 {
    let stones = Stones::from_input(input);

    stones.blink(75)
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
