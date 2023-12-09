use std::fs;

fn read_input() -> Vec<Vec<i32>> {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result: Vec<Vec<i32>> = vec![];

    for line in input {
        result.push(
            line.split_whitespace()
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect(),
        );
    }

    result
}

fn get_prediction(value_history: &Vec<i32>) -> i32 {
    let mut extrapolation: Vec<Vec<i32>> = vec![value_history.to_vec()];

    let mut done = false;
    let mut next_row = 0;
    while !done {
        let last_row_len = extrapolation[next_row].len();
        extrapolation.push(vec![]);
        next_row += 1;

        let mut all_zeroes = true;
        for i in 1..last_row_len {
            let val = extrapolation[next_row - 1][i] - extrapolation[next_row - 1][i - 1];
            extrapolation[next_row].push(val);

            if val != 0 {
                all_zeroes = false;
            }
        }

        done = all_zeroes;
    }

    for row in (0..next_row).rev() {
        let row_len = extrapolation[row + 1].len();

        let val = extrapolation[row][row_len - 1] + extrapolation[row + 1][row_len - 1];
        extrapolation[row].push(val);
    }

    extrapolation[0][value_history.len()]
}

fn solve(input: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;
    for line in input {
        result += get_prediction(&line);
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
