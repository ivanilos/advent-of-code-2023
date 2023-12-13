use std::collections::HashMap;
use std::fs;

const DAMAGED: char = '#';
const OPERATIONAL: char = '.';

fn read_input() -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut rows: Vec<Vec<char>> = vec![];
    let mut records: Vec<Vec<usize>> = vec![];
    for line in input {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();

        let row: Vec<char> = splitted_line[0].chars().collect();
        let record: Vec<usize> = splitted_line[1]
            .split(',')
            .map(|x| -> usize { x.parse().unwrap() })
            .collect();

        rows.push(row);
        records.push(record);
    }
    (rows, records)
}

fn no_damaged_spring_after(row_pos: usize, row: &Vec<char>) -> bool {
    for i in row_pos..row.len() {
        if row[i] == DAMAGED {
            return false;
        }
    }

    true
}

fn can_fit_record_val(val: usize, is_last_record: bool, row_pos: usize, row: &Vec<char>) -> bool {
    if row_pos + val - 1 + (!is_last_record as usize) >= row.len() {
        return false;
    }

    for i in row_pos..(row_pos + val) {
        if row[i] == OPERATIONAL {
            return false;
        }
    }

    if !is_last_record {
        return row[row_pos + val] != DAMAGED;
    }

    true
}

fn can_skip_row_pos(row_pos: usize, row: &Vec<char>) -> bool {
    return row[row_pos] != DAMAGED;
}

fn get_ways(
    row_pos: usize,
    record_pos: usize,
    row: &Vec<char>,
    record: &Vec<usize>,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if row_pos >= row.len() {
        return (record_pos as usize >= record.len()) as u64;
    }
    if record_pos >= record.len() {
        return no_damaged_spring_after(row_pos, row) as u64;
    }

    if memo.contains_key(&(row_pos, record_pos)) {
        return memo[&(row_pos, record_pos)];
    }

    let mut result = 0;
    // apply record
    let is_last_record = record_pos == record.len() - 1;
    if can_fit_record_val(record[record_pos], is_last_record, row_pos, row) {
        let new_row_pos = row_pos + record[record_pos] + !is_last_record as usize;
        let new_record_pos = record_pos + 1;

        result += get_ways(new_row_pos, new_record_pos, row, record, memo);
    }
    // do not apply record
    if can_skip_row_pos(row_pos, row) {
        result += get_ways(row_pos + 1, record_pos, row, record, memo);
    }

    memo.insert((row_pos, record_pos), result);

    result
}

fn solve(rows: Vec<Vec<char>>, records: Vec<Vec<usize>>) -> u64 {
    let mut result = 0;

    for i in 0..rows.len() {
        let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

        result += get_ways(0, 0, &rows[i], &records[i], &mut memo);
    }

    result
}

fn main() {
    let (rows, records) = read_input();

    let result = solve(rows, records);

    println!("{result}");
}
