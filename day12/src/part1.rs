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
    // total_springs: usize,
    allowed_start_ranges: Vec<Range<usize>>,
}

impl SpringConditionRecord {
    fn new(spring_state: &str, cgds: Vec<usize>) -> SpringConditionRecord {
        let mut sr = SpringConditionRecord {
            spring_state: spring_state.to_string(),
            cgds: cgds.clone(),
            // total_springs: cgds.into_iter().fold(0, |acc, s| acc + s),
            allowed_start_ranges: vec![],
        };

        sr.allowed_start_ranges = sr.get_allowed_start_ranges();

        sr
    }

    fn is_combination_valid(&self, windows: &Vec<Range<usize>>) -> bool {
        for w in windows {
            if self.spring_state[w.start..w.end].contains('.') {
                return false
            }
        }

        true
    }

    fn count_valid_combinations(&self, current_windows: &mut Vec<Range<usize>>, window_index: usize, depth: usize) -> usize {
        assert!(current_windows.len() > 1);
        assert!(current_windows.len() == self.allowed_start_ranges.len());
        assert!(window_index < current_windows.len());

        let mut sum = 0;
        // let current_window = &current_windows[window_index];

        println!("> Depth: {}", depth);
        // dbg!(&current_windows);

        // Reached final window, must check all combinations of final window
        // with the given `current_windows` and return sum
        let last_index = current_windows.len() - 1;
        if window_index == last_index {
            println!("REACHED FINAL WINDOW: {:?}", current_windows);

            // Iterate from the 2nd to last endpoint+1 to the maximum allowed index for the final window
            // for _i in current_windows[last_index-1].end+1..self.allowed_start_ranges[last_index].end {
            for _i in current_windows[last_index].start+1..=self.allowed_start_ranges[last_index].end {
                if self.is_combination_valid(current_windows) {
                    sum += 1;
                }
                println!("Checked combination: {:?} - {}", current_windows, sum);

                // Move final window right by 1
                slide_window(current_windows, window_index, 1);
            }

            // Reset final window position
            current_windows[last_index] = Range {
                start: current_windows[last_index-1].end + 1,
                end: current_windows[last_index-1].end + 1 + current_windows[last_index].len(),
            };

            return sum;

        } else if current_windows[window_index].start <= self.allowed_start_ranges[window_index].end {
            // Current window still in allowed range and can be slid to the right
                sum += self.count_valid_combinations(current_windows, window_index+1, depth+1); 
                println!("SLIDING WINDOW: {:?}", &current_windows);
                slide_windows_starting_at(current_windows, window_index);
                println!("SLID WINDOW + 1: {:?}", &current_windows);
                // current_windows[window_index].start += 1;
                // current_windows[window_index].end += 1;
                // Current window can't be slid to the right anymore, so return
        } else {
            println!("CANT SLIDE WINDOW ANYMORE: {:?}", current_windows);
            return sum;
        }

        // return sum;
        sum + self.count_valid_combinations(current_windows, window_index+1, depth+1)
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
                split[0],
                split[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect()
            )
        })
        .collect();

    // dbg!(&records[0]);
    
    records.iter().fold(0, |acc, r| {
        let count = r.get_valid_combination_count();
        println!("COUNT > {}", count);
        acc + count
    }).to_string()
}
