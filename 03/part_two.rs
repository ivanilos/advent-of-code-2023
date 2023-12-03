use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const RADIX: u32 = 10;
const NEEDED_ADJ: usize = 2;

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

fn asterisk_border_symbol_pos(grid: &Vec<Vec<char>>, i: i32, j: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec![];

    for k in 0..(DX.len()) {
        let x: i32 = i + DX[k];
        let y: i32 = j + DY[k];

        if is_in(&grid, x, y) && grid[x as usize][y as usize] == '*' {
            result.push((x, y));
        }
    }

    result
}

fn sum_gear_ratios(gear_candidates: &HashMap<(i32, i32), Vec<u32>>) -> u32 {
    let mut result: u32 = 0;

    for (_, adjacents) in &*gear_candidates {
        if adjacents.len() == NEEDED_ADJ {
            result += adjacents[0] * adjacents[1];
        }
    }

    result
}

fn solve(grid: Vec<Vec<char>>) -> u32 {
    let mut gear_candidates: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        let mut cur: u32 = 0;
        let mut asterisk_positions = HashSet::new();

        for (j, chr) in row.iter().enumerate() {
            if chr.is_digit(RADIX) {
                cur = 10 * cur + chr.to_digit(RADIX).unwrap();

                let symbol_pos = asterisk_border_symbol_pos(&grid, i as i32, j as i32);

                for pos in symbol_pos {
                    asterisk_positions.insert(pos);
                }
            } else {
                for pos in asterisk_positions.iter() {
                    let key = (pos.0, pos.1);

                    gear_candidates.entry(key).or_insert(Vec::new()).push(cur);
                }

                asterisk_positions.clear();
                cur = 0;
            }
        }

        for pos in asterisk_positions.iter() {
            let key = (pos.0, pos.1);

            gear_candidates.entry(key).or_insert(Vec::new()).push(cur);
        }
    }

    return sum_gear_ratios(&gear_candidates);
}

fn main() {
    let grid = read_input();

    let result = solve(grid);

    println!("{result}");
}
