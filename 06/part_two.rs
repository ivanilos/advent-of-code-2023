use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(input: Vec<String>) -> (u64, u64) {
    let splitted_time: Vec<&str> = input[0].split(":").collect();
    let time_str: String = splitted_time[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let time: u64 = time_str.parse::<u64>().unwrap();

    let splitted_distance: Vec<&str> = input[1].split(":").collect();
    let distance_str: String = splitted_distance[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let distance: u64 = distance_str.parse::<u64>().unwrap();

    return (time, distance);
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
    let (time, distance) = parse_input(input);

    let delta = (time * time) as i64 - (4 * distance) as i64;

    if delta >= 0 {
        let mut min_time = 1 + (((time as f64) - (delta as f64).sqrt()) / 2.0).floor() as i64;
        min_time = std::cmp::max(0, min_time);

        let mut max_time = -1 + (((time as f64) + (delta as f64).sqrt()) / 2.0).ceil() as i64;
        max_time = std::cmp::min(time as i64, max_time);

        max_time - min_time + 1
    } else {
        0
    }
}

fn main() {
    let input = read_input();

    let result = solve(input);

    println!("{result}");
}
