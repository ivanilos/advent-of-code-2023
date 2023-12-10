use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn get_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..input.len() {
        for j in 0..(input[i].len()) {
            if input[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn is_connected(input: &Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) -> bool {
    if from.0 == to.0 {
        // horizontal pipe needed
        if from.1 < to.1 {
            return vec!['-', 'J', '7'].contains(&input[to.0][to.1]);
        } else if from.1 > to.1 {
            return vec!['-', 'L', 'F'].contains(&input[to.0][to.1]);
        }
    } else if from.1 == to.1 {
        // vertical pipe needed
        if from.0 < to.0 {
            return vec!['|', 'J', 'L'].contains(&input[to.0][to.1]);
        } else if from.0 > to.0 {
            return vec!['|', '7', 'F'].contains(&input[to.0][to.1]);
        }
    }

    false
}

fn is_in(input: &Vec<Vec<char>>, start_x: i32, start_y: i32) -> bool {
    0 <= start_x
        && start_x < input.len() as i32
        && 0 <= start_y
        && start_y <= input[start_x as usize].len() as i32
}

fn get_next_pos(from: (usize, usize), to: (usize, usize), symbol: char) -> (i32, i32) {
    let delta: Vec<(i32, i32)> = match symbol {
        '-' => vec![(0, -1), (0, 1)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, -1), (1, 0)],
        '|' => vec![(-1, 0), (1, 0)],
        'L' => vec![(-1, 0), (0, 1)],
        'F' => vec![(1, 0), (0, 1)],
        _ => todo!(),
    };

    if to.0 as i32 + delta[0].0 == from.0 as i32 && to.1 as i32 + delta[0].1 == from.1 as i32 {
        return (to.0 as i32 + delta[1].0, to.1 as i32 + delta[1].1);
    } else {
        return (to.0 as i32 + delta[0].0, to.1 as i32 + delta[0].1);
    }
}

fn get_cycle(input: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
    let dx: Vec<i32> = vec![1, 0, -1, 0];
    let dy: Vec<i32> = vec![0, 1, 0, -1];

    for i in 0..4 {
        let nx = start_x as i32 + dx[i];
        let ny = start_y as i32 + dy[i];

        if is_in(input, nx, ny) {
            let mut from = (start_x, start_y);
            let mut to = (nx as usize, ny as usize);

            let mut result: Vec<(usize, usize)> = vec![from];

            while input[to.0][to.1] != 'S' {
                if is_connected(input, from, to) {
                    let next_pos = get_next_pos(from, to, input[to.0][to.1]);

                    if is_in(input, next_pos.0, next_pos.1) {
                        result.push(to);

                        from = to;
                        to = (next_pos.0 as usize, next_pos.1 as usize);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if input[to.0][to.1] == 'S' {
                return result;
            }
        }
    }

    vec![]
}

fn change_start(input: &mut Vec<Vec<char>>, cycle: &Vec<(usize, usize)>) {
    let before = cycle[cycle.len() - 1];
    let after = cycle[1];

    for start_symbol in vec!['-', 'J', '7', '|', 'L', 'F'] {
        input[cycle[0].0][cycle[0].1] = start_symbol;

        if is_connected(input, before, cycle[0]) && is_connected(input, after, cycle[0]) {
            return;
        }
    }

    panic!("start symbol not found");
}

fn get_inside_cycle_empty_squares(input: &mut Vec<Vec<char>>, cycle: Vec<(usize, usize)>) -> u32 {
    let cycle_set: HashSet<(usize, usize)> = HashSet::from_iter(cycle.iter().cloned());

    let mut result = 0;

    for i in 0..input.len() {
        let mut inside = 0;
        let mut last = '|';
        for j in 0..(input[i].len()) {
            if cycle_set.contains(&(i, j)) {
                match input[i][j] {
                    'F' => {
                        if vec!['|', 'F', '7'].contains(&last) {
                            inside = 1 - inside;
                            last = 'F';
                        }
                    }
                    'L' => {
                        if vec!['|', 'L', 'J'].contains(&last) {
                            inside = 1 - inside;
                            last = 'L';
                        }
                    }
                    'J' => {
                        if vec!['|', 'J', 'L'].contains(&last) {
                            inside = 1 - inside;
                            last = 'J';
                        }
                    }
                    '7' => {
                        if vec!['|', '7', 'F'].contains(&last) {
                            inside = 1 - inside;
                            last = '7';
                        }
                    }
                    '|' => {
                        inside = 1 - inside;
                        last = '|';
                    }
                    _ => (),
                }
            } else {
                result += inside;
            }
        }
    }

    result
}

fn solve(input: &mut Vec<Vec<char>>) -> u32 {
    let (start_x, start_y) = get_start(&input);

    let cycle = get_cycle(&input, start_x, start_y);

    change_start(input, &cycle);

    get_inside_cycle_empty_squares(input, cycle)
}

fn main() {
    let mut input = read_input();

    let result = solve(&mut input);

    println!("{result}");
}
