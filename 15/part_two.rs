use std::collections::BTreeMap;
use std::fs;

const RADIX: u32 = 10;

fn read_input() -> Vec<Vec<char>> {
    let line: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let input: Vec<Vec<char>> = line[0]
        .split(',')
        .map(|x| -> Vec<char> { x.chars().collect() })
        .collect();

    input
}

fn get_hash(step: &Vec<char>) -> usize {
    let mut result = 0;

    for chr in step {
        if chr == &'=' || chr == &'-' {
            break;
        }

        result += *chr as u32;
        result *= 17;
        result %= 256;
    }

    result as usize
}

fn solve(input: Vec<Vec<char>>) -> usize {
    let mut boxes: Vec<BTreeMap<Vec<char>, (u32, u32)>> = vec![BTreeMap::new(); 256];
    let mut inserted_in_box: Vec<u32> = vec![0; 256];

    for step in input {
        let box_idx = get_hash(&step);

        let last_symbol = step[step.len() - 1];

        if last_symbol == '-' {
            let label = step[0..step.len() - 1].to_vec();
            boxes[box_idx].remove(&label);
        } else {
            let label = step[0..step.len() - 2].to_vec();
            let focal_length = last_symbol.to_digit(RADIX).unwrap();

            if boxes[box_idx].contains_key(&label) {
                let pos = (boxes[box_idx].get(&label)).unwrap().0;
                boxes[box_idx].insert(label, (pos, focal_length));
            } else {
                let pos = inserted_in_box[box_idx];
                boxes[box_idx].insert(label, (pos, focal_length));
                inserted_in_box[box_idx] += 1;
            }
        }
    }

    let mut result = 0;
    for box_idx in 0..256 {
        let mut lens: Vec<&(u32, u32)> = boxes[box_idx].values().collect();
        lens.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for slot in 0..(lens.len()) {
            result += (box_idx + 1) * (slot + 1) * (lens[slot].1 as usize);
        }
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
