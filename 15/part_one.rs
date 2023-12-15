use std::fs;

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

fn get_hash(step: Vec<char>) -> u32 {
    let mut result = 0;

    for chr in step {
        result += chr as u32;
        result *= 17;
        result %= 256;
    }

    result
}

fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for step in input {
        result += get_hash(step);
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
