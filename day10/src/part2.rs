use aoc_utils::{
    read_file,
};

#[derive(Debug)]
enum DIR {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn get_pos_by_dir(dir: &DIR, pos: (usize, usize)) -> (usize, usize) {
    match dir {
        DIR::NORTH => (pos.0 - 1, pos.1),
        DIR::SOUTH => (pos.0 + 1, pos.1),
        DIR::EAST  => (pos.0, pos.1 + 1),
        DIR::WEST  => (pos.0, pos.1 - 1),
    }
}

fn get_next_loop_pos(dir: DIR, pos: (usize, usize), pos_value: char, max_x: usize, max_y: usize) -> ((usize, usize), DIR) {
    match (pos_value, &dir, pos) {
        // Continue in the same direction
        ('|', _, (_, _)) |
        ('-', _, (_, _)) => (get_pos_by_dir(&dir, pos), dir),

        ('L', DIR::WEST,  (_, _)) => (get_pos_by_dir(&DIR::NORTH, pos), DIR::NORTH), // WEST to NORTH
        ('L', DIR::SOUTH, (_, _)) => (get_pos_by_dir(&DIR::EAST,  pos), DIR::EAST),  // SOUTH to EAST

        ('J', DIR::EAST,  (_, _)) => (get_pos_by_dir(&DIR::NORTH, pos), DIR::NORTH), // EAST to NORTH
        ('J', DIR::SOUTH, (_, _)) => (get_pos_by_dir(&DIR::WEST,  pos), DIR::WEST),  // SOUTH to WEST

        ('7', DIR::EAST,  (_, _)) => (get_pos_by_dir(&DIR::SOUTH, pos), DIR::SOUTH), // EAST to SOUTH
        ('7', DIR::NORTH, (_, _)) => (get_pos_by_dir(&DIR::WEST,  pos), DIR::WEST),  // NORTH to WEST

        ('F', DIR::WEST,  (_, _)) => (get_pos_by_dir(&DIR::SOUTH, pos), DIR::SOUTH), // WEST to SOUTH
        ('F', DIR::NORTH, (_, _)) => (get_pos_by_dir(&DIR::EAST,  pos), DIR::EAST),  // NORTH to EAST

        _ => panic!("Could not proceed to next position! Given {dir:?}, {pos:?}, {pos_value:?}"),
    }
}

fn get_valid_replacement(map: &Vec<Vec<char>>, pos: (usize, usize), max_x: usize, max_y: usize) -> (char, DIR) {
    let north   = if pos.0 > 0      { map[pos.0-1][pos.1] } else { ' ' };
    let south   = if pos.0 < max_y  { map[pos.0+1][pos.1] } else { ' ' };
    let east    = if pos.1 < max_x  { map[pos.0][pos.1+1] } else { ' ' };
    let west    = if pos.1 > 0      { map[pos.0][pos.1-1] } else { ' ' };

    match (north, south, east, west) {
        ('|' | 'F' | '7'    , '|' | 'J' | 'L'   , _e                , _w                ) => ('|', DIR::NORTH),
        (_n                 , _s                , '-' | 'J' | '7'   , '-' | 'L' | 'F'   ) => ('-', DIR::EAST),
        ('|' | 'F' | '7'    , _s                , '-' | 'J' | '7'   , _w                ) => ('L', DIR::SOUTH),
        ('|' | 'F' | '7'    , _s                , _e                , '-' | 'L' | 'F'   ) => ('J', DIR::EAST),
        (_n                 , '|' | 'J' | 'L'   , _e                , '-' | 'L' | 'F'   ) => ('7', DIR::EAST),
        (_n                 , '|' | 'J' | 'L'   , '-' | 'J' | '7'   , _w                ) => ('F', DIR::NORTH),
        (_, _, _, _) => panic!("Could not find valid replacement for {pos:?} in map")
    }
}

fn build_loop(start_pos: (usize, usize), map: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
    assert!(map.len() > 0);
    assert!(map[0].len() > 0);
    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let mut loop_node_positions: Vec<(usize, usize)> = vec![];

    // Establish initial direction
    let start_dir: DIR;
    let start_char: char;
    (start_char, start_dir) = get_valid_replacement(&map, start_pos, max_x, max_y);


    let mut dir: DIR;
    let mut curr_pos: (usize, usize);
    (curr_pos, dir) = get_next_loop_pos(start_dir, start_pos, start_char, max_x, max_y);
    loop_node_positions.push(curr_pos);

    loop {
        let next_pos: (usize, usize);
        (next_pos, dir) = get_next_loop_pos(dir, curr_pos, map[curr_pos.0][curr_pos.1], max_x, max_y);
        // dbg!(map[next_pos.0][next_pos.1], &dir);
        loop_node_positions.push(next_pos);

        // Complete loop
        if map[next_pos.0][next_pos.1] == 'S' {
            break
        }
        curr_pos = next_pos;
    }


    loop_node_positions
}

fn find_start(start_char: char, map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (r, row) in map.into_iter().enumerate() {
        for (c, _col) in row.into_iter().enumerate() {
            if map[r][c] == start_char {
                return Some((r, c));
            }
        }
    }

    None

}

fn is_point_inside_loop(point: (usize, usize), loop_points: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> bool {
    let max_height = map.len();

    // Point on loop (polygon)
    if loop_points.iter().any(|p| *p == (point.0, point.1)) {
        return false;
    }

    let mut count_f = 0; // combination count of 'F' in column
    let mut count_l = 0; // combination count of 'L' in column
    let mut count_j = 0; // combination count of 'J' in column
    let mut count_7 = 0; // combination count of '7' in column

    // Ray cast downwards
    let mut y = point.0 + 1;
    let mut intersection_count = 0;
    while y < max_height {

        let current_tile = map[y][point.1];
        if loop_points.iter().any(|p| *p == (y, point.1)) { // point on loop
            if current_tile == 'F'{
                count_f += 1;
            } else if current_tile == 'J' {
                count_j += 1;
            } else if current_tile == 'L' {
                count_l += 1;
            } else if current_tile == '7' {
                count_7 += 1;
            } else if current_tile == '-' {
                // println!("{:?}", (y, point.1));
                intersection_count += 1;
            } 
        }
        y += 1;
    }

    if count_f >= 1 && count_j >= 1 {
        intersection_count += count_f.min(count_j);
        // intersection_count += 1;
    }

    if count_l >= 1 && count_7 >= 1 {
        intersection_count += count_l.min(count_7);
        // intersection_count += 1;
    }

    // println!("f: {count_f}, j: {count_j}, l: {count_l}, 7: {count_7}");
    intersection_count % 2 == 1
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let mut map: Vec<Vec<char>> = data
        .split_whitespace()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let start = find_start('S', &map).unwrap();
    let animal_loop = build_loop(start, &mut map);

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let mut internal_point_count = 0;

    dbg!(is_point_inside_loop((6, 0), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 1), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 2), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 3), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 4), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 5), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 6), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 7), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 8), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 9), &animal_loop, &map));
    dbg!(is_point_inside_loop((6, 10), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 11), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 12), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 13), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 14), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 15), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 16), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 17), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 18), &animal_loop, &map));
    // dbg!(is_point_inside_loop((6, 19), &animal_loop, &map));

    for r in 0..max_y {
        println!("Row {r}");
        for c in 0..max_x {
            if is_point_inside_loop((r, c), &animal_loop, &map) {
                internal_point_count += 1;
            }
        }
    }

    internal_point_count.to_string()
}

// 1520 is too high
