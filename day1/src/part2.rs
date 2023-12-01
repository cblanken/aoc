use aoc_utils::read_file;


static DIGITS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn find_first_spelled_digits(line: &str) -> Option<(u32, u32)> {
    let mut start_indexes: [(u32, u32); 9] = [(u32::MAX, 0); 9];
    for i in 0..9 {
        let start_index = line.to_string().find(DIGITS[i]);
        if start_index.is_some() {
            start_indexes[i] = (start_index.unwrap() as u32, i as u32 + 1);
        }
    }

    let start = start_indexes.into_iter().fold((u32::MAX, 0), |acc, n| if n.0 < acc.0 { n } else { acc });

    match start {
        (u32::MAX, _) => None,
        _ => Some(start),
    }
}

fn find_last_spelled_digits(line: &str) -> Option<(u32, u32)> {
    let mut start_indexes: [(u32, u32); 9] = [(0, 0); 9];
    for i in 0..9 {
        let start_index = line.to_string().rfind(DIGITS[i]);
        if start_index.is_some() {
            start_indexes[i] = (start_index.unwrap() as u32, i as u32 + 1);
        }
    }

    let start = start_indexes.into_iter().fold((0, 0), |acc, n| if n.0 > acc.0 { n } else { acc });

    match start {
        (0, _) => None,
        _ => Some(start),
    }
}

fn find_first_digit(line: &str) -> Option<(u32, u32)> {
    for (i, c) in line.chars().enumerate() {
        match c.to_digit(10) {
            Some(val) => return Some((i as u32, val)),
            None => continue,
        }
    }

    return None;
}

fn find_last_digit(line: &str) -> Option<(u32, u32)> {
    for (i, c) in line.chars().rev().enumerate() {
        match c.to_digit(10) {
            Some(val) => return Some((line.len() as u32 - i as u32, val)),
            None => continue,
        }
    }

    return None;
}

pub fn solve(filepath: &str) -> String {
    let contents = read_file(filepath);

    let calibration = contents.lines().into_iter()
        .map(|line| { 
            let mut first_spelled = match find_first_spelled_digits(&line) {
                Some(x) => x,
                None => (u32::MAX, 0),
            };

            let mut last_spelled = match find_last_spelled_digits(&line) {
                Some(x) => x,
                None => (0, u32::MAX),
            };

            let first_num = match find_first_digit(&line) {
                Some(x) => x,
                None => (u32::MAX, 0),
            };

            let last_num = match find_last_digit(&line) {
                Some(x) => x,
                None => (0, 0),
            };

            if first_num.0 < first_spelled.0 {
                first_spelled = first_num
            }

            if last_num.0 > last_spelled.0 {
                last_spelled = last_num
            }

            // println!("{}{}; {},{}", first_spelled.1, last_spelled.1, first_spelled.0, last_spelled.0);
            format!("{}{}", first_spelled.1, last_spelled.1).to_string()
        })
        .map(|calibration_str| calibration_str.parse::<u32>().unwrap())
        .fold(0, |acc, num| acc + num);


    calibration.to_string()
}

