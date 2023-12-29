use std::{ops::{Range, Index}, thread::current};

use aoc_utils::read_file;

fn slide_windows_starting_at(windows: &mut Vec<Range<usize>>, curr_index: usize) {
    for i in curr_index..windows.len() {
        windows[i] = Range {
            start: windows[i].start+1,
            end: windows[i].end+1,
        }
    }
}

fn slide_window(windows: &mut Vec<Range<usize>>, curr_index: usize, distance: usize) {
    windows[curr_index] = Range {
        start: windows[curr_index].start+distance,
        end: windows[curr_index].end+distance,
    }
}

#[derive(Debug)]
struct SpringConditionRecord {
    spring_state: String,
    cgds: Vec<usize>, // Contiguous Group sizes of Damaged Springs
    damaged_spring_ranges: Vec<Range<usize>>,
    total_damaged_springs: usize,
    allowed_start_ranges: Vec<Range<usize>>,

}

impl SpringConditionRecord {
    fn new(spring_state: &str, cgds: Vec<usize>) -> SpringConditionRecord {
        let mut sr = SpringConditionRecord {
            spring_state: spring_state.to_string(),
            cgds: cgds.clone(),
            damaged_spring_ranges: vec![],
            total_damaged_springs: cgds.into_iter().fold(0, |acc, s| acc + s),
            allowed_start_ranges: vec![],
        };

        let mut broken_spring_index = vec![];
        for (i, c) in sr.spring_state.chars().enumerate() {
            if c == '#' {
                broken_spring_index.push(i);
            }
        }


        sr.allowed_start_ranges = sr.get_allowed_start_ranges();

        sr
    }

    fn is_combination_valid(&self, windows: &Vec<Range<usize>>, window_index: usize) -> bool {
        // let last_i = self.spring_state.len() - 1;


        let w = &windows[window_index];
        if self.spring_state[w.start..w.end].contains('.') {
            // dbg!("1 false");
            return false;
        } 

        // Check one tile left and right of current range for 'blocking' broken springs
        let last_i = self.spring_state.len() - 1;
        if  w.end < last_i && &self.spring_state[w.end..w.end+1] == "#" || 
            w.start > 0 && &self.spring_state[w.start-1..w.start] == "#" {
            return false;
        }

        // Check for 'broken spring' not specified by one of the provided ranges
        for (i, c) in self.spring_state.chars().enumerate() {
            if c == '#' && !windows.iter().any(|w| w.contains(&i)) {
                // dbg!("3 false");
                return false;
            }
        }

        true
    }

    fn count_valid_combinations(&self, current_windows: &mut Vec<Range<usize>>, window_index: usize, depth: usize) -> usize {
        assert!(current_windows.len() > 1);
        assert!(current_windows.len() == self.allowed_start_ranges.len());
        assert!(window_index < current_windows.len());

        let mut sum = 0;

        // println!("> Depth: {}", depth);
        // dbg!(&current_windows);

        // Reached final window, must check all combinations of final window
        // with the given `current_windows` and return sum
        let last_index = current_windows.len() - 1;
        if window_index == last_index {
            // println!("REACHED FINAL WINDOW: {:?}", current_windows);

            // current_windows[last_index] = self.allowed_start_ranges[last_index].start..self.allowed_start_ranges[last_index].start+self.cgds[last_index];

            // Iterate from the 2nd to last endpoint+1 to the maximum allowed index for the final window
            for _i in current_windows[last_index].start+1..=self.allowed_start_ranges[last_index].end+1 {
            // for _i in self.allowed_start_ranges[last_index].start..=self.allowed_start_ranges[last_index].end {
                // dbg!(_i);
                // current_windows[last_index] = _i.._i+self.cgds[last_index];
                let is_valid = self.is_combination_valid(current_windows, last_index);
                if is_valid {
                    sum += 1;
                }
                // println!("{:?} - Checked combination: {:?} - {}", self.allowed_start_ranges, current_windows, if is_valid { "MATCH" } else {""});
                // println!("Checked combination: {:?} - {}", current_windows, if is_valid { "MATCH" } else {""});

                // Move final window right by 1
                slide_window(current_windows, window_index, 1);
            }

            // Reset final window position
            // println!("RESET FINAL WINDOW!, {:?}", current_windows);
            current_windows[last_index] = Range {
                start: current_windows[last_index-1].end + 1,
                end: current_windows[last_index-1].end + 1 + current_windows[last_index].len(),
            };
        } else {
            for _i in current_windows[window_index].start+1..=self.allowed_start_ranges[window_index].end {

                // Skip recursing if `current_window` can't possibly match
                let w = &current_windows[window_index];
                if self.spring_state[w.start..w.end].contains('.') {
                    // dbg!("1 false");
                    slide_windows_starting_at(current_windows, window_index);
                    continue
                } 

                // Check one tile left and right of current range for 'blocking' broken springs
                let last_i = self.spring_state.len() - 1;
                if  w.end < last_i && &self.spring_state[w.end..w.end+1] == "#" || 
                    w.start > 0 && &self.spring_state[w.start-1..w.start] == "#" {
                    slide_windows_starting_at(current_windows, window_index);
                    continue;
                }


                // Check if `current_window` has passed a set of required "#" (damaged springs)
                // and none of the other `windows` account for them, if so the potential matches
                // can be discarded



                sum += self.count_valid_combinations(current_windows, window_index+1, depth+1); 
                // println!("SLIDING WINDOW: {:?}", &current_windows);
                // Reset window positions
                slide_windows_starting_at(current_windows, window_index);
                // println!("SLID WINDOWS + 1: {:?}", &current_windows);
            }

            // println!("RESET WINDOWS: {:?}, windex: {}", current_windows, window_index);
            for i in window_index..current_windows.len() {

                if i == 0 { continue }

                current_windows[i] = Range {
                    start: current_windows[i-1].end + 1,
                    end: current_windows[i-1].end + 1 + self.cgds[i],
                };
            }

            // println!("! RESETTED WINDOWS: {:?}", current_windows);
        }

        sum
    }

    fn get_allowed_start_ranges(&self) -> Vec<Range<usize>> {
        let mut allowed_window_start_ranges: Vec<Range<usize>> = vec![];

        // Enumerate possible ranges required by '.' gap between windows
        for (i, _s) in self.cgds.iter().enumerate() {
            let left_sum: usize = self.cgds[..i].iter().sum::<usize>();
            let right_sum: usize = self.cgds[i..].iter().sum::<usize>();

            allowed_window_start_ranges.push(Range {
                start: left_sum + i,
                end: (self.spring_state.len()) - right_sum ,
            });
        }


        // FROM LEFT SIDE
        // Identify stricter ranges based on '#' sequences in `spring_state`
        // for the 'first' and the 'last' windows (other interanal windows) either
        // can't be 'locked' or it will be harder to determine
        let r = &allowed_window_start_ranges[0];
        let first_bad_spring = &self.spring_state[r.start..r.end].find('#');
        let last_bad_spring: Option<usize>;


        // Find contiguous range of bad springs (from left side) and limit current
        // range (`r`) to overlap the hardcoded contiguous range
        if let Some(mut i) = first_bad_spring {
            // Find length of 'bad spring' sequence
            // let mut i: usize = first_bad_spring.unwrap();
            loop {
                i += 1;
                if self.spring_state.chars().nth(i).unwrap() != '#' || i >= r.end {
                    last_bad_spring = Some(i);
                    break
                }
            }


            // TODO: make sure to count any '?' at the front to see if the 
            // restricted sequence can be applied
            
            // Unknown spring count before first bad spring
            let  unknown_count = self.spring_state
                .chars()
                .collect::<Vec<char>>()[0..r.start]
                .iter()
                .fold(0, |acc, c| if *c == '?' { acc + 1 } else { acc });

            if unknown_count < self.cgds[0] {
                // println!("RESTRICTING RANGE[0]");
                if let (Some(a), Some(b)) = (first_bad_spring, last_bad_spring) {
                    let size_diff = (r.end-r.start) - (b-a);
                    let start: usize;
                    if size_diff > *a {
                        start = 0;
                    } else {
                        start = a - size_diff;
                    }
                    allowed_window_start_ranges[0] = Range {
                        start,
                        end: *a+1,
                    }
                }
            }
        }


        // // FROM RIGHT SIDE
        // // Identify stricter ranges based on '#' sequences in `spring_state`
        // // for the 'first' and the 'last' windows (other interanal windows) either
        // // can't be 'locked' or it will be harder to determine
        // let r = &allowed_window_start_ranges[allowed_window_start_ranges.len()-1];
        // let mut last_bad_spring = self.spring_state[r.start..].rfind('#');
        // let first_bad_spring: Option<usize>;


        // // Find contiguous range of bad springs (from left side) and limit current
        // // range (`r`) to overlap the hardcoded contiguous range
        // if let Some(mut i) = last_bad_spring {
        //     // Find length of 'bad spring' sequence
        //     // let mut i: usize = first_bad_spring.unwrap();
        //     // last_bad_spring = Some(self.spring_state.len() - 1 - i);

        //     // Fix `i`
        //     last_bad_spring = Some(i + r.start);
        //     i = i + r.start;

        //     loop {
        //         if self.spring_state.chars().nth(i).unwrap() != '#' || i < r.start {
        //             first_bad_spring = Some(i+1);
        //             break
        //         }
        //         i -= 1;
        //     }

        //     // Unknown spring count before first bad spring (from right to left)
        //     let  unknown_count = self.spring_state
        //         .chars()
        //         .rev()
        //         .collect::<Vec<char>>()[0..self.spring_state.len() - first_bad_spring.unwrap()]
        //         .iter()
        //         .fold(0, |acc, c| if *c == '?' { acc + 1 } else { acc });

        //     // if dbg!(unknown_count) < dbg!(allowed_window_start_ranges.last().unwrap().len()) {
        //     if dbg!(unknown_count) < dbg!(*self.cgds.last().unwrap()) {
        //         println!("RESTRICTING RANGE[-1]");
        //         if let (Some(a), Some(b)) = (first_bad_spring, last_bad_spring) {
        //             // dbg!(a, b, self.cgds.last());
        //             let size_diff = (self.cgds.last().unwrap()).abs_diff(b-a+1);
        //             // dbg!(&self.spring_state, r, first_bad_spring, last_bad_spring, size_diff);
        //             // println!("{:?}, {:?}, {:?}, {:?}, {:?}", &self.spring_state, r, first_bad_spring, last_bad_spring, size_diff);
        //             let end: usize;
        //             if a + size_diff + self.cgds.last().unwrap() > self.spring_state.len() - 1 {
        //                 end = self.spring_state.len() - 1 - self.cgds.last().unwrap();
        //             } else {
        //                 // start = a - size_diff;
        //                 // end = b + size_diff;
        //                 end = b;
        //             }
        //             dbg!(a, b, end, size_diff, self.cgds.last());
        //             allowed_window_start_ranges[self.cgds.len()-1] = Range {
        //                 // start: a - size_diff,
        //                 start: a,
        //                 end,
        //                 // end: b.min(self.spring_state.len()-1 - self.cgds.last().unwrap()),
        //             }
        //         }
        //     }
        // }

        allowed_window_start_ranges
    }

    pub fn get_valid_combination_count(&self) -> usize {
        let mut sliding_windows: Vec<Range<usize>> = vec![];
        let allowed_window_start_ranges = self.get_allowed_start_ranges();
        // println!("ALLOWED RANGES: {:?}", &allowed_window_start_ranges);


        // Set initial sliding window positions
        for (i, r) in allowed_window_start_ranges.iter().enumerate() {
            sliding_windows.push(r.start..r.start + self.cgds[i]);
        }

        // TODO: check if sliding window can be 'fixed' to a location
        // in particular the first and last 'windows'

        // dbg!(&sliding_windows);

        // Calculate possible combinations based on known state
        // and any 'fixed' sliding windows or sub-windows
        self.count_valid_combinations(&mut sliding_windows, 0, 0)
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let records: Vec<SpringConditionRecord> = data.lines()
        .map(|l| {
            let split: Vec<_> = l.split_whitespace().collect();
            SpringConditionRecord::new(
                &format!("{}?", &split[0]).repeat(5)[..(split[0].len()+1)*5 - 1],
                format!("{},", &split[1]).repeat(5)[..(split[1].len()+1)*5 - 1].split(',').map(|n| n.parse::<usize>().unwrap()).collect(),
                // split[0],
                // split[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect()
            )
        })
        .collect();

    
    // println!("{:?}", records);

    let mut i = 0;
    records.into_iter().fold(0, |acc, r| {
        println!("{i}\tTESTING: {:?}", r.spring_state);

        let count = r.get_valid_combination_count();
        println!("\tCOUNT: {}", count);
        i += 1;
        acc + count
    }).to_string()

    // "p2".to_string()
}
