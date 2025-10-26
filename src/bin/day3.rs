use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input-day3.txt")
        .expect("Should be able to read file");

    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let muls: Vec<(i32, i32)> = re.captures_iter(&contents).map(|caps| {
        let (_group, [a, b]) = caps.extract();
        let a: i32 = a.parse()
            .expect("First int not a number");
        let b: i32 = b.parse()
            .expect("Second int not a number");
        (a, b)
    }).collect();

    let total = muls.iter().fold(0, |total, (a, b)| total + a * b);

    println!("Total {total:?}");

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))").unwrap();
    
    let mut should_proccess = true;
    let mut total = 0;
    for captures in re.captures_iter(&contents) {
        let (group, [_]) = captures.extract();
        if group == "do()" {
            should_proccess = true;
            continue;
        }
        if group == "don't()" {
            should_proccess = false;
            continue;
        }

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let Some((_group, [a, b])) = re.captures(group).map(|caps| caps.extract()) else {
            panic!("Error extracting");
        };

        if should_proccess {
            let a: i32 = a.parse()
                .expect("First int not a number");
            let b: i32 = b.parse()
                .expect("Second int not a number");
            total += a * b
        }
    }

    println!("Total {total:?}");
}
