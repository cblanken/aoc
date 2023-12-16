use std::{ops::Range, thread::current};

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
    total_springs: usize,
    allowed_start_ranges: Vec<Range<usize>>,
}

impl SpringConditionRecord {
    fn new(spring_state: &str, cgds: Vec<usize>) -> SpringConditionRecord {
        let mut sr = SpringConditionRecord {
            spring_state: spring_state.to_string(),
            cgds: cgds.clone(),
            total_springs: cgds.into_iter().fold(0, |acc, s| acc + s),
            allowed_start_ranges: vec![],
        };

        sr.allowed_start_ranges = sr.get_allowed_start_ranges();

        sr
    }

    fn is_combination_valid(&self, windows: &Vec<Range<usize>>) -> bool {
        let last_i = self.spring_state.len() - 1;
        for w in windows {
            if self.spring_state[w.start..w.end].contains('.') {
                // dbg!("1 false");
                return false;
            } 

            // Check one tile left and right of current range for 'blocking' broken springs
            if  (w.start > 0 && w.end < last_i && &self.spring_state[w.start..=w.start+1] == "#") ||
                // (w.end < last_i && &self.spring_state[last_i..last_i+1] == "#") || // TODO FIX THIS?
                (w.start == 0 && &self.spring_state[w.end..w.end+1] == "#") ||
                (w.end >= last_i && &self.spring_state[w.start-1..w.start] == "#") {
                // dbg!("2 false", w);
                return false;
            }

            // Check for 'broken spring' not specified by one of the provided ranges
            for (i, c) in self.spring_state.chars().enumerate() {
                if c == '#' && !windows.iter().any(|w| w.contains(&i)) {
                    // dbg!("3 false");
                    return false;
                }
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

            // Iterate from the 2nd to last endpoint+1 to the maximum allowed index for the final window
            // for _i in current_windows[last_index-1].end+1..self.allowed_start_ranges[last_index].end {
            for _i in current_windows[last_index].start+1..=self.allowed_start_ranges[last_index].end+1 {
                if self.is_combination_valid(current_windows) {
                    sum += 1;
                }
                println!("Checked combination: {:?} - {}", current_windows, if self.is_combination_valid(current_windows) { "MATCH" } else {""});

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
            let windows_copy = current_windows.clone();
            for _i in current_windows[window_index].start+1..=self.allowed_start_ranges[window_index].end {
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
        for (i, _s) in self.cgds.iter().enumerate() {
            let left_sum: usize = self.cgds[..i].iter().sum::<usize>();
            let right_sum: usize = self.cgds[i..].iter().sum::<usize>();

            allowed_window_start_ranges.push(Range {
                start: left_sum + i,
                end: (self.spring_state.len()) - right_sum ,
            });
        }

        allowed_window_start_ranges
    }

    pub fn get_valid_combination_count(&self) -> usize {
        let mut sliding_windows: Vec<Range<usize>> = vec![];
        let allowed_window_start_ranges = self.get_allowed_start_ranges();
        println!("ALLOWED RANGES: {:?}", &allowed_window_start_ranges);


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
                    // .split(',').map(|n| n.parse::<usize>().unwrap()).collect(),
                // split[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect()
            )
        })
        .collect();

    // dbg!(&records[0]);
    
    records.into_iter().fold(0, |acc, mut r| {
        let count = r.get_valid_combination_count();
        println!("COUNT > {}", count);
        acc + count
    }).to_string()

    // "p2".to_string()
}
