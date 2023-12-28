use aoc_utils::{
    read_file,
};

use std::num::ParseIntError;
use std::fmt;
use std::ops::Range;


fn get_ranges(src_range: Range<u64>, seed_range: Range<u64>) -> Vec<Range<u64>> {
    // No overlap
    if seed_range.end <= src_range.start  || seed_range.start >= src_range.end {

    // Exact or inner overlap of seed over source range
    } else if seed_range.start >= src_range.start && seed_range.end <= src_range.end {

    // Seed range is less than source range with overlap
    } else if seed_range.end > src_range.start && seed_range.end < src_range.end {

    // Seed range is greater than source range with overlap
    } else if seed_range.start > src_range.start && seed_range.start < src_range.end {

    // Seed range overlaps source range on left and right
    } else if seed_range.start < src_range.start && seed_range.end > src_range.end {

    }
}

#[derive(Debug)]
struct AlmanacMap {
    src_name: String,
    dst_name: String,
    map_ranges: Vec<Vec<u64>>,
}

impl AlmanacMap {
    fn new(src_name: &str, dst_name: &str, map_ranges: Vec<Vec<u64>>) -> AlmanacMap {
        AlmanacMap { 
            src_name: src_name.to_string(),
            dst_name: dst_name.to_string(),
            map_ranges,
        }
    }

    fn apply(&self, seed: u64) -> u64 {
        for (i, map_range) in self.map_ranges.iter().enumerate() {
            // dbg!(map_range);
            if map_range.len() == 0 {
                println!("EMPTY MAP RANGE: {} {} @ {}", self.src_name, self.dst_name, i);
                return seed;
            }
            let dst_start = map_range[0];
            let src_start = map_range[1];
            let length = map_range[2];

            if seed >= src_start && seed < src_start + length {
                // println!("UPDATE: {}({}) -> {}({})", self.src_name, seed, self.dst_name, seed - src_start + dst_start);
                return seed - src_start + dst_start;
            }
        }

        seed
    }

    // fn apply_range(&self, seed_range: Range<u64>) {
    //     for (i, map_range) in self.map_ranges.iter().enumerate() {
    //         if map_range.len() == 0 {
    //             println!("EMPTY MAP RANGE: {} {} @ {}", self.src_name, self.dst_name, i);
    //             return seed;
    //         }
    //         let dst_start = map_range[0];
    //         let src_start = map_range[1];
    //         let length = map_range[2];

    //         if seed >= src_start && seed < src_start + length {
    //             // println!("UPDATE: {}({}) -> {}({})", self.src_name, seed, self.dst_name, seed - src_start + dst_start);
    //             return seed - src_start + dst_start;
    //         }
    //     }
    // }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    match &data.split("\n\n").collect::<Vec<&str>>()[..] {
        [ seeds, maps @ .. ] => {
            let seed_nums: Vec<u64> = seeds
                .trim()
                .split_whitespace()
                .map(|s| s.trim().parse::<u64>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();

            // TODO fix this. use iterators
            let mut seed_pairs: Vec<(u64, u64)> = vec![];
            for i in 0..seed_nums.len()-1 {
                if i % 2 == 0 {
                    seed_pairs.push((seed_nums[i], seed_nums[i+1]))
                }
            }


            let almanac_maps: Vec<AlmanacMap> = maps.into_iter().map(|m| {
                let lines: Vec<&str> = m.split('\n').collect();
                let [ src_name, dst_name ] = &lines[0]
                    .split_whitespace()
                    .collect::<Vec<&str>>()[0]
                    .split("-to-")
                    .collect::<Vec<&str>>()[..]
                else {
                    panic!("Couldn't parse map src_name and dst_name!")
                };

                let map_ranges: Vec<Vec<u64>> = lines[1..]
                    .into_iter()
                    .map(|r| {
                        r.trim().split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect()
                    })
                    .filter(|r: &Vec<u64>| r.len() > 0)
                    .collect();


                AlmanacMap::new(src_name, dst_name, map_ranges)

            }).collect();

            // for m in &almanac_maps {
            //     dbg!(m);
            // }



            // TODO: Too slow, need to do range comparisons
            // not check every single seed value
            // let mut locations: Vec<u64> = vec![];
            let mut min_location = u64::MAX;
            for pair in seed_pairs {
                println!("SEED PAIR, {},{}", pair.0, pair.1);
                let seeds = pair.0 .. pair.0 + pair.1;
                for s in seeds {
                    // println!("SEED {s}");
                    let mut out = s;
                    for m in &almanac_maps {
                        out = m.apply(out);
                    }

                    if out < min_location { 
                        min_location = out;
                        println!("> NEW MIN LOCATION: {min_location}");
                    }
                    // locations.push(out);
                }

            }

            return min_location.to_string();
            // return locations.into_iter().fold(u64::MAX, |acc, l| if l < acc { l } else { acc } ).to_string();
        },
        [] => panic!("Could not parse seeds and maps"),

    }

}
