use std::fmt;
use std::io;
use std::io::prelude::*;

use aoc_utils::read_file;

#[derive(Debug, PartialEq, PartialOrd)]
enum DIR {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Clone, Copy, Debug)]
struct Pos(i32, i32);

type BeamBranch = Vec<Pos>;

struct LightGrid {
    grid: Vec<Vec<char>>,
    egrid: Vec<Vec<u32>>, // energized tile counts
    width: i32,
    height: i32,
    energize_action_count: u32,
    max_depth: u32,
}

impl LightGrid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        let egrid = vec![vec![0; width as usize]; height as usize];
        assert!(height > 0);
        assert!(width > 0);
        Self {
            grid,
            egrid,
            height,
            width,
            energize_action_count: 0,
            max_depth: 10,
        }
    }

    fn energize_cell(&mut self, pos: Pos) {
        self.egrid[pos.0 as usize][pos.1 as usize] += 1;
        self.energize_action_count += 1;
    }

    fn get_energized_tile_count(&self) -> u32 {
        self.egrid.iter()
            .fold(0, |acc, r| acc + r.into_iter().filter(|t| **t > 0).count() as u32)
    }


    fn energize(&mut self, pos: Pos, dir: DIR, depth: u32) {
        if depth > self.max_depth {
            self.max_depth = depth;
        }


        // // DEBUG PROMPTS
        // println!("{self}");
        // let mut buffer = String::new();
        // let stdin = io::stdin();
        // stdin.read_line(&mut buffer);


        // Outside of grid
        if  pos.0 < 0 || pos.0 > self.height - 1 ||
            pos.1 < 0 || pos.1 > self.width - 1 {
            return
        }

        let egrid_target = self.egrid[pos.0 as usize][pos.1 as usize];
        let grid_target = self.grid[pos.0 as usize][pos.1 as usize];

        // Cut off possible loops
        if grid_target == '|' && egrid_target > 0 && (dir == DIR::EAST || dir == DIR::WEST) {
            return
        } else if grid_target == '-' && egrid_target > 0 && (dir == DIR::NORTH || dir == DIR::SOUTH) {
            return
        }

        // Or redirect if mirror ('/', '\') found
        if grid_target == '/' {
            self.energize_cell(pos);
            match dir {
                DIR::NORTH  => self.energize(Pos(pos.0, pos.1+1), DIR::EAST, depth+1),
                DIR::SOUTH  => self.energize(Pos(pos.0, pos.1-1), DIR::WEST, depth+1),
                DIR::EAST   => self.energize(Pos(pos.0-1, pos.1), DIR::NORTH, depth+1),
                DIR::WEST   => self.energize(Pos(pos.0+1, pos.1), DIR::SOUTH, depth+1),
            }

            return
        } else if grid_target == '\\' {
            self.energize_cell(pos);
            match dir {
                DIR::NORTH  => self.energize(Pos(pos.0, pos.1-1), DIR::WEST, depth+1),
                DIR::SOUTH  => self.energize(Pos(pos.0, pos.1+1), DIR::EAST, depth+1),
                DIR::EAST   => self.energize(Pos(pos.0+1, pos.1), DIR::SOUTH, depth+1),
                DIR::WEST   => self.energize(Pos(pos.0-1, pos.1), DIR::NORTH, depth+1),
            }

            return
        }

        // Split beam if splitter ('|', '-') found
        if grid_target == '|' {
            if egrid_target == 0 {
                self.energize_cell(pos);
                match dir {
                    DIR::EAST | DIR::WEST => {
                        self.energize(Pos(pos.0-1, pos.1), DIR::NORTH, depth+1);
                        self.energize(Pos(pos.0+1, pos.1), DIR::SOUTH, depth+1);
                        return
                    },
                    _ => {}
                }
            }
        } else if grid_target == '-' {
            if egrid_target == 0 {
                self.energize_cell(pos);
                match dir {
                    DIR::NORTH | DIR::SOUTH => {
                        self.energize(Pos(pos.0, pos.1+1), DIR::EAST, depth+1);
                        self.energize(Pos(pos.0, pos.1-1), DIR::WEST, depth+1);
                        return
                    },
                    _ => {}
                }
            }
        }

        self.energize_cell(pos);

        // Continue in current direction
        match dir {
            DIR::NORTH  => self.energize(Pos(pos.0-1, pos.1), DIR::NORTH, depth+1),
            DIR::SOUTH  => self.energize(Pos(pos.0+1, pos.1), DIR::SOUTH, depth+1),
            DIR::EAST   => self.energize(Pos(pos.0, pos.1+1), DIR::EAST, depth+1),
            DIR::WEST   => self.energize(Pos(pos.0, pos.1-1), DIR::WEST, depth+1),
        }
    }
}

impl fmt::Display for LightGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "==================================================================").unwrap();
        for r in &self.grid {
            for c in r {
                write!(f, "{c}").unwrap();
            }
            writeln!(f, "").unwrap();
        }

        writeln!(f, "").unwrap();
        for r in &self.egrid {
            for n in r {
                if *n != 0 {
                    write!(f, "{}", n % 10).unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            writeln!(f, "").unwrap();
        }

        writeln!(f, "\nW:{}, H:{}, Max Depth:{}, Energize actions: {}, Energized tiles: {}",
            self.width,
            self.height,
            self.max_depth,
            self.energize_action_count,
            self.get_energized_tile_count()).unwrap();
        writeln!(f, "==================================================================")
    }
}


pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let grid: Vec<Vec<char>> = data
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();

    // dbg!(grid);

    let mut lgrid = LightGrid::new(grid);

    // let mut running_count = 0;
    // for i in 0..1000 {
    // // for i in 158..1000 {
    //     lgrid.egrid = vec![vec![0; lgrid.width as usize]; lgrid.height as usize];
    //     lgrid.max_depth = i;
    //     lgrid.energize(Pos(0, 0), DIR::EAST, 0);
    //     println!("{}", lgrid);
    //     let ecount = lgrid.get_energized_tile_count();
    //     if ecount < running_count {
    //         println!("ECOUNT DROPPED from {running_count} to {ecount}");
    //     }
    //     println!("{i:>5} > ecount: {ecount}");
    //     running_count = ecount;

    //     let mut buffer = String::new();
    //     let stdin = io::stdin(); // We get `Stdin` here.
    //     stdin.read_line(&mut buffer);
    // }

    lgrid.max_depth = 0;
    lgrid.energize(Pos(0, 0), DIR::EAST, 0);
    println!("{}", lgrid);

    lgrid.get_energized_tile_count().to_string()
}
