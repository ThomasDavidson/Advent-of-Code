#[derive(Debug)]
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

#[derive(Debug)]
struct Apparatus {
    components: Vec<Component>,
}
impl Apparatus {
    fn from_str(input: &str) -> Self {
        let mut new_self = Self {
            components: Vec::new(),
        };

        for line in input.lines() {
            let (component_str, connections) = line.split_once(": ").unwrap();
            let component = Component::from_str(component_str);
            new_self.components.push(component);
        }
        new_self
    }
    fn adjacent_components(&self, component: Component) -> Vec<Component> {
        !todo!()
    }
    fn connected_components(&self, component: Component) -> Vec<Component> {
        !todo!()
    }
    fn remove_wire(&mut self, component1: Component, component2: Component) {
        !todo!()
    }
}

fn main() {
    let input = include_str!("../example.txt");

    let apparatus = Apparatus::from_str(input);
    for c in apparatus.components {
        println!("{:?}", c);
    }
}
