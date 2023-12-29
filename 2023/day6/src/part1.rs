use aoc_utils::{
    read_file,
};

use roots::Roots;
use roots::find_roots_quadratic;

const ACCELERATION: i64 = 1;

fn get_race_distance(duration: f64, time: f64) -> f64 {
    time * (-time + duration)
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let lines: Vec<&str> = data.lines().collect();

    let times: Vec<i64> = lines[0]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect();

    let dists: Vec<i64> = lines[1]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect();

    let mut counts: Vec<f64> = vec![];

    for i in 0..times.len().min(dists.len()) {
        let duration = times[i] as u64;
        let distance_record = dists[i] as f64;

        println!("Game {i}");
        let roots = match find_roots_quadratic(-1 as f64, duration as f64, -distance_record) {
            Roots::No(_) => panic!("No roots!"),
            Roots::One(x) => panic!("One root: {}", x[0]),
            Roots::Two(x) => {
                println!("Two roots: {} and {}", x[0] as f64, x[1] as f64);
                Some(x)
            },
            _ => panic!("Way too many roots!")
        };

        let record_beating_options_count = 
            ((roots.unwrap()[1] - 0.0001).floor() - (roots.unwrap()[0] + 0.0001).ceil()) + 1f64;
        println!("Options to win: {record_beating_options_count}\n");

        counts.push(record_beating_options_count);
    }

    counts.into_iter().reduce(|acc, c| acc * c).unwrap().to_string()
}
