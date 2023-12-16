use std::fmt;

use aoc_utils::{
    read_file,
};


fn hash(s: &str) -> u32 {
    let mut h: u32 = 0;
    for c in s.chars() {
        h += c as u32;
        h *= 17;
        h = h % 256;
    }

    h
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let line: &str = data.lines().collect::<Vec<&str>>()[0];

    let init_seq: Vec<&str> = line.split(',').collect();

    init_seq.iter().fold(0, |acc, s| acc + hash(s)).to_string()
}
