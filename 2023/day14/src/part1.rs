use std::fmt;
use std::ops::Range;

use aoc_utils::read_file;

struct Pos(usize, usize);

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


#[derive(Debug)]
struct ReflectorMap {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    north_south_ranges_between_pillars: Vec<Vec<RangeCount>>,
}

/// A tuple struct with a range and an associated `count`
#[derive(Clone, Debug)]
struct RangeCount(Range<usize>, usize);

fn get_load_sum_from_range_count(r: RangeCount, map_height: usize) -> usize {
    (0..r.1).fold(0, |acc, i| acc + (map_height - r.0.start - i))
}

impl ReflectorMap {
    fn new(map: Vec<Vec<char>>) -> Self {
        assert!(map.len() > 0);
        assert!(map[0].len() > 0);
        let mut rmap = Self {
            height: map.len(),
            width: map[0].len(),
            map,
            north_south_ranges_between_pillars: vec![],
        };

        rmap.north_south_ranges_between_pillars = rmap.find_north_south_ranges_per_column();

        rmap
    }

    fn find_north_south_ranges_per_column(&self) -> Vec<Vec<RangeCount>> {
        let mut ranges = vec![vec![]; self.width];
        for c in 0..self.width {
            let mut start_row = 0;
            let mut rounded_rocks_cnt = 0;
            for r in 0..self.height {
                if self.map[r][c] == 'O' {
                    rounded_rocks_cnt += 1;
                } else if self.map[r][c] == '#' && rounded_rocks_cnt > 0 {
                    // Stop and add new range
                    ranges[c].push(RangeCount(start_row..r, rounded_rocks_cnt));
                    start_row = r+1;
                    rounded_rocks_cnt = 0;
                } else if self.map[r][c] == '#' {
                    start_row = r+1;
                    rounded_rocks_cnt = 0;
                }

                // Catch range ending at bottom of map
                if r == self.height - 1 && rounded_rocks_cnt > 0  {
                    ranges[c].push(RangeCount(start_row..r, rounded_rocks_cnt));
                }
            }
        }

        ranges
    }

    fn get_load_on_north_support(self) -> usize {
        self.north_south_ranges_between_pillars
            .into_iter()
            .fold(0, |acc, col_vec| acc + col_vec
                .into_iter()
                .fold(0, |range_acc, r| range_acc + get_load_sum_from_range_count(r, self.height)))
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let map = ReflectorMap::new(data
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect());

    map.get_load_on_north_support().to_string()

    // "part1".to_string()
}
