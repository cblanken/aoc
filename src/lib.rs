use std::fs;

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
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub fn is_adjacent_to_area(area: &(Vec2, Vec2), target: &Vec2) -> bool {
    // area: top-left and bottom-right position of "area" rectangle
    let start_x = area.0.x;
    let start_y = area.0.y;
    let end_x = area.1.x;
    let end_y = area.1.y;

    let x_range = start_x..=end_x + 1;
    let mid_y_range = start_y..=end_y + 1;
    // println!("{start_x}, {start_y}, {end_x}, {end_y}");
    // println!("{}, {}", &target.x, &target.y);
    if (x_range.contains(&target.x) && target.y == start_y - 1) ||  // Top row
        (mid_y_range.contains(&target.y) && (target.x == start_x - 1 || target.x == end_x + 1)) || // Middle sides
        (x_range.contains(&target.x) && target.y == end_y + 1) { // Bottom row sides
        return true;
    } else {
        return false;
    }
}
