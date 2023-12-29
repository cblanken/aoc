use aoc_utils::{
    read_file,
    str_to_vec2d,
    Vec2,
    get_adjacent_positions,
};

use itertools::Itertools;


fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn find_adjacent_nums(area: &(Vec2, Vec2), data_2d: &Vec<Vec<char>>, max_pos: &Vec2) -> Vec<Vec2> {
    get_adjacent_positions(area, max_pos)
        .into_iter()
        .filter(|pos| data_2d[pos.y][pos.x].is_digit(10))
        .collect()
}

fn expand_num(target: Vec2, data_2d: &Vec<Vec<char>>, max_pos: &Vec2) -> (Vec2, Vec2) {
    // Given a single digit position in input expand to full number range

    let mut left = Vec2 { x: target.x, y: target.y };
    let mut right = Vec2 { x: target.x, y: target.y };

    // Search left
    let mut left_x = target.x;
    while left_x > 0 {
        if data_2d[target.y][left_x - 1].is_digit(10) {
            left_x -= 1;
        } else {
            break;
        }
    }
    left.x = left_x;

    // Search right
    let mut right_x = target.x;
    while right_x < max_pos.x {
        if data_2d[target.y][right_x + 1].is_digit(10) {
            right_x += 1;
        } else {
            break;
        }
    }
    right.x = right_x;


    (left, right)
}

pub fn solve(filepath: &str) -> String {

    let data = read_file(filepath);
    let data_2d = str_to_vec2d(&data);

    let max_pos = Vec2 {
        x: data_2d[0].len() - 1,
        y: data_2d.len() - 1,
    };

    // Find all gears in input (ignore other symbols)
    let mut gears = Vec::<Vec2>::new();
    for (y, line) in data_2d.clone().into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if is_gear(c) {
                gears.push( Vec2 { x, y } );
            }
        }
    }

    // println!("GEAR RATIO NUMS");
    gears.into_iter()
        .fold(0, |acc, gear| {
            // println!("=== GEAR: ({},{}) ===", gear.x, gear.y);
            let area = (gear.clone(), gear.clone());
            let adj_nums = find_adjacent_nums(&area, &data_2d, &max_pos);

            if adj_nums.len() > 1 {
                let nums: Vec<(Vec2, Vec2)> = adj_nums.into_iter()
                    .map(|num| expand_num(num, &data_2d, &max_pos))
                    .into_iter()
                    .unique()
                    .collect();

                if nums.len() != 2 {
                    return acc;
                }

                let mut part_numbers = Vec::<usize>::new();
                for n in nums {
                    // println!("{}, {} | {}, {}", n.0.x, n.0.y, n.1.x, n.1.y);

                    let mut num = "".to_string();
                    let start_x = n.0.x;
                    let end_x = n.1.x;
                    let y = n.0.y;
                    for x in start_x..=end_x {
                        num.push(data_2d[y][x]);
                    }
                    // println!("{num} adjacent to symbol!");
                    part_numbers.push(num.parse::<usize>().unwrap());
                }

                return acc + part_numbers.into_iter().fold(1, |a, n| a * n);
            }

            return acc;
        }).to_string()
}
