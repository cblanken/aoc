use aoc_utils::{
    read_file,
};

use std::fmt;
use std::num::ParseIntError;

struct Card {
    winning_numbers: Vec<Result<u32, ParseIntError>>,
    match_numbers: Vec<Result<u32, ParseIntError>>,
    score: u32,
    match_number_count: u32,
}

impl Card {
    fn new(winning_numbers: Vec<Result<u32, ParseIntError>>, match_numbers: Vec<Result<u32, ParseIntError>>) -> Card {
        let mut card = Card { winning_numbers, match_numbers, score: 0, match_number_count: 0 };
        card.score = card.score();
        card.match_number_count = card.get_match_num_count();

        card
    }

    fn get_match_num_count(&self) -> u32 {
        self.match_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count() as u32
    }

    fn score(&self) -> u32 {
        let match_num_count = self.get_match_num_count();

        if match_num_count > 0 {
            let base: u32 = 2;
            let score = base.pow(match_num_count as u32 - 1);
            return score;
        } else {
            return 0;
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card: {}/{}", self.score, self.match_number_count)
    }

}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let mut cards: Vec<(u32, Card)> = data.lines()
        .into_iter()
        .map(|line| {
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

            (1, Card::new(winning_numbers, match_numbers))
        })
        .collect();

    for i in 0..cards.len() {
        for j in 1..cards[i].1.match_number_count + 1 {
            cards[i + j as usize].0 = cards[i+j as usize].0 + cards[i].0; 
        }
    }

    let ans = cards.into_iter().fold(0, |acc, card| acc + card.0);
    ans.to_string()
}
