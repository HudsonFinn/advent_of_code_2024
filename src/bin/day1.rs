use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input-day1.txt")
        .expect("Should have been able to read file");


    let (mut list1, mut list2) = parse_lines(contents);

    // Sort lists
    list1.sort();
    list2.sort();

    // ------------- Part 1 ---------------

    if list1.len() != list2.len() {
        panic!("Lists of different length");
    }

    let mut total_diff: i32 = 0;
    for n in 0..list1.len() {
        let diff = list2[n] - list1[n];
        let diff = diff.abs();
        total_diff += diff;
    }

    println!("Total diff: {total_diff}");

    // ------------- Part 2 ---------------
    let mut list_2_map: HashMap<i32, i32> = HashMap::new();

    for number in list2 {
        if let Some(count) = list_2_map.get(&number) {
            list_2_map.insert(number, count + 1);
        } else {
            list_2_map.insert(number, 1);
        }
    }

    let mut total_similarity = 0;
    for number in list1 {
        if let Some(count) = list_2_map.get(&number) {
            total_similarity += number * *count
        }
    }
    println!("Total similarity: {total_similarity}");


}

fn parse_lines(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split("   ").collect();

        let num1 = split[0];
        let num2 = split[1];

        let num1: i32 = num1.parse()
            .expect("Could not parse first number to int");
        let num2: i32 = num2.parse()
            .expect("Could not parse second number to int");

        list1.push(num1);
        list2.push(num2);
    };

    (list1, list2)
}
