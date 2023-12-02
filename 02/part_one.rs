use std::fs;

const RADIX: u32 = 10;

const BALLS: &[u32] = &[12, 13, 14];
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

fn solve(input: Vec<String>) -> i32 {
    let mut result = 0i32;

    for (idx, line) in input.iter().enumerate() {
        let games: Vec<Vec<Ball>> = parse_line(line);

        let mut good = true;
        for game in games {
            for balls in game {
                if balls.count > BALLS[balls.color as usize] {
                    good = false;
                    break;
                }
            }
        }
        if good {
            result += (idx as i32) + 1;
        }
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
