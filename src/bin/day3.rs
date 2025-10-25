use std::fs;

fn main() {
    let contents = fs::read_to_string("input-day3.txt")
        .expect("Should be able to read file");

    println!("{contents}");
}
