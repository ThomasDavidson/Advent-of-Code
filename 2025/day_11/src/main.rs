use library::input::{Day, InputType};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Index;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Name([char; 3]);
impl Name {
    fn parse(text: &str) -> Self {
        let array = text.chars().collect::<Vec<char>>().try_into().unwrap();
        Self(array)
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for c in self.0.iter() {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Device {
    name: Name,
    connections: Vec<Name>,
}

impl Device {
    fn parse(line: &str) -> Self {
        let (name, connections) = line.split_once(':').unwrap();

        let name = Name::parse(name);
        let connections: Vec<Name> = connections.split_whitespace().map(Name::parse).collect();

        Self { name, connections }
    }
    fn next(&self) -> &[Name] {
        &self.connections
    }
}

struct Reactor {
    devices: Vec<Device>,
}

impl Reactor {
    fn parse(input: &str) -> Self {
        let devices: Vec<Device> = input.lines().map(Device::parse).collect();
        Self { devices }
    }
    fn find(&self, name: &Name) -> &Device {
        self.devices.iter().find(|dev| dev.name == *name).unwrap()
    }
}
impl Index<usize> for Reactor {
    type Output = Device;

    fn index(&self, index: usize) -> &Self::Output {
        &self.devices[index]
    }
}
type Lookup = HashMap<Name, (u64, bool, bool)>;
#[derive(Clone, Debug)]
struct Traverse {
    current: Name,
    history: Vec<Name>,
}
impl Traverse {
    fn new(current: Name) -> Self {
        Self {
            current,
            history: Vec::new(),
        }
    }

    fn next(&self, devices: &Reactor) -> Vec<Self> {
        let current = devices.find(&self.current);

        let next = current
            .next()
            .iter()
            .filter(|device| !self.history.contains(device));

        let mut traversals = Vec::new();
        for name in next {
            let mut traversal = self.clone();
            traversal.history.push(self.current.clone());
            traversal.current = name.clone();
            traversals.push(traversal);
        }

        traversals
    }
    fn next_recursive(
        &self,
        devices: &Reactor,
        lookup: &mut Lookup,
        (name1, name2): (&Name, &Name),
        (mut condition1, mut condition2): (bool, bool),
        end: &Name,
    ) -> u64 {
        if &self.current == name1 {
            condition1 = true;
        }
        if &self.current == name2 {
            condition2 = true;
        }

        if &self.current == end {
            return if condition1 && condition2 { 1 } else { 0 };
        }

        if let Some((result, req1, req2)) = lookup.get(&self.current) {
            if *req1 == condition1 && *req2 == condition2 {
                return *result;
            };
        }

        let mut result = 0;

        for next in self.next(devices) {
            result += next.next_recursive(
                devices,
                lookup,
                (name1, name2),
                (condition1, condition2),
                end,
            );
        }

        lookup.insert(self.current.clone(), (result, condition1, condition2));
        result
    }
}

struct Day11;
const DAY: Day11 = Day11;

impl Day<u64> for Day11 {
    fn part_1(&self, input: &str) -> u64 {
        let devices = Reactor::parse(input);

        let start = Name::parse("you");
        let end = Name::parse("out");

        let mut traversals = vec![Traverse::new(start)];

        let mut end_count: u64 = 0;
        while let Some(traverse) = traversals.pop() {
            for next in traverse.next(&devices) {
                if next.current == end {
                    end_count += 1;
                    continue;
                }
                traversals.push(next);
            }
        }
        end_count
    }
    fn part_2(&self, input: &str) -> u64 {
        let devices = Reactor::parse(input);

        let start = Name::parse("svr");
        let end = Name::parse("out");

        let traverse = Traverse::new(start);

        let mut lookup: Lookup = HashMap::new();

        traverse.next_recursive(
            &devices,
            &mut lookup,
            (&Name::parse("dac"), &Name::parse("fft")),
            (false, false),
            &end,
        )
    }
}
fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
