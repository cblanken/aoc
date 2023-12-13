use std::fmt;

use aoc_utils::{
    read_file,
};

use combinations::Combinations;

#[derive(Debug)]
struct SpringConditionRecord {
    spring_state: String,
    known_positions: Vec<usize>,
    unknown_positions: Vec<usize>,
    cgds: Vec<u32>, // Contiguous Group sizes of Damaged Springs
    total_springs: u32,
}

impl SpringConditionRecord {
    fn new(spring_state: &str, cgds: Vec<u32>) -> SpringConditionRecord {
        SpringConditionRecord {
            spring_state: spring_state.to_string(),
            known_positions: spring_state.trim()
                .chars()
                .enumerate()
                .map(|(i, c)| (i, c))
                .filter(|x| x.1 != '?')
                .map(|x| x.0).collect(),
            unknown_positions: spring_state.trim()
                .chars()
                .enumerate()
                .map(|(i, c)| (i, c))
                .filter(|x| x.1 == '?')
                .map(|x| x.0)
                .collect(),
            cgds: cgds.clone(),
            total_springs: cgds.into_iter().fold(0, |acc, s| acc + s),
        }
    }

    fn get_combinations(&self) -> Combinations<usize> {
        // TODO: optimize by reducing initial combinations
        Combinations::new(self.unknown_positions.clone(), self.unknown_positions.len())
    }

    fn is_combination_valid(&self) -> bool {
        // TODO
        true
    }

    fn get_valid_combination_count(&self) -> u32 {
        // TODO
        0
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let records: Vec<SpringConditionRecord> = data.lines()
        .map(|l| {
            let split: Vec<_> = l.split_whitespace().collect();
            SpringConditionRecord::new(
                split[0],
                split[1].split(',').map(|n| n.parse::<u32>().unwrap()).collect()
            )
        })
        .collect();

    dbg!(&records[0]);

    "part 1".to_string()
}
