use library::input::{Day, InputType};
use library::math::lcm;
use std::collections::HashMap;

struct System {
    instructions: Vec<char>,
    network: HashMap<[char; 3], Node>,
}
impl System {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let instructions: Vec<char> = lines.next().unwrap().chars().collect();

        let mut network: HashMap<[char; 3], Node> = HashMap::new();

        // skip empty line
        lines.next();

        for line in lines {
            let (parent, node) = parse_relation(line);
            network.insert(parent, node);
        }

        Self {
            instructions,
            network,
        }
    }
}

#[derive(Debug)]
struct Node {
    left: [char; 3],
    right: [char; 3],
}

fn parse_relation(line: &str) -> ([char; 3], Node) {
    let chars: Vec<char> = line.chars().collect();

    let parent: [char; 3] = match &chars[0..3].try_into() {
        Ok(a) => *a,
        Err(err) => panic!("{:?}", err),
    };
    let mut node: Node = Node {
        left: ['0'; 3],
        right: ['0'; 3],
    };

    node.left = match &chars[7..10].try_into() {
        Ok(a) => *a,
        Err(err) => panic!("{:?}", err),
    };
    node.right = match &chars[12..15].try_into() {
        Ok(a) => *a,
        Err(err) => panic!("{:?}", err),
    };

    (parent, node)
}

fn get_distance_to_z(
    network: &HashMap<[char; 3], Node>,
    instructions: &Vec<char>,
    starting_node: [char; 3],
) -> u64 {
    let mut curr_node: [char; 3] = starting_node;
    let solution: [char; 3] = match starting_node {
        ['A', 'A', 'A'] => ['Z'; 3],
        _ => {
            let mut sol = starting_node;
            sol[2] = 'Z';
            sol
        }
    };

    let mut day_1_answer: u64 = 0;

    while curr_node != solution {
        for direction in instructions {
            day_1_answer += 1;
            let node = match network.get(&curr_node) {
                Some(a) => a,
                None => return 0,
            };
            curr_node = match direction {
                'L' => node.left,
                'R' => node.right,
                _ => panic!("Above should match all"),
            };

            if curr_node == solution {
                break;
            }
        }
    }
    // println!("");

    day_1_answer
}

fn get_ring_length(network: &HashMap<[char; 3], Node>, node: &[char; 3]) -> u64 {
    let node_ring_1 = network.get(node).unwrap();

    let mut traverse_node: [char; 3] = node_ring_1.left;
    let mut ring_len = 0;

    while ring_len <= 1
        || !(traverse_node == node_ring_1.left || traverse_node == node_ring_1.right)
    {
        traverse_node = network.get(&traverse_node).unwrap().left;
        ring_len += 1;
    }
    ring_len
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(1, |lcm_so_far, &num| lcm(lcm_so_far, num))
}

fn _brute_force_part_2(
    network: &HashMap<[char; 3], Node>,
    instructions: &Vec<char>,
    nodes: &[[char; 3]],
) -> u64 {
    let mut day_2_current_nodes = nodes.to_owned();
    let mut day_2_answer: u64 = 0;
    let mut done: bool = false;

    while !done {
        for direction in instructions {
            day_2_answer += 1;
            for curr_node in &mut day_2_current_nodes {
                let node = network.get(curr_node as &[char; 3]).unwrap();
                *curr_node = match direction {
                    'L' => node.left,
                    'R' => node.right,
                    _ => panic!("Above should match all"),
                };
            }

            if day_2_current_nodes.iter().all(|&a| a[2] == 'Z') {
                done = true;
                break;
            }
        }
    }
    day_2_answer
}

struct Day8;
const DAY: Day8 = Day8;
impl Day<u64> for Day8 {
    fn part_1(&self, input: &str) -> u64 {
        let system = System::parse(input);

        get_distance_to_z(&system.network, &system.instructions, ['A'; 3])
    }
    fn part_2(&self, input: &str) -> u64 {
        let system = System::parse(input);

        let day_2_starting: Vec<[char; 3]> = system
            .network
            .keys()
            .filter(|&a| a[2] == 'A')
            .copied()
            .collect();

        let mut ring_lengths: Vec<u64> = Vec::new();

        for node in &day_2_starting {
            ring_lengths.push(get_ring_length(&system.network, node));
        }
        // add number of instructions
        ring_lengths.push(system.instructions.len() as u64);

        lcm_of_vec(&ring_lengths)
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
