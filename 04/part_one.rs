use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_numbers(line: String) -> (Vec<u32>, Vec<u32>) {
    let splitted_line: Vec<&str> = line.split(":").collect();
    let numbers: Vec<&str> = splitted_line[1].split('|').collect();

    let winning: Vec<u32> = numbers[0]
        .split_whitespace()
        .map(|x| -> u32 { x.parse().unwrap() })
        .collect();
    let chosen: Vec<u32> = numbers[1]
        .split_whitespace()
        .map(|x| -> u32 { x.parse().unwrap() })
        .collect();

    (winning, chosen)
}

fn solve(input: Vec<String>) -> u32 {
    let mut result = 0;

    for line in input {
        let (winning, chosen) = get_numbers(line);

        let winning_set: HashSet<u32> = HashSet::from_iter(winning.iter().cloned());

        let mut score = 0;
        for number in chosen {
            if winning_set.contains(&number) {
                score = if score == 0 { 1 } else { 2 * score }
            }
        }
        result += score;
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
