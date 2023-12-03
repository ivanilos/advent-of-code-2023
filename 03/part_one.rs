use std::fs;

const RADIX: u32 = 10;

const DX: &[i32] = &[-1, -1, -1, 0, 1, 1, 1, 0];
const DY: &[i32] = &[-1, 0, 1, 1, 1, 0, -1, -1];

fn read_input() -> Vec<Vec<char>> {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result: Vec<Vec<char>> = vec![];
    for line in input {
        result.push(line.chars().collect());
    }

    result
}

fn is_in(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    0 <= i && i < rows && 0 <= j && j < cols
}

fn has_border_symbol(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    for k in 0..(DX.len()) {
        let x: i32 = i + DX[k];
        let y: i32 = j + DY[k];

        if is_in(&grid, x, y)
            && !grid[x as usize][y as usize].is_digit(RADIX)
            && grid[x as usize][y as usize] != '.'
        {
            return true;
        }
    }

    false
}

fn solve(grid: Vec<Vec<char>>) -> u32 {
    let mut result: u32 = 0;

    for (i, row) in grid.iter().enumerate() {
        let mut cur: u32 = 0;
        let mut good = false;

        for (j, chr) in row.iter().enumerate() {
            if chr.is_digit(RADIX) {
                cur = 10 * cur + chr.to_digit(RADIX).unwrap();

                good = good || has_border_symbol(&grid, i as i32, j as i32);
            } else {
                result += if good { cur } else { 0 };

                good = false;
                cur = 0;
            }
        }
        if good {
            result += cur;
        }
    }

    result
}

fn main() {
    let grid = read_input();

    let result = solve(grid);

    println!("{result}");
}
