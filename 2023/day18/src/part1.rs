use std::fmt;

use aoc_utils::read_file;

#[derive(Clone, Copy, Debug, PartialEq)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UNKNOWN,
}

#[derive(Debug)]
struct DigPlanItem {
    dir: DIR,
    dist: i32,
    hex: String,
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct DigPlan {
    plan: Vec<DigPlanItem>,
    max_width: u32,
    max_height: u32,
    map: Vec<Vec<char>>,
}

fn is_right_turn(old_dir: DIR, new_dir: DIR) -> bool {
    match (old_dir, new_dir) {
        (DIR::UP, DIR::RIGHT) => true,
        (DIR::RIGHT, DIR::DOWN) => true,
        (DIR::DOWN, DIR::LEFT) => true,
        (DIR::LEFT, DIR::UP) => true,
        _ => false
    }
}

fn is_left_turn(old_dir: DIR, new_dir: DIR) -> bool {
    match (old_dir, new_dir) {
        (DIR::UP, DIR::LEFT) => true,
        (DIR::LEFT, DIR::DOWN) => true,
        (DIR::DOWN, DIR::RIGHT) => true,
        (DIR::RIGHT, DIR::UP) => true,
        _ => false
    }
}


fn draw(map: &mut Vec<Vec<char>>, start: Pos, end: Pos) -> &mut Vec<Vec<char>> {
    if start.x < end.x {
        for x in start.x..=end.x {
            map[start.y as usize][x as usize] = '#';
        }
    } else if end.x < start.x {
        for x in (end.x..start.x).rev() {
            map[start.y as usize][x as usize] = '#';
        }
    } else {
        map[end.y as usize][end.x as usize] = '#';
    }


    if start.y < end.y {
        for y in start.y..=end.y {
            map[y as usize][start.x as usize] = '#';
        }
    } else if end.y < start.y {
        for y in (end.y..start.y).rev() {
            map[y as usize][start.x as usize] = '#';
        }
    } else {
        map[end.y as usize][end.x as usize] = '#';
    }
    map
}

impl DigPlan {
    fn new(plan: Vec<DigPlanItem>) -> Self {
        DigPlan {
            plan,
            max_width: 0,
            max_height: 0,
            map: vec![vec![]],
        }
    }

    fn get_points(&mut self) -> Vec<Pos> {
        let mut points: Vec<Pos> = vec![];

        let mut max_x = 0;
        let mut min_x = 9999999;
        let mut max_y = 0;
        let mut min_y = 9999999;

        let mut right_left_count = 0;
        let mut inside_polygon = false;

        let mut pos = Pos { x: -0, y: -0 };
        points.push(pos);
        let mut last_dir = self.plan[0].dir;
        for item in &self.plan {

            if is_right_turn(last_dir, item.dir) {
                right_left_count += 1;
            } else if is_left_turn(last_dir, item.dir) {
                right_left_count -= 1;
            }



            if item.dir != last_dir {
                points.push(pos);
                last_dir = item.dir;
            }

            if pos.y > max_y {
                max_y = pos.y;
            }

            if pos.x > max_x {
                max_x = pos.x;
            }

            if pos.y < min_y {
                min_y = pos.y;
            }

            if pos.x < min_x {
                min_x = pos.x;
            }


            // let dist = if item.dist > 1 { item.dist + 1 } else { 1 };
            // let dist = if right_left_count < 2 { item.dist + 1 } else { item.dist };
            let dist = if inside_polygon { 
                item.dist - 1 
            } else if right_left_count < 2 {
                item.dist + 1
            } else { 
                item.dist
            };

            match item.dir {
                DIR::UP     => { pos.y -= dist },
                DIR::DOWN   => { pos.y += dist },
                DIR::LEFT   => { pos.x -= dist },
                DIR::RIGHT  => { pos.x += dist },
                _ => panic!("Uh oh bad direction"),
            }
            // dbg!(right_left_count, inside_polygon, item.dir, dist, pos);
            // println!("");

            if inside_polygon && right_left_count == 0 {
                inside_polygon = false;
            } else if right_left_count > 1 {
                inside_polygon = true;
            }
        }

        self.max_height = (max_y - min_y) as u32;
        self.max_width = (max_x - min_x) as u32;

        points
    }

    fn get_area(&mut self) -> i32 {
        let mut sum: i32 = 0;
        let mut right_left_count = 0;
        let points = self.get_points();
        let point_count = points.len() - 1;
        for (i, p) in points.iter().enumerate() {
            sum += (p.y + points[(i+1) % point_count].y) * (p.x - points[(i+1) % point_count].x);
            // dbg!(p, sum);
        }

        sum / 2
    }

    fn print_map(&mut self) {
        let mut map = vec![vec!['.'; (self.max_width * 3) as usize]; (self.max_height * 3) as usize];

        let points = self.get_points();
        let mut last_point = Pos { x: 0, y: 0 };
        for p in points {
            draw(&mut map, last_point, p);
            last_point = p;
        }

        // for p in self.get_points() {
        //     let x = p.x + self.max_width as i32;
        //     let y = p.y + self.max_height as i32;
        //     map[y as usize][x as usize] = '#';
        // }

        for r in map {
            for c in r {
                print!("{c}");
            }

            println!("");
        }
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let mut dp = DigPlan::new(data.lines().map(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        // dbg!(&split[0]);
        DigPlanItem { 
            dir: match split[0].trim() {
                "U" => DIR::UP,
                "D" => DIR::DOWN,
                "L" => DIR::LEFT,
                "R" => DIR::RIGHT,
                _ => DIR::UNKNOWN,
            },
            dist: split[1].parse::<i32>().unwrap(),
            hex: split[2][1..split[2].len()-1].to_string(),
        }
    }).collect());

    let points = dp.get_points();
    // dbg!(&points);

    // let map = vec![vec!['.'; dp.max_width as usize]; dp.max_height as usize];
    // dbg!(map);

    dp.print_map();
    dp.get_area().to_string()

    // "p1".to_string()
}
