use std::fs;

const RADIX: u32 = 10;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_first_digit(str: &String) -> u32 {
    for char in str.chars() {
        if char.is_digit(RADIX) {
            return char.to_digit(RADIX).unwrap();
        }
    }

    0
}

fn get_last_digit(str: &String) -> u32 {
    for char in str.chars().rev() {
        if char.is_digit(RADIX) {
            return char.to_digit(RADIX).unwrap();
        }
    }

    0
}

fn solve(input: Vec<String>) -> u32 {
    let mut result = 0;

    for str in input {
        let first_digit = get_first_digit(&str);
        let last_digit = get_last_digit(&str);

        result += 10 * first_digit + last_digit;
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
