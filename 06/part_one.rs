use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(input: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let splitted_times: Vec<&str> = input[0].split(":").collect();

    let times: Vec<u64> = splitted_times[1]
        .split_whitespace()
        .map(|x| -> u64 { x.parse().unwrap() })
        .collect();

    let splitted_distances: Vec<&str> = input[1].split(":").collect();

    let distances: Vec<u64> = splitted_distances[1]
        .split_whitespace()
        .map(|x| -> u64 { x.parse().unwrap() })
        .collect();

    return (times, distances);
}

/*
Holding the button for T seconds, max time M, and distance to beat D:

T * (M - T) > D =>
-T^2 + MT - D > 0 =>

T = (-M +- sqrt(M^2 - 4D)) / -2 =>

T = (M -+ sqrt(M^2 - 4D)) / 2

T1 = (M - sqrt(M^2 - 4D)) / 2
T2 = (M - sqrt(M^2 - 4D)) / 2

Time should be be in (T1, T2)

*/
fn solve(input: Vec<String>) -> i64 {
    let (times, distances) = parse_input(input);

    let mut result = 1;
    for i in 0..times.len() {
        let delta = (times[i] * times[i]) as i64 - (4 * distances[i]) as i64;

        if delta >= 0 {
            let mut min_time =
                1 + (((times[i] as f64) - (delta as f64).sqrt()) / 2.0).floor() as i64;
            min_time = std::cmp::max(0, min_time);

            let mut max_time =
                -1 + (((times[i] as f64) + (delta as f64).sqrt()) / 2.0).ceil() as i64;
            max_time = std::cmp::min(times[i] as i64, max_time);

            result *= max_time - min_time + 1;
        }
    }

    result
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
