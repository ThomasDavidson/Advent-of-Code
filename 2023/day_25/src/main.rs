use core::fmt;
use library::input::{Day, InputType};
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Link {
    original: [Component; 2],
    current: Component,
}
impl Link {
    fn new(source: Component, destination: Component) -> Self {
        Self {
            original: [source, destination],
            current: destination,
        }
    }
}
impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} - {}",
            self.current, self.original[0], self.original[1]
        )
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Component {
    label: [char; 3],
}
impl Component {
    fn from_str(str: &str) -> Self {
        let chars: Vec<char> = str.chars().collect();
        Self {
            label: chars.try_into().unwrap(),
        }
    }
}
impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.label[0], self.label[1], self.label[2])
    }
}

#[derive(Debug, Clone)]
struct Apparatus {
    components: HashMap<Component, Vec<Link>>,
}
impl Apparatus {
    fn from_str(input: &str) -> Self {
        let mut new_self = Self {
            components: HashMap::new(),
        };

        for line in input.lines() {
            let (component_str, connections_str) = line.split_once(": ").unwrap();
            let component = Component::from_str(component_str);
            let connections: Vec<Component> = connections_str
                .split(" ")
                .map(Component::from_str)
                .collect();

            new_self.components.insert(
                component,
                connections
                    .into_iter()
                    .map(|connection| Link::new(component, connection))
                    .collect(),
            );
        }
        // add reverse direction
        for (key, links) in new_self.components.clone() {
            for link in links {
                let new_dest = Link::new(link.current, key);
                match new_self.components.get_mut(&link.current) {
                    Some(link_node) => link_node.push(new_dest),
                    None => {
                        new_self.components.insert(link.current, vec![new_dest]);
                    }
                }
            }
        }
        new_self
    }

    fn get_components(&self) -> Vec<Component> {
        self.clone().components.into_keys().collect()
    }

    fn adjacent_components(&self, component: &Component) -> &Vec<Link> {
        self.components.get(component).unwrap()
    }

    fn adjacent_components_mut(&mut self, component: &Component) -> &mut Vec<Link> {
        self.components.get_mut(component).unwrap()
    }

    fn connected_components(&self, component: &Component) -> Vec<Component> {
        let mut output = vec![component];
        let mut traverse = vec![component];

        while let Some(node) = traverse.pop() {
            let nodes = self.adjacent_components(node);
            for adjacent in nodes {
                if output.iter().any(|links| &&adjacent.current == links) {
                    continue;
                }
                traverse.push(&adjacent.current);
                output.push(&adjacent.current);
            }
        }
        output.into_iter().map(|n| n.to_owned()).collect()
    }

    fn remove_wire(&mut self, component1: &Component, component2: &Component) {
        {
            let connection1 = self.adjacent_components_mut(component1);
            if let Some(pos1) = connection1.iter().position(|c| c.current == *component2) {
                connection1.remove(pos1);
            };
        }

        {
            let connection2 = self.adjacent_components_mut(component2);
            if let Some(pos2) = connection2.iter().position(|c| c.current == *component1) {
                connection2.remove(pos2);
            };
        }
    }
    fn add_wire(
        &mut self,
        component1: &Component,
        component2: &Component,
        original: [Component; 2],
    ) {
        {
            let connection1 = self.adjacent_components_mut(component1);
            connection1.push(Link {
                original,
                current: component2.to_owned(),
            });
        }

        {
            let connection2 = self.adjacent_components_mut(component2);
            connection2.push(Link {
                original,
                current: component1.to_owned(),
            });
        }
    }

    fn merge_components(&mut self, source: &Component, desination: &Component) {
        // move connections to destination to source
        for adjacent_source in self.adjacent_components(source).to_owned() {
            self.remove_wire(&adjacent_source.current, source);
            if adjacent_source.current != *desination {
                self.add_wire(
                    &adjacent_source.current,
                    desination,
                    adjacent_source.original,
                );
            }
        }
    }
}

impl fmt::Display for Apparatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (source, destinations) in self.components.iter() {
            if destinations.is_empty() {
                continue;
            }

            write!(f, "{}:", source)?;
            for destination in destinations {
                write!(f, " {}", destination)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Day25;
const DAY: Day25 = Day25;
impl Day<usize> for Day25 {
    fn part_1(&self, input: &str) -> usize {
        // process input
        let orig_apparatus = Apparatus::from_str(input);

        let mut rng = rand::rng();

        for _i in 0..100 {
            let mut apparatus = orig_apparatus.clone();
            let mut components: Vec<Component> = apparatus.get_components();

            for _j in 0..(apparatus.components.len() - 2) {
                components.shuffle(&mut rng);
                // get random node
                let source = components.pop().unwrap();
                // get random connected node
                let destination = {
                    let adjacent_source = apparatus.adjacent_components(&source);
                    // random index
                    let Some(destination) = adjacent_source.choose(&mut rng) else {
                        panic!("None adjacent")
                    };
                    destination
                }
                .to_owned();

                // combine it with random connected code
                apparatus.merge_components(&source, &destination.current);
            }

            // get 3 remaining nodes
            let remaining_nodes: Vec<(Component, Vec<Link>)> = apparatus
                .components
                .into_iter()
                .filter(|c| !c.1.is_empty())
                .collect();
            // check original graph with leftover connections cut
            if remaining_nodes.iter().any(|node| node.1.len() != 3) {
                continue;
            }
            // check if remaining nodes have 3 connections

            let mut test_apparatus = orig_apparatus.clone();

            // get node from each side of the cut
            let mut test_nodes: Option<[Component; 2]> = None;

            for node in &remaining_nodes {
                for n in &node.1 {
                    if test_nodes.is_none() {
                        test_nodes = Some(n.original);
                    }
                    test_apparatus.remove_wire(&n.original[0], &n.original[1]);
                }
            }

            let test_nodes = test_nodes.unwrap();

            let side_1_count = test_apparatus.connected_components(&test_nodes[0]).len();
            let side_2_count = test_apparatus.connected_components(&test_nodes[1]).len();

            if side_1_count != side_2_count {
                return side_1_count * side_2_count;
            }
        }
        panic!("Did not find answer")
    }
    fn part_2(&mut self, _input: &str) -> usize {
        0
    }
}

fn main() -> std::io::Result<()> {
    DAY.clone().run(InputType::UserInput)
}
