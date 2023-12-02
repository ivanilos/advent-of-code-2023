use std::cmp;
use std::fs;

const RADIX: u32 = 10;

const RED: u32 = 0;
const GREEN: u32 = 1;
const BLUE: u32 = 2;

struct Ball {
    count: u32,
    color: u32,
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_game(balls_str: Vec<&str>) -> Vec<Ball> {
    let mut result: Vec<Ball> = vec![];

    for draw in balls_str {
        let trimmed_draw = draw.trim();

        let mut count = 0;
        for char in trimmed_draw.chars() {
            if char.is_digit(RADIX) {
                count = 10 * count + char.to_digit(RADIX).unwrap()
            } else {
                break;
            }
        }

        let color: u32;

        match trimmed_draw.chars().last().unwrap() {
            'd' => color = RED,
            'n' => color = GREEN,
            'e' => color = BLUE,
            _ => todo!(),
        }

        result.push(Ball {
            count: count,
            color: color,
        });
    }

    return result;
}

fn parse_line(line: &String) -> Vec<Vec<Ball>> {
    let splitted_line: Vec<&str> = line.split(":").collect();
    let games: Vec<&str> = splitted_line[1].split(";").collect();

    let mut result: Vec<Vec<Ball>> = vec![];
    for game in games {
        let balls_str: Vec<&str> = game.split(",").collect();

        let balls: Vec<Ball> = parse_game(balls_str);

        result.push(balls)
    }

    return result;
}

fn get_power_set(games: Vec<Vec<Ball>>) -> u32 {
    let mut colors: Vec<u32> = vec![0, 0, 0];

    for game in games {
        for ball in game {
            match ball.color {
                RED => colors[0] = cmp::max(colors[0], ball.count),
                GREEN => colors[1] = cmp::max(colors[1], ball.count),
                BLUE => colors[2] = cmp::max(colors[2], ball.count),
                _ => todo!(),
            }
        }
    }

    colors[0] * colors[1] * colors[2]
}

fn solve(input: Vec<String>) -> u32 {
    let mut result = 0;

    for line in input {
        let games: Vec<Vec<Ball>> = parse_line(&line);

        result += get_power_set(games)
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
