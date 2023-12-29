use aoc_utils::{
    read_file,
    str_to_vec2d,
    Vec2,
    get_adjacent_positions,
};


pub fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);
    let data_2d = str_to_vec2d(&data);

    let max_pos = Vec2 {
        x: data_2d[0].len() - 1,
        y: data_2d.len() - 1,
    };

    let mut nums = Vec::<(Vec2, Vec2)>::new();
    let mut symbols = Vec::<Vec2>::new();

    for (y, line) in data_2d.clone().into_iter().enumerate() {
        let mut start_x: Option<usize> = None;
        let mut end_x: Option<usize> = None;
        for (x, c) in line.into_iter().enumerate() {
            if c.is_digit(10) && start_x.is_none() { // first digit in series
                start_x = Some(x);
                if end_x.is_none() {
                    end_x = Some(x);
                }
            } else if c.is_digit(10) && start_x.is_some() { // non-first digit in series
                end_x = Some(x);
            } else { // non-digit

                if end_x.is_some() {
                    nums.push((
                        Vec2 { x: start_x.unwrap(), y },
                        Vec2 { x: end_x.unwrap(), y },
                    ));

                    // println!("Number found: @({},{});{}", start_x.unwrap(), end_x.unwrap(), y);
                    start_x = None;
                    end_x = None;

                }

                if c != '.' { // symbol
                    symbols.push(Vec2 { x, y });
                    // println!("Symbol found: '{c}' @{x},{y}");
                } else { } // empty (dot)
            }

            if x >= max_pos.x && start_x.is_some() {
                nums.push((
                    Vec2 { x: start_x.unwrap(), y },
                    Vec2 { x: max_pos.x, y },
                ));

                println!("Number found: @({},{});{}", start_x.unwrap(), end_x.unwrap(), y);
                start_x = None;
                end_x = None;
            }
        }
    }

    nums.into_iter()
        .fold(0, |acc, num_seq| {
            let border_contains_symbol = get_adjacent_positions(&num_seq, &max_pos)
                .into_iter()
                .any(|pos| is_symbol(data_2d[pos.y][pos.x]));

            if border_contains_symbol {
                let mut num = "".to_string();
                let start_x = num_seq.0.x as usize;
                let end_x = num_seq.1.x as usize;
                let y = num_seq.0.y as usize;
                for i in start_x..=end_x {
                    num.push(data_2d[y][i]);
                }
                println!("{num} adjacent to symbol!");
                return acc + num.parse::<usize>().unwrap();
            } else {
                return acc;
            }
        }).to_string()
}
