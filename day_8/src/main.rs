use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: [char; 3],
    right: [char; 3],
}

fn get_instructions(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn create_network(input: &str) -> HashMap<[char; 3], Node> {
    let mut network: HashMap<[char; 3], Node> = HashMap::new();

    for line in input.lines() {
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

        // print!("Inserted: {:?}", node);
        network.insert(parent, node);
    }
    // println!("");

    network
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

    // print!("{}", String::from_iter(curr_node.to_owned().iter()));

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

            // print!(" => {}", String::from_iter(curr_node.to_owned().iter()));

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

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(1, |lcm_so_far, &num| lcm(lcm_so_far, num))
}

fn brute_force_part_2(
    network: &HashMap<[char; 3], Node>,
    instructions: &Vec<char>,
    nodes: &Vec<&[char; 3]>,
) -> u64 {
    let mut day_2_current_nodes = nodes.clone();
    let mut day_2_answer: u64 = 0;
    let mut done: bool = false;

    while !done {
        for direction in instructions {
            day_2_answer += 1;
            for curr_node in &mut day_2_current_nodes {
                let node = network.get(&curr_node as &[char; 3]).unwrap();
                *curr_node = match direction {
                    'L' => &node.left,
                    'R' => &node.right,
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

fn main() {
    let input = include_str!("../example3.txt");

    let split_input: Vec<&str> = input.split("\r\n\r\n").collect();

    let instructions: Vec<char> = get_instructions(split_input.get(0).unwrap());

    let network = create_network(split_input.get(1).unwrap());

    let day_1_answer = get_distance_to_z(&network, &instructions, ['A'; 3]);
    println!("Day 1 Answer: {}", day_1_answer);

    let day_2_starting: &Vec<&[char; 3]> = &network.keys().filter(|&a| a[2] == 'A').collect();

    let mut ring_lengths: Vec<u64> = Vec::new();

    for node in day_2_starting {
        ring_lengths.push(get_ring_length(&network, node));
        println!(
            "dis to z: {}",
            get_distance_to_z(&network, &instructions, **node)
        );
    }

    println!("ring_lengths: {:?}", ring_lengths);
    let day_2_answer = lcm_of_vec(&ring_lengths);
    println!("Day 2 Answer: {:?}", day_2_answer);
}
