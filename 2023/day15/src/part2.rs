use std::{collections::HashMap, fmt};

use aoc_utils::read_file;

fn hash(s: &str) -> u32 {
    let mut h: u32 = 0;
    for c in s.chars() {
        h += c as u32;
        h *= 17;
        h = h % 256;
    }

    h
}

type LenseBox = Vec<(String, usize)>;

fn get_focusing_power(lense_box: &LenseBox, box_idx: usize) -> usize {
    lense_box
        .iter()
        .enumerate()
        .fold(0, |acc, (i, focal_length)| {
            acc + (1 + box_idx) * (i + 1) * focal_length.1
        })
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let line: &str = data.lines().collect::<Vec<&str>>()[0];

    let init_seq: Vec<&str> = line.split(',').collect();

    let mut boxes: Vec<LenseBox> = vec![vec![]; 256];

    for s in init_seq.iter() {
        // Check operator
        if s.contains('=') {
            let label = &s[..s.find('=').unwrap()];
            let focal_len = s[s.len()-1..].parse::<usize>().unwrap();
            let box_idx = hash(label) as usize;


            let new_lense = (label.to_string(), focal_len);

            // Replace existing lense
            let mut lense_added = false;
            if boxes[box_idx].len() > 0 {
                for i in 0..boxes[box_idx].len() {
                    if boxes[box_idx][i].0 == label {
                        boxes[box_idx][i] = new_lense.clone();
                        lense_added = true;
                        break;
                    }
                }
            }

            if !lense_added {
                boxes[box_idx].push(new_lense);
            }
        } else if s.contains('-') {
            let label = &s[..s.find('-').unwrap()];
            let box_idx = hash(label) as usize;

            let target_box_id = boxes[box_idx].iter().position(|lbox| lbox.0 == label);
            if let Some(i) = target_box_id {
                boxes[box_idx].remove(i);
            }
        }
        // dbg!(&boxes[..10]);

    }

    // dbg!(&init_seq);

    boxes.iter()
        .enumerate()
        .fold(0, |acc, (i, lbox)| acc + get_focusing_power(lbox, i)).to_string()
}
