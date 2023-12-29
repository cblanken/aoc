use aoc_utils::{
    read_file,
};

use std::num::ParseIntError;

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    data.lines()
        .into_iter()
        .fold(0, |acc, line| {
            let split: Vec<&str> = line.split('|').collect();

            let winning_numbers: Vec<Result<u32, ParseIntError>> = split[0]
                .split(":")
                .collect::<Vec<&str>>()[1].trim()
                .split_whitespace()
                .map(|num| num.trim().parse::<u32>())
                .collect();

            let match_numbers: Vec<Result<u32, ParseIntError>> = split[1]
                .split_whitespace()
                .map(|num| num.trim().parse::<u32>())
                .collect();

            // println!("\n=== Winning Nums ===");
            // for n in winning_numbers {
            //     println!("WINNING NUM: {}", n.unwrap());
            // }

            // println!("\n=== Match Nums ===");
            // for n in match_numbers {
            //     println!("MATCH NUM: {}", n.unwrap());
            // }

            let match_num_count = match_numbers.iter().filter(|n| winning_numbers.contains(n)).count();

            if match_num_count > 0 {
                let base: u32 = 2;
                let score = base.pow(match_num_count as u32 - 1);
                println!("SCORE: {score}");
                return acc + score;
            }

            acc
        }).to_string()
}
