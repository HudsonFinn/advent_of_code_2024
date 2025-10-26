use std::fs;

fn main() {
    let contents = fs::read_to_string("input-day4.txt")
        .expect("Could not read data from file");

    let contents: Vec<Vec<&str>> = contents.lines().map(|line| {
        let line: Vec<&str> = line.split("").collect();
        let mut vec = Vec::new();
        vec.extend(line[1..line.len()-1].iter());
        vec
    }).collect();

    // ------------ Part 1 ------------

    let height = contents.len();
    let width = contents[0].len();

    let mut matches = 0;
    for y in 0..height {
        for x in 0..width {
            if contents[y][x] == "X" {
                matches += check_coord_for_xmas(&contents, x, y);
            }
        }
    }

    println!("Total: {matches}");

    // ------------ Part 2 ------------
    
    let height = contents.len();
    let width = contents[0].len();

    let mut matches = 0;
    for y in 0..height {
        for x in 0..width {
            if contents[y][x] == "A" {
                matches += check_coord_for_x_max(&contents, x, y);
            }
        }
    }
    println!("Total: {matches}");
}

fn check_coord_for_x_max(contents: &Vec<Vec<&str>>, x: usize, y: usize) -> i32 {
    let height = contents.len();
    let width = contents[0].len();

    if x > 0 && y > 0 && x < width - 1 && y < height - 1 {
        let top_left = contents[y - 1][x - 1];
        let top_right = contents[y - 1][x + 1];
        let bottom_left = contents[y + 1][x - 1];
        let bottom_right = contents[y + 1][x + 1];

        if top_left == bottom_right || top_right == bottom_left {
            return 0;
        }



        let values = Vec::from([top_left, top_right, bottom_left, bottom_right]);

        let count = values.iter().fold(0i32, |count, value| {
            if *value == "S" || *value == "M" {
                return count + 1;
            } else {
                return count
            }
        });

        if count == 4 {
            return 1;
        }    
    }

    return 0;
}

fn check_coord_for_xmas(contents: &Vec<Vec<&str>>, x: usize, y: usize) -> i32 {
    let height = contents.len();
    let width = contents[0].len();

    let mut total = 0;
    // Check left
    if x > 2 {
        total += check_line_for_xmas(contents, x, y, -1, 0);
    }
    // Check right
    if x < width - 3 {
        total += check_line_for_xmas(contents, x, y, 1, 0);
    }
    // Check down
    if y < height - 3 {
        total += check_line_for_xmas(contents, x, y, 0, 1);
    }
    // Check up
    if y > 2 {
        total += check_line_for_xmas(contents, x, y, 0, -1);
    }
    // Check diagonal (Up, Left)
    if x > 2 && y > 2 {
        total += check_line_for_xmas(contents, x, y, -1, -1);
    }
    // Check diagonal (Up, Right)
    if x < width - 3 && y > 2 {
        total += check_line_for_xmas(contents, x, y, 1, -1);
    }
    // Check diagonal (Down, Left)
    if x > 2 && y < height - 3 {
        total += check_line_for_xmas(contents, x, y, -1, 1);
    }
    // Check diagonal (Down, Right)
    if x < width - 3 && y < height - 3 {
        total += check_line_for_xmas(contents, x, y, 1, 1);
    }

    return total;
}

fn check_line_for_xmas(contents: &Vec<Vec<&str>>, x: usize, y: usize, xdirection: i32, ydirection: i32) -> i32 {
    if contents[(y as i32 + ydirection * 1) as usize][(x as i32 + xdirection * 1) as usize] != "M" { return 0 }
    if contents[(y as i32 + ydirection * 2) as usize][(x as i32 + xdirection * 2) as usize] != "A" { return 0 }
    if contents[(y as i32 + ydirection * 3) as usize][(x as i32 + xdirection * 3) as usize] != "S" { return 0 }
    return 1;
}
