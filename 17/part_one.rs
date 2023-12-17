use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

const DIRS: i32 = 4;

// [UP, RIGHT, DOWN, LEFT]
const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

const RADIX: u32 = 10;

const MAX_STEPS_ON_DIR: i32 = 3;

const INF: u32 = 1000000000;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Node {
    x: i32,
    y: i32,
    dir: i32,
    steps_on_dir: i32,
}

fn read_input() -> Vec<Vec<u32>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            s.chars()
                .map(|chr| -> u32 { chr.to_digit(RADIX).unwrap() })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn is_in(x: i32, y: i32, input: &Vec<Vec<u32>>) -> bool {
    return 0 <= x && x < input.len() as i32 && 0 <= y && y < input[0].len() as i32;
}

fn dijkstra(x: i32, y: i32, input: Vec<Vec<u32>>) -> u32 {
    let mut dist: HashMap<Node, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, Node)>> = BinaryHeap::new();

    dist.insert(
        Node {
            x: x,
            y: y,
            dir: 1,
            steps_on_dir: 0,
        },
        0,
    );
    heap.push(Reverse((
        0,
        Node {
            x: x,
            y: y,
            dir: 1,
            steps_on_dir: 0,
        },
    )));

    while let Some(Reverse((cost, node))) = heap.pop() {
        let Node {
            x,
            y,
            dir,
            steps_on_dir,
        } = node;
        let node_cost = dist.get(&node);

        if cost > *node_cost.unwrap() {
            continue;
        }

        for i in -1..=1 {
            let nx = x + DX[((dir + i + DIRS) % DIRS) as usize];
            let ny = y + DY[((dir + i + DIRS) % DIRS) as usize];
            let n_dir = (dir + i + DIRS) % DIRS;
            let n_steps_on_dir = if i == 0 { steps_on_dir + 1 } else { 1 };

            if n_dir == dir && steps_on_dir == MAX_STEPS_ON_DIR {
                continue;
            }

            if is_in(nx, ny, &input) {
                let n_cost = cost + input[nx as usize][ny as usize];

                let n_node = Node {
                    x: nx,
                    y: ny,
                    dir: n_dir,
                    steps_on_dir: n_steps_on_dir,
                };
                let old_cost = dist.get(&n_node);
                if old_cost == None || *old_cost.unwrap() > n_cost {
                    dist.insert(n_node.clone(), n_cost);
                    heap.push(Reverse((n_cost, n_node)));
                }
            }
        }
    }

    let target_x = input.len() - 1;
    let target_y = input[0].len() - 1;

    let mut result = INF;
    for dir in 0..DIRS {
        for steps_on_dir in 0..=MAX_STEPS_ON_DIR {
            let node = Node {
                x: target_x as i32,
                y: target_y as i32,
                dir: dir as i32,
                steps_on_dir: steps_on_dir as i32,
            };

            let val = dist.get(&node);
            if val != None {
                result = min(result, *val.unwrap());
            }
        }
    }

    result
}

fn solve(input: Vec<Vec<u32>>) -> u32 {
    dijkstra(0, 0, input)
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
