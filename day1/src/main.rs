use std::string::String;
use aoc_utils::{
    print_file,
    read_file,
};


static INPUT_FILE_PATH: &str = "input.txt";

fn find_first_digit(line: &str) -> u32 {
    for c in line.chars() {
        match c.to_digit(10) {
            Some(val) => return val,
            None => continue,
        }
    }

    return 0;
}

fn find_last_digit(line: &str) -> u32 {
    for c in line.chars().rev() {
        match c.to_digit(10) {
            Some(val) => return val,
            None => continue,
        }
    }

    return 0;
}

fn part1() -> String {
    let contents = read_file(INPUT_FILE_PATH);

    let calibration = contents.lines().into_iter()
        .map(|line| { 
            let first = find_first_digit(&line);
            let last = find_last_digit(&line);
            format!("{first}{last}").to_string()
        })
        .map(|calibration_str| calibration_str.parse::<i32>().unwrap())
        .fold(0, |acc, num| acc + num);
    

    calibration.to_string()
}

fn part2() -> String {
    "ANSWER".to_string()
}

fn main() {

    print_file(INPUT_FILE_PATH);

    println!("");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

