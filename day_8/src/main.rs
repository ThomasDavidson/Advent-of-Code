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

fn get_day_1_answer(network: &HashMap<[char; 3], Node>, instructions: &Vec<char>) -> u64 {
    let mut curr_node: [char; 3] = ['A'; 3];
    let solution: [char; 3] = ['Z'; 3];
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

fn main() {
    let input = include_str!("../input.txt");

    let split_input: Vec<&str> = input.split("\r\n\r\n").collect();

    let instructions: Vec<char> = get_instructions(split_input.get(0).unwrap());

    let network = create_network(split_input.get(1).unwrap());

    let day_1_answer = get_day_1_answer(&network, &instructions);
    println!("Day 1 Answer: {}", day_1_answer);

}
