use aoc_utils::print_file;
mod part1;
// mod part2;

static INPUT_FILE_PATH: &str = "sample1.txt";

fn main() {
    print_file(INPUT_FILE_PATH);
    println!("");
    println!("Part 1: {}", part1::solve(INPUT_FILE_PATH));
    // println!("Part 2: {}", part2::solve(INPUT_FILE_PATH));
}

