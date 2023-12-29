use std::fmt;
use std::ops::Range;

use aoc_utils::read_file;

struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
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
    horizontal_pillar_ranges: Vec<Vec<Range<usize>>>,
    vertical_pillar_ranges: Vec<Vec<Range<usize>>>,
    // north_south_ranges_between_pillars: Vec<Vec<RangeCount>>,
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
            horizontal_pillar_ranges: vec![],
            vertical_pillar_ranges: vec![],
            // north_south_ranges_between_pillars: vec![],
        };

        // rmap.north_south_ranges_between_pillars = rmap.find_vertical_tilted_ranges_per_column();
        rmap.horizontal_pillar_ranges = rmap.find_horizontal_pillar_ranges();
        rmap.vertical_pillar_ranges = rmap.find_vertical_pillar_ranges();

        rmap
    }

    // TODO make sure range 

    fn get_west_ranges(&self, ranges_by_column: Vec<Vec<Range<usize>>>) -> Vec<Vec<Range<usize>>> {
        let vpr = self.vertical_pillar_ranges;
        let mut ranges = vec![vec![]; self.height];

        // Take input from North tilt range counts
        for row in 0..self.height {
            for rcbc in ranges_by_column {
                for range in rcbc {
                    if range.contains(&row) {

                    }
                }
            }
        }

        ranges
    }

    fn find_vertical_pillar_ranges(&self) -> Vec<Vec<Range<usize>>> {
        let mut ranges = vec![];

        for c in 0..self.width {
            let mut column_ranges: Vec<Range<usize>> = vec![];
            let mut start_row = 0;
            let mut pillar_cnt = 0;
            for r in 0..self.height {

                if self.map[r][c] == '#' {
                    pillar_cnt += 1;
                }

                if self.map[r][c] == '#' && pillar_cnt == 1 {
                    start_row = r;
                }

                if (pillar_cnt > 0 && self.map[r][c] != '#') ||
                    pillar_cnt > 0 && r == self.height - 1 {
                    column_ranges.push(start_row..start_row+pillar_cnt);
                    pillar_cnt = 0;
                }

            }

            ranges.push(column_ranges);
        }

        ranges
    }

    fn find_horizontal_pillar_ranges(&self) -> Vec<Vec<Range<usize>>> {
        let mut ranges = vec![];

        for r in 0..self.height {
            let mut row_ranges: Vec<Range<usize>> = vec![];
            let mut start_col = 0;
            let mut pillar_cnt = 0;
            for c in 0..self.width {

                if self.map[r][c] == '#' {
                    pillar_cnt += 1;
                }

                if self.map[r][c] == '#' && pillar_cnt == 1 {
                    start_col = c;
                }

                if (pillar_cnt > 0 && self.map[r][c] != '#') ||
                    pillar_cnt > 0 && c == self.width - 1 {
                    row_ranges.push(start_col..start_col+pillar_cnt);
                    pillar_cnt = 0;
                }

            }

            ranges.push(row_ranges);
        }

        ranges
    }

    fn find_vertical_tilted_ranges_per_column(&self) -> Vec<Vec<RangeCount>> {
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


    fn find_horizontal_tilted_ranges_per_column(&self) -> Vec<Vec<RangeCount>> {
        let mut ranges = vec![vec![]; self.width];

        // let range = if dir == DIR::WEST { self.width..0 } else { 0..self.width };
        for r in 0..self.height {
            let mut start_row = 0;
            let mut rounded_rocks_cnt = 0;
            for c in 0..self.width {
                if self.map[r][c] == 'O' {
                    rounded_rocks_cnt += 1;
                } else if self.map[r][c] == '#' {
                    // Stop and add new range
                    ranges[c].push(RangeCount(start_row..r, rounded_rocks_cnt));
                    start_row = r+1;
                    rounded_rocks_cnt = 0;
                }
                // Catch range ending at bottom of map
                if r == self.width - 1 {
                    ranges[c].push(RangeCount(start_row..r, rounded_rocks_cnt));
                }
            }
        }

        ranges
    }

    fn get_load_on_north_support(self) -> usize {
        dbg!(self.find_vertical_tilted_ranges_per_column())
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

    // dbg!(map.find_vertical_pillar_ranges());
    // dbg!(map.find_horizontal_pillar_ranges());


    let horizontal_pillar_ranges = map.find_horizontal_pillar_ranges();
    let vertical_pillar_ranges = map.find_vertical_pillar_ranges();


    for i in 0..1000000000 {
        // infer empty space ranges

        // get_north_ranges(range_counts_by_column, vertical_pillar_ranges);

        // get_west_ranges(range_counts_by_column, horizontal_pillar_ranges);
        // get_south_ranges(range_counts_by_column, horizontal_pillar_ranges);
        // get_east_ranges(range_counts_by_column, horizontal_pillar_ranges);
    }

    map.get_load_on_north_support().to_string()
}
