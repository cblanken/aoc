use aoc_utils::print_file;
mod part1;
mod part2;

static INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    print_file(INPUT_FILE_PATH);
    println!("");
    // println!("Part 1: {}", part1::solve(INPUT_FILE_PATH));
    println!("Part 2: {}", part2::solve(INPUT_FILE_PATH));
}

// Part 2
// 1_932_313 is too low
// 12_387_506 is too low
// 109_347_579 is too low
// 1416098443 is incorrect
// 2147483647 panicked integer overflow
