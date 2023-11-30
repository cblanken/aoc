use std::fs;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Could not read file");

    contents
}

pub fn print_file(path: &str) {
    let contents = read_file(path);

    let hr = "=".repeat(12 + path.len());
    println!("{hr}");
    println!("INPUT FILE: {path}");
    println!("{hr}");
    for l in contents.lines() {
        println!("{l}");
    }

    println!("{hr}");
    println!("END FILE");
    println!("{hr}");
}

