use std::fs;
use std::str::Lines;

fn main() {
    let contents = fs::read_to_string("input-day2.txt")
        .expect("Should have been able to read file");

    let reports = parse_reports(contents);

    // --------- Part 1 -----------

    let mut total_passes = 0;
    for levels in &reports {
        if levels.len() < 2 {
            continue;
        }
        if levels[0] == levels[1] {
            continue;
        }

        let is_increasing = if levels[0] < levels[1] {
            true
        } else {
            false
        };

        let mut passes = true;
        for number in 0..(levels.len()-1) {
            let diff: i32 = levels[number + 1] - levels[number];
            if is_increasing && (diff < 1 || diff > 3) {
                passes = false;
                break;
            }
            if !is_increasing && (diff > -1 || diff < -3) {
                passes = false;
                break;
            }
        }
        if passes {
            total_passes += 1;
                    
        }
    }

    println!("Total Passes: {total_passes:?}");

    // --------- Part 2 -----------
    let mut total_passes = 0;

    for report in &reports {
        let size = report.len();
        let score = get_score(&report[..]);

        if score.abs()  == (size as i32 - 1) {
            total_passes += 1;
            continue;
        }

        for i in 0..size {
            let sub_size = size - 1;
            let mut score = 0;
            if i == 0 {
                score = get_score(&report[1..size]);
            }
            if i == size - 1 {
                score = get_score(&report[0..size - 1]);
            }

            if i > 0 && i < size - 1 {
                score = get_score(&[&report[0..i], &report[i+1..size]].concat());
            }

            if score.abs()  == (sub_size as i32 - 1) {
                total_passes += 1;
                break;
            }

        }
    }
    
    println!("Total Passes: {total_passes}");
}

fn get_score(report: &[i32]) -> i32 {
    let size = report.len();

    if size < 2 {
        return 0;
    }
    if report[0] == report[1] {
        return 0;
    }

    let mut score: i32 = 0;
    for number in 0..(size - 1) {
        score += delta(report[number], report[number + 1]);
    }

    return score;
}

fn delta(a: i32, b: i32) -> i32 {
    let diff: i32 = a - b;
    if diff > 0 && diff < 4 {
        return 1;
    }
    if diff < 0 && diff > -4 {
        return -1;
    }
    return 0;
}

fn parse_reports(contents: String) -> Vec<Vec<i32>> {
    let contents: Lines = contents.lines();

    contents.map(|line| {
        let line = line.split(" ");

        line.map(|number| {
            number.parse::<i32>()
                .expect("Not a number")
        }).collect()
    }).collect()
}
