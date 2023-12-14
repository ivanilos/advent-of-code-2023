use std::fs;

fn read_input() -> Vec<Vec<Vec<char>>> {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut grids: Vec<Vec<Vec<char>>> = vec![];
    let mut cur_grid: Vec<Vec<char>> = vec![];
    for line in input {
        if line.is_empty() {
            grids.push(cur_grid);
            cur_grid = vec![];
        } else {
            cur_grid.push(line.chars().collect());
        }
    }

    grids.push(cur_grid);

    grids
}

fn mirror_horizontally(grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for left_center in 0..(grid[0].len() - 1) {
        let mut left = left_center as i32;
        let mut right = (left_center + 1) as i32;

        let mut differences: Vec<(usize, usize)> = vec![];

        while left >= 0 && right < grid[0].len() as i32 {
            for i in 0..(grid.len()) {
                if grid[i][left as usize] != grid[i][right as usize] {
                    differences.push((i, left as usize));
                }
            }
            left -= 1;
            right += 1;
        }

        if differences.len() == 1 {
            result += left_center + 1;
        }
    }

    result
}

fn mirror_vertically(grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for top_center in 0..(grid.len() - 1) {
        let mut top = top_center as i32;
        let mut bottom = (top_center + 1) as i32;

        let mut differences: Vec<(usize, usize)> = vec![];

        while top >= 0 && bottom < grid.len() as i32 {
            for i in 0..(grid[top as usize].len()) {
                if grid[top as usize][i] != grid[bottom as usize][i] {
                    differences.push((top as usize, i));
                }
            }
            top -= 1;
            bottom += 1;
        }

        if differences.len() == 1 {
            result += top_center + 1;
        }
    }

    result
}

fn solve(grids: Vec<Vec<Vec<char>>>) -> usize {
    let mut result = 0;

    for grid in grids {
        result += mirror_horizontally(&grid);
        result += 100 * mirror_vertically(&grid);
    }

    result
}

fn main() {
    let grids = read_input();

    let result = solve(grids);

    println!("{result}");
}
