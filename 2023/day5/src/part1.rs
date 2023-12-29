use aoc_utils::{
    read_file,
};

use std::num::ParseIntError;
use std::fmt;

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
}

impl fmt::Display for AlmanacMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "src: {}; dst: {}; ranges: {}", self.src_name, self.dst_name, self.map_ranges.len())
    }
}
pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    match &data.split("\n\n").collect::<Vec<&str>>()[..] {
        [ seeds, maps @ .. ] => {
            let seeds: Vec<Result<u64, ParseIntError>> = seeds
                .trim()
                .split_whitespace()
                .map(|s| s.trim().parse::<u64>())
                .filter(|x| x.is_ok())
                .collect();

            println!("Seeds: ");
            for s in &seeds {
                match s {
                    Ok(x) => println!("> {x}"),
                    Err(e) => println!("\tCould not parse seed! {e}"),
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
                    .collect();

                AlmanacMap::new(src_name, dst_name, map_ranges)

            }).collect();

            for m in &almanac_maps {
                println!("{}", m);
            }

            let mut locations: Vec<u64> = vec![];
            for s in &seeds {
                match s.clone() {
                    Ok(x) => {
                        let mut out = x;
                        for m in &almanac_maps {
                            out = m.apply(out);
                        }

                        locations.push(out);
                    },
                    Err(e) => {
                        println!("BAD SEED VALUE. {}", e);
                    }
                };
            }

            return locations.into_iter().fold(u64::MAX, |acc, l| if l < acc { l } else { acc } ).to_string();
        },
        [] => panic!("Could not parse seeds and maps"),

    }

}
