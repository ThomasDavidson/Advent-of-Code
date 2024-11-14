use core::fmt;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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
    components: HashMap<Component, Vec<Component>>,
}
impl Apparatus {
    fn from_str(input: &str, bidirectional_connections: bool) -> Self {
        let mut new_self = Self {
            components: HashMap::new(),
        };

        for line in input.lines() {
            let (component_str, connections_str) = line.split_once(": ").unwrap();
            let component = Component::from_str(component_str);
            let connections: Vec<Component> = connections_str
                .split(" ")
                .map(|connection_str| Component::from_str(connection_str))
                .collect();

            new_self.components.insert(component, connections);
        }
        if bidirectional_connections {
            // add reverse direction
            for (key, links) in new_self.components.clone() {
                for link in links {
                    match new_self.components.get_mut(&link) {
                        Some(link_node) => link_node.push(key.clone()),
                        None => {
                            new_self.components.insert(link, vec![key.clone()]);
                        }
                    }
                }
            }
        }
        new_self
    }

    fn get_components(&self) -> Vec<Component> {
        self.clone().components.into_keys().collect()
    }

    fn adjacent_components(&self, component: &Component) -> &Vec<Component> {
        self.components.get(component).unwrap()
    }

    fn adjacent_components_mut(&mut self, component: &Component) -> &mut Vec<Component> {
        self.components.get_mut(component).unwrap()
    }

    fn connected_components(&self, component: &Component) -> Vec<Component> {
        let mut output = vec![component];
        let mut traverse = vec![component];

        while let Some(node) = traverse.pop() {
            let nodes = self.adjacent_components(node);
            for adjacent in nodes {
                if output.contains(&&adjacent) {
                    continue;
                }
                traverse.push(adjacent);
                output.push(adjacent);
            }
        }
        output.into_iter().map(|n| n.to_owned()).collect()
    }

    fn remove_wire(&mut self, component1: &Component, component2: &Component) {
        {
            let connection1 = self.adjacent_components_mut(&component1);
            let pos1 = connection1.iter().position(|c| c == component2).unwrap();
            connection1.remove(pos1);
        }

        {
            let connection2 = self.adjacent_components_mut(&component2);
            let pos2 = connection2.iter().position(|c| c == component1).unwrap();
            connection2.remove(pos2);
        }
    }
    fn add_wire(&mut self, component1: &Component, component2: &Component) {
        {
            let connection1 = self.adjacent_components_mut(&component1);
            connection1.push(component2.to_owned());
        }

        {
            let connection2 = self.adjacent_components_mut(&component2);
            connection2.push(component1.to_owned());
        }
    }

    fn merge_components(&mut self, source: &Component, desination: &Component) {
        // move connections to destination to source
        for adjacent_source in self.adjacent_components(source).to_owned() {
            self.remove_wire(&adjacent_source, source);
            if adjacent_source != *desination {
                self.add_wire(&adjacent_source, desination);
            }
        }
    }
}

impl fmt::Display for Apparatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (source, destinations) in self.components.iter() {
            if destinations.len() == 0 {
                continue;
            }

            write!(f, "{}:", source)?;
            for desination in destinations {
                write!(f, " {}", desination)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../example.txt");

    // process input
    let apparatus = Apparatus::from_str(input, true);
    println!("{apparatus}");

    let mut rng = rand::thread_rng();

    for _i in 0..100 {
        let mut apparatus = apparatus.clone();
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
            apparatus.merge_components(&source, &destination);
        }
        // check original graph with leftover connections cut
        println!("{apparatus}");

        println!();
    }
}
