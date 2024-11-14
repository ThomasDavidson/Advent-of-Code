use core::fmt;
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
    fn adjacent_components(&self, component: &Component) -> Vec<Component> {
        let mut output = vec![component];
        let mut traverse = vec![component];

        while let Some(node) = traverse.pop() {
            let nodes = self.components.get(node).unwrap();
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

    fn connected_components(&self, component: Component) -> Vec<Component> {
        !todo!()
    }
    fn remove_wire(&mut self, component1: &Component, component2: &Component) {
        {
            let connection1 = self.components.get_mut(&component1).unwrap();
            let pos1 = connection1.iter().position(|c| c == component2).unwrap();
            connection1.remove(pos1);
        }

        {
            let connection2 = self.components.get_mut(&component2).unwrap();
            let pos2 = connection2.iter().position(|c| c == component1).unwrap();
            connection2.remove(pos2);
        }
    }
    fn merge_components(&mut self, component1: &Component, component2: &Component) {
        !todo!()
    }
}

fn main() {
    let input = include_str!("../example.txt");

    // process input
    let apparatus = Apparatus::from_str(input, true);

    let mut rng = rand::thread_rng();

    for _i in 0..100 {
        for _j in 0..(apparatus.components.len() - 2) {
            // get random node

            // combine it with random connected code
        }
        // check original graph with leftover connections cut
    }
}
