use std::fs;

const RADIX: u32 = 10;

#[derive(Debug, Copy, Clone)]
struct Mapping {
    destination_range: u64,
    source_range: u64,
    range_len: u64,
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(input: Vec<String>) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let splitted_seeds: Vec<&str> = input[0].split(":").collect();

    let seeds: Vec<u64> = splitted_seeds[1]
        .split_whitespace()
        .map(|x| -> u64 { x.parse().unwrap() })
        .collect();

    let mut mappings: Vec<Vec<Mapping>> = vec![];

    let mut cur_mapping: Vec<Mapping> = vec![];
    for i in 2..input.len() {
        if input[i].is_empty() {
            mappings.push(cur_mapping.to_vec());
            cur_mapping.clear();
        } else if input[i].chars().next().unwrap().is_digit(RADIX) {
            let mapping_vec: Vec<u64> = input[i]
                .split_whitespace()
                .map(|x| -> u64 { x.parse().unwrap() })
                .collect();

            let mapping: Mapping = Mapping {
                destination_range: mapping_vec[0],
                source_range: mapping_vec[1],
                range_len: mapping_vec[2],
            };

            cur_mapping.push(mapping);
        }
    }

    mappings.push(cur_mapping.to_vec());
    cur_mapping.clear();

    return (seeds, mappings);
}

fn solve(input: Vec<String>) -> u64 {
    let (seeds, mappings) = parse_input(input);

    let mut result = std::u64::MAX;

    for seed in seeds {
        let mut cur_idx = seed;

        for next_mapping_maps in 0..mappings.len() {
            for map in &mappings[next_mapping_maps] {
                if map.source_range <= cur_idx && cur_idx <= map.source_range + map.range_len {
                    cur_idx = map.destination_range + cur_idx - map.source_range;
                    break;
                }
            }
        }

        result = std::cmp::min(result, cur_idx);
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
