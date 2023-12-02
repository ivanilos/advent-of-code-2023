use std::fs;

const RADIX: u32 = 10;
const DIGITS_WRITTEN: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn spelled_digit_at(chars: &Vec<char>, start: u32) -> u32 {
    let start_usize = start as usize;

    for num in 0..DIGITS_WRITTEN.len() {
        let num_vec: Vec<char> = DIGITS_WRITTEN[num].chars().collect();

        if start_usize + num_vec.len() > chars.len() {
            continue;
        }

        let mut equal = true;
        for i in 0..num_vec.len() {
            if num_vec[i] != chars[start_usize + i] {
                equal = false;
                break;
            }
        }

        if equal {
            return (num as u32) + 1;
        }
    }

    0
}

fn get_first_digit(str: &String) -> u32 {
    let chars = str.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        let char = chars[i];
        if char.is_digit(RADIX) {
            return char.to_digit(RADIX).unwrap();
        }

        let written_digit = spelled_digit_at(&chars, i as u32);
        if written_digit != 0 {
            return written_digit;
        }
    }

    0
}

fn get_last_digit(str: &String) -> u32 {
    let chars = str.chars().collect::<Vec<char>>();

    for i in (0..chars.len()).rev() {
        let char = chars[i];
        if char.is_digit(RADIX) {
            return char.to_digit(RADIX).unwrap();
        }

        let written_digit = spelled_digit_at(&chars, i as u32);
        if written_digit != 0 {
            return written_digit;
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
