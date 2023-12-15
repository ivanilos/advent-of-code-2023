use std::collections::HashMap;
use std::fs;

const CYCLES: u32 = 1000000000;

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..(grid[0].len()) {
        let mut next_free_row = 0;

        while next_free_row < grid.len() && grid[next_free_row][col] != '.' {
            next_free_row += 1
        }

        for row in 0..(grid.len()) {
            if grid[row][col] == 'O' && row > next_free_row {
                grid[row][col] = '.';
                grid[next_free_row][col] = 'O';

                while next_free_row < grid.len() && grid[next_free_row][col] != '.' {
                    next_free_row += 1
                }
            } else if grid[row][col] == '#' {
                next_free_row = row + 1;
                while next_free_row < grid.len() && grid[next_free_row][col] != '.' {
                    next_free_row += 1
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for row in 0..(grid.len()) {
        let mut next_free_col = 0;

        while next_free_col < grid[row].len() && grid[row][next_free_col] != '.' {
            next_free_col += 1
        }

        for col in 0..(grid[row].len()) {
            if grid[row][col] == 'O' && col > next_free_col {
                grid[row][col] = '.';
                grid[row][next_free_col] = 'O';

                while next_free_col < grid.len() && grid[row][next_free_col] != '.' {
                    next_free_col += 1
                }
            } else if grid[row][col] == '#' {
                next_free_col = col + 1;
                while next_free_col < grid.len() && grid[row][next_free_col] != '.' {
                    next_free_col += 1
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    grid.reverse();
    tilt_north(grid);
    grid.reverse();
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for row in 0..(grid.len()) {
        grid[row].reverse();
    }

    tilt_west(grid);

    for row in 0..(grid.len()) {
        grid[row].reverse();
    }
}

fn get_load(grid: Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for i in 0..(grid.len()) {
        for j in 0..(grid[i].len()) {
            if grid[i][j] == 'O' {
                result += grid.len() - i;
            }
        }
    }

    result
}

fn solve(mut grid: Vec<Vec<char>>) -> usize {
    let mut seen_position: HashMap<Vec<Vec<char>>, u32> = HashMap::new();

    let mut period = 0;

    let mut cycle = 0;
    while cycle < CYCLES {
        if seen_position.contains_key(&grid) {
            period = cycle - seen_position[&grid];
            break;
        }

        seen_position.insert(grid.clone(), cycle);

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        cycle += 1;
    }

    while (CYCLES - cycle) % period != 0 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        cycle += 1;
    }

    get_load(grid)
}

fn main() {
    let grid = read_input();

    let result = solve(grid);

    println!("{result}");
}
