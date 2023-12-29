use std::fmt;

use aoc_utils::{
    read_file,
};

struct Pos(usize, usize);

struct MirrorMap {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    vertical_reflections: Vec<usize>,
    horizontal_reflections: Vec<usize>,
}

impl MirrorMap {
    fn new(map: Vec<Vec<char>>) -> MirrorMap {
        let mut mmap = MirrorMap {
            map, width: 0,
            height: 0,
            vertical_reflections: vec![],
            horizontal_reflections: vec![]
        };

        assert!(mmap.map.len() > 1);
        assert!(mmap.map[0].len() > 1);
        mmap.height = mmap.map.len();
        mmap.width = mmap.map[0].len();
        mmap.vertical_reflections = mmap.find_vertical_reflections(false);
        mmap.horizontal_reflections = mmap.find_horizontal_reflections(false);

        mmap
    }

    fn summarize(&self) -> usize {
        // Find all horizontal reflections
        let h_reflections: Vec<usize> = self.find_horizontal_reflections(false);
        let h_sum = h_reflections.iter().fold(0, |acc, ri| acc + ri);

        // Find all vertical reflections
        let v_reflections: Vec<usize> = self.find_vertical_reflections(false);
        let v_sum = v_reflections.iter().fold(0, |acc, hi| acc + hi);

        h_sum + 100 * v_sum
    }

    fn summarize_with_smudge(&self) -> usize {
        // Find all horizontal reflections
        let h_reflections: Vec<usize> = self.find_horizontal_reflections(true);
        let h_sum = h_reflections.iter().fold(0, |acc, ri| acc + ri);

        // Find all vertical reflections
        let v_reflections: Vec<usize> = self.find_vertical_reflections(true);
        let v_sum = v_reflections.iter().fold(0, |acc, hi| acc + hi);

        h_sum + 100 * v_sum
    }

    fn is_horizontal_reflection(&self, col: usize, check_smudge: bool) -> bool {
        // Column index is actually between indexes of map
        // so the `col` will refer to the space between
        // `col` + 1 and `col` on the `map`
        assert!(col > 0);
        assert!(col < self.width);


        // TODO: fix first error found
        // if fixing single error (smudge) still resolves to a 
        // valid reflection, then keep it
        // otherwise reset error count to 0
        // and continue to next 
        //
        // might need to return status for whether a smudge
        // was found or not
        //
        // maybe assert!() that at least 2 different reflections
        // are found 

        let mut smudges_found: u8 = 0;

        // d = distance from center line
        let mut is_reflected_between_col = true;
        for d in 0..=col.min(self.width-1-col) {
            for r in 0..self.height {
                if col-d == 0 {
                    break
                }
                if self.map[r][col-d-1] != self.map[r][col+d] {
                    if check_smudge && smudges_found == 0 {
                        // Ignore first smudge
                        smudges_found += 1;
                        continue
                    }
                    is_reflected_between_col = false;
                    break
                }
            }
        }

        is_reflected_between_col
    }

    fn is_vertical_reflection(&self, row: usize, check_smudge: bool) -> bool {
        // Row index is actually between indexes of map
        // so the `row` will refer to the space between
        // `row` + 1 and `row` on the `map`
        assert!(row > 0);
        assert!(row < self.height);

        let mut smudges_found: u8 = 0;

        // d = distance from center line
        let mut is_reflected_between_row = true;
        for d in 0..row.min(self.height-row) {
            for c in 0..self.width {
                if self.map[row-d-1][c] != self.map[row+d][c] {
                    // dbg!("NOT REFLECTED BETWEEN ROW", row, d, c, self.map[row-d-1][c], self.map[row+d][c]);
                    if check_smudge && smudges_found == 0 {
                        // Ignore first smudge
                        smudges_found += 1;
                        continue
                    }
                    is_reflected_between_row = false;
                    break
                }
            }
        }

        is_reflected_between_row
    }

    fn find_vertical_reflections(&self, check_smudge: bool) -> Vec<usize> {
        // Return indexes of vertical reflections
        (1..self.height).filter(|n| self.is_vertical_reflection(*n, check_smudge)).collect()
        
    }

    fn find_horizontal_reflections(&self, check_smudge: bool) -> Vec<usize> {
        // Return indexes of horizontal reflections
        (1..self.width).filter(|n| self.is_horizontal_reflection(*n, check_smudge)).collect()
    }
}

impl fmt::Display for MirrorMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in &self.map {
            for c in r {
                print!("{c}");
            }
            println!("")
        }

        writeln!(f, "W: {}, H: {}, H-refs: {:?}, V-refs: {:?}\n", self.width, self.height, self.horizontal_reflections, self.vertical_reflections)
    }
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let maps: Vec<MirrorMap> = data.split("\n\n")
        .map(|mirror_map_str| {
            MirrorMap::new(mirror_map_str.split_whitespace().map(|l| l.chars().collect::<Vec<char>>()).collect())
        })
        .collect();

    // dbg!(maps[1].is_horizontal_reflection(1));
    // dbg!(maps[1].is_horizontal_reflection(2));
    // dbg!(maps[1].is_horizontal_reflection(3));
    // dbg!(maps[1].is_horizontal_reflection(4));
    // dbg!(maps[1].is_horizontal_reflection(5));
    // dbg!(maps[1].is_horizontal_reflection(6));
    // dbg!(maps[1].is_horizontal_reflection(7));
    // dbg!(maps[1].is_horizontal_reflection(8));
    // dbg!(maps[0].is_horizontal_reflection(8));
    // dbg!(maps[0].is_horizontal_reflection(9));

    for m in &maps {
        print!("{}", m);
    }

    maps.into_iter().fold(0, |acc, m| acc + (m.summarize_with_smudge() - m.summarize())).to_string()
}
