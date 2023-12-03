use aoc_utils::{
    read_file,
    str_to_vec2d,
    Vec2,
    is_adjacent_to_area,
};


pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);
    let data_2d = str_to_vec2d(&data);

    let mut nums = Vec::<(Vec2, Vec2)>::new();
    let mut symbols = Vec::<Vec2>::new();

    for (y, line) in data_2d.clone().into_iter().enumerate() {
        let mut start_x: i32 = -1;
        let mut end_x: i32 = -1;
        for (x, c) in line.into_iter().enumerate() {
            if c.is_digit(10) && start_x < 0 { // first digit in series
                start_x = x as i32;
            } else if c.is_digit(10) { // non-first digit in series
                end_x = x as i32;
            } else if c != '.' { // symbol
                symbols.push(Vec2 { x: x as i32, y: y as i32 });
                println!("Symbol found: '{c}' @{x},{y}");
            } else if end_x > 0 { // empty and digit series found
                nums.push((
                    Vec2 { x: start_x, y: y as i32 },
                    Vec2 { x: end_x, y: y as i32 },
                ));
                println!("Number found: @({start_x},{end_x});{y}");

                start_x = -1;
                end_x = -1

            }
        }
        println!();
    }

    nums.into_iter()
        .fold(0, |acc, num_seq| {
            if symbols.clone().into_iter().any(|sym| is_adjacent_to_area(&num_seq, &sym)) {
                let mut num = "".to_string();
                let start_x = num_seq.0.x as usize;
                let end_x = num_seq.1.x as usize;
                let y = num_seq.0.y as usize;
                for i in start_x..=end_x {
                    num.push(data_2d[y][i]);
                }
                println!("{num} adjacent to symbol!");
                return acc + num.parse::<u32>().unwrap();
            } else {
                println!("Num NOT adjacent to symbol!");
                return acc;
            }
        }).to_string()

    // let is_adj = is_adjacent_to_area(&nums[1], &symbols[0]);
    // println!("{}", is_adj);

    // data_2d[0][0].to_string()
}
