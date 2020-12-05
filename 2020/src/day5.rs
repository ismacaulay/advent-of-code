use crate::utils;

fn read_problem_data() -> std::result::Result<Vec<String>, String> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day5.txt") {
        for line in lines {
            if let Ok(s) = line {
                result.push(s);
            }
        }
    }

    Ok(result)
}

fn compute_range(l: char, lower: usize, upper: usize) -> (usize, usize) {
    let mut new_lower = lower;
    let mut new_upper = upper;

    let diff = upper - lower;

    if l == 'F' || l == 'L' {
        new_upper = new_lower + (diff as f32 / 2.0).floor() as usize;
    } else if l == 'B' || l == 'R' {
        new_lower = new_lower + (diff as f32 / 2.0).ceil() as usize;
    }

    (new_lower, new_upper)
}

fn compute_row_col(bp: &String) -> (usize, usize) {
    let mut row = 0;
    let mut col = 0;
    let mut lower = 0;
    let mut upper = 127;

    for (i, c) in bp.chars().enumerate() {
        match i {
            0..=5 => {
                let (nl, nu) = compute_range(c, lower, upper);
                lower = nl;
                upper = nu;
            }
            6 => {
                if c == 'F' {
                    row = std::cmp::min(lower, upper);
                } else {
                    row = std::cmp::max(lower, upper);
                }

                lower = 0;
                upper = 7;
            }
            7..=8 => {
                let (nl, nu) = compute_range(c, lower, upper);
                lower = nl;
                upper = nu;
            }
            9 => {
                if c == 'L' {
                    col = std::cmp::min(lower, upper);
                } else {
                    col = std::cmp::max(lower, upper);
                }
            }
            _ => {}
        }
    }

    (row, col)
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 5.1:");

    // println!("Test {:?}", compute_row_col(&String::from("FBFBBFFRLR")));
    // println!("Test {:?}", compute_row_col(&String::from("BFFFBBFRRR")));
    // println!("Test {:?}", compute_row_col(&String::from("FFFBBBFRRR")));
    // println!("Test {:?}", compute_row_col(&String::from("BBFFBBFRLL")));

    let mut largest_seat_id = 0;
    if let Ok(lines) = read_problem_data() {
        for line in lines {
            let (row, col) = compute_row_col(&line);
            largest_seat_id = std::cmp::max(largest_seat_id, row * 8 + col);
        }
    }

    println!("Largest seat id {}", largest_seat_id);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 5.2:");

    let mut seat_ids = Vec::new();
    if let Ok(lines) = read_problem_data() {
        for line in lines {
            let (row, col) = compute_row_col(&line);
            seat_ids.push(row * 8 + col);
        }
    }

    seat_ids.sort();

    for i in 1..seat_ids.len() {
        let diff = seat_ids[i] - seat_ids[i - 1];
        if diff == 2 {
            println!("My seat id is: {}", seat_ids[i] - 1);
            break;
        }
    }
}
