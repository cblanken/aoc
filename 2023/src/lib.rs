use std::fs;
use std::cmp;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Could not read file");

    contents
}

pub fn str_to_vec2d(s: &str) -> Vec<Vec<char>> {
    let mut str_vec = Vec::new();
    for line in s.lines().into_iter() {
        let mut line_vec = Vec::<char>::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        str_vec.push(line_vec);
    }

    str_vec
}

pub fn print_file(path: &str) {
    let contents = read_file(path);

    let hr = "=".repeat(12 + path.len());
    println!("{hr}");
    println!("INPUT FILE: {path}");
    println!("{hr}");
    for l in contents.lines() {
        println!("{l}");
    }

    println!("{hr}");
    println!("END FILE");
    println!("{hr}");
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

pub fn is_adjacent_to_area(area: &(Vec2, Vec2), target: &Vec2, max_x: usize) -> bool {
    // area: top-left and bottom-right position of "area" rectangle
    let start_x = area.0.x;
    let start_y = area.0.y;
    let end_x = area.1.x;
    let end_y = area.1.y;

    let x_range = start_x..=end_x + 1;
    let mid_y_range = start_y..=end_y + 1;
    if (x_range.contains(&target.x) && start_y > 0 && target.y == start_y - 1) ||  // Top row
        (mid_y_range.contains(&target.y) && start_x > 0 && (target.x == cmp::max(start_x - 1, 0) || target.x == cmp::min(end_x + 1, max_x))) || // Middle sides
        (x_range.contains(&target.x) && target.y == end_y + 1) { // Bottom row
        return true;
    } else {
        return false;
    }
}

pub fn is_point_in_border(point: Vec2, area: &(Vec2, Vec2),  max_pos: &Vec2) -> bool {
    let border = get_adjacent_positions(area, max_pos);
    border.into_iter().any(|p| p == point)
}

pub fn get_adjacent_positions(area: &(Vec2, Vec2),  max_pos: &Vec2) -> Vec<Vec2> {
    let mut adjacent_positions = Vec::<Vec2>::new();

    assert!(max_pos.x > 0 && max_pos.y > 0);
    assert!(area.0.x <= area.1.x && area.0.y <= area.1.y);

    let start_x = area.0.x;
    let start_y = area.0.y;
    let end_x = area.1.x;
    let end_y = area.1.y;

    let is_flush_left = start_x <= 0;
    let is_flush_right = end_x >= max_pos.x;
    let is_flush_top = start_y <= 0;
    let is_flush_bottom = end_y >= max_pos.y;

    // Middle left
    if !is_flush_left {
        for i in start_y..=end_y {
            adjacent_positions.push(Vec2 { x: start_x - 1, y: i })
        }
    }

    // Middle right
    if !is_flush_right {
        for i in start_y..=end_y {
            adjacent_positions.push(Vec2 { x: end_x + 1, y: i })
        }
    }
    
    // Middle top
    if !is_flush_top {
        for i in start_x..=end_x {
            adjacent_positions.push(Vec2 { x: i, y: start_y - 1 })
        }
    }

    // Middle bottom
    if !is_flush_bottom {
        for i in start_x..=end_x {
            adjacent_positions.push(Vec2 { x: i, y: end_y + 1 })
        }
    }

    // Corners

    // Top left
    if !is_flush_left && !is_flush_top {
        adjacent_positions.push(Vec2 { x: start_x - 1, y: start_y - 1 });
    }

    // Top right
    if !is_flush_right && !is_flush_top {
        adjacent_positions.push(Vec2 { x: end_x + 1, y: start_y - 1 });
    }

    // Bottom left
    if !is_flush_left && !is_flush_bottom {
        adjacent_positions.push(Vec2 { x: start_x - 1, y: end_y + 1 });
    }

    // Bottom right
    if !is_flush_right && !is_flush_bottom {
        adjacent_positions.push(Vec2 { x: end_x + 1, y: end_y + 1 });
    }

    adjacent_positions
}
