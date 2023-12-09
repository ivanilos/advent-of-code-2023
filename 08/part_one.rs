use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node {
    name: String,
    adj: Adj,
}

#[derive(Debug)]
struct Adj {
    left: String,
    right: String,
}

fn read_input() -> (Vec<char>, Vec<Node>) {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let commands = input[0].chars().collect();
    let mut nodes: Vec<Node> = vec![];

    for i in 2..input.len() {
        let line = &input[i];

        let node = Node {
            name: line[0..3].to_string(),
            adj: Adj {
                left: line[7..10].to_string(),
                right: line[12..15].to_string(),
            },
        };

        nodes.push(node);
    }

    (commands, nodes)
}

fn get_adjacents(nodes: &Vec<Node>) -> HashMap<String, &Adj> {
    let mut result: HashMap<String, &Adj> = HashMap::new();
    for node in nodes {
        result.insert(node.name.to_string(), &node.adj);
    }

    result    
}

fn solve(commands: Vec<char>, nodes: Vec<Node>) -> usize {
    let adjacents = get_adjacents(&nodes);

    let len = commands.len();
    let mut next_command_idx = 0usize;
    let mut cur_node = "AAA";

    while cur_node != "ZZZ" {
        match commands[next_command_idx % len] {
            'L' => cur_node = &adjacents[cur_node].left,
            'R' => cur_node = &adjacents[cur_node].right,
            _ => todo!(),
        }
        next_command_idx += 1;
    }

    next_command_idx
}

fn main() {
    let (commands, nodes) = read_input();

    let result = solve(commands, nodes);

    println!("{result}");
}
