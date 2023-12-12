use std::fs;

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

// We solve for each axis separately doing a scanline from lowest coordinate to highest coordinate
// A coordinate will be travelled (cur_galaxies) * (total_galaxies - cur_galaxies) * (space_size)
fn solve(grid: Vec<Vec<char>>) -> u32 {
    let mut x_qt: Vec<u32> = vec![0; grid.len()];
    let mut y_qt: Vec<u32> = vec![0; grid[0].len()];

    let mut x_empty: Vec<u32> = vec![0; grid.len()];
    let mut y_empty: Vec<u32> = vec![0; grid[0].len()];

    let mut galaxies = 0;

    for i in 0..grid.len() {
        for j in 0..(grid[i].len()) {
            if grid[i][j] == '#' {
                x_qt[i] += 1;
                y_qt[j] += 1;
                galaxies += 1;
            }
        }
    }

    for i in 0..grid.len() {
        if x_qt[i] == 0 {
            x_empty[i] = 1;
        }
    }

    for j in 0..(grid[0].len()) {
        if y_qt[j] == 0 {
            y_empty[j] = 1;
        }
    }

    let mut result = 0;
    let mut cur_qt = 0;
    for i in 0..x_qt.len() {
        result += cur_qt * (galaxies - cur_qt) * (1 + x_empty[i]);
        cur_qt += x_qt[i];
    }

    cur_qt = 0;
    for j in 0..y_qt.len() {
        result += cur_qt * (galaxies - cur_qt) * (1 + y_empty[j]);
        cur_qt += y_qt[j];
    }

    result
}

fn main() {
    let grid = read_input();

    let result = solve(grid);

    println!("{result}");
}
