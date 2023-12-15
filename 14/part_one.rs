use std::fs;

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
    tilt_north(&mut grid);

    get_load(grid)
}

fn main() {
    let grid = read_input();

    let result = solve(grid);

    println!("{result}");
}
