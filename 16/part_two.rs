use std::cmp::max;
use std::fs;

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

const DIRS: usize = 4;

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn get_new_positions(x: i32, y: i32, dir: usize, tile: char) -> Vec<(i32, i32, usize)> {
    if tile == '.' {
        return vec![(x + DX[dir], y + DY[dir], dir)];
    } else if tile == '-' {
        if dir == RIGHT || dir == LEFT {
            return vec![(x + DX[dir], y + DY[dir], dir)];
        } else {
            return vec![
                (x + DX[LEFT], y + DY[LEFT], LEFT),
                (x + DX[RIGHT], y + DY[RIGHT], RIGHT),
            ];
        }
    } else if tile == '|' {
        if dir == UP || dir == DOWN {
            return vec![(x + DX[dir], y + DY[dir], dir)];
        } else {
            return vec![
                (x + DX[UP], y + DY[UP], UP),
                (x + DX[DOWN], y + DY[DOWN], DOWN),
            ];
        }
    } else if tile == '/' {
        match dir {
            UP => return vec![(x + DX[RIGHT], y + DY[RIGHT], RIGHT)],
            RIGHT => return vec![(x + DX[UP], y + DY[UP], UP)],
            DOWN => return vec![(x + DX[LEFT], y + DY[LEFT], LEFT)],
            LEFT => return vec![(x + DX[DOWN], y + DY[DOWN], DOWN)],
            _ => panic!("invalid dir"),
        }
    } else {
        // tile == '\'
        match dir {
            UP => return vec![(x + DX[LEFT], y + DY[LEFT], LEFT)],
            RIGHT => return vec![(x + DX[DOWN], y + DY[DOWN], DOWN)],
            DOWN => return vec![(x + DX[RIGHT], y + DY[RIGHT], RIGHT)],
            LEFT => return vec![(x + DX[UP], y + DY[UP], UP)],
            _ => panic!("invalid dir"),
        }
    }
}

fn is_in(x: i32, y: i32, input: &Vec<Vec<char>>) -> bool {
    return 0 <= x && x < input.len() as i32 && 0 <= y && y < input[0].len() as i32;
}

fn dfs(
    x: usize,
    y: usize,
    dir: usize,
    input: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<Vec<bool>>>,
) {
    energized[x][y][dir] = true;

    let new_positions = get_new_positions(x as i32, y as i32, dir, input[x][y]);

    for pos in new_positions {
        if is_in(pos.0, pos.1, input) && !energized[pos.0 as usize][pos.1 as usize][pos.2] {
            dfs(pos.0 as usize, pos.1 as usize, pos.2, input, energized);
        }
    }
}

fn solve_with_pos_and_direction(sx: usize, sy: usize, s_dir: usize, input: &Vec<Vec<char>>) -> u32 {
    let mut energized: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; DIRS]; input[0].len()]; input.len()];

    dfs(sx, sy, s_dir, &input, &mut energized);

    let mut result = 0;
    for i in 0..(input.len()) {
        for j in 0..(input[i].len()) {
            for dir in 0..DIRS {
                if energized[i][j][dir] {
                    result += 1;
                    break;
                }
            }
        }
    }

    result
}

fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    for i in 0..input.len() {
        result = max(result, solve_with_pos_and_direction(i, 0, RIGHT, &input));
        result = max(
            result,
            solve_with_pos_and_direction(i, input[0].len() - 1, LEFT, &input),
        );
    }

    for j in 0..(input[0].len()) {
        result = max(result, solve_with_pos_and_direction(0, j, DOWN, &input));
        result = max(
            result,
            solve_with_pos_and_direction(input.len() - 1, j, UP, &input),
        );
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
