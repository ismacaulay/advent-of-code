use crate::utils;
use std::collections::HashMap;

fn read_problem_data() -> std::result::Result<Vec<String>, String> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day6.txt") {
        for line in lines {
            if let Ok(s) = line {
                result.push(s);
            }
        }
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 6.1:");
    let mut map: HashMap<char, bool> = HashMap::new();
    let mut count = 0;

    if let Ok(data) = read_problem_data() {
        for line in data {
            if line.len() == 0 {
                count += map.keys().count();
                map = HashMap::new();
            } else {
                for c in line.chars() {
                    map.insert(c, true);
                }
            }
        }
    }

    println!("Found count: {}", count);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 6.2:");
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut count = 0;
    let mut num_people = 0;

    if let Ok(data) = read_problem_data() {
        for line in data {
            if line.len() == 0 {
                for v in map.values() {
                    if *v == num_people {
                        count += 1;
                    }
                }

                map = HashMap::new();
                num_people = 0;
            } else {
                num_people += 1;
                for c in line.chars() {
                    let mut current = *(map.get(&c).unwrap_or(&0));
                    current += 1;
                    map.insert(c, current);
                }
            }
        }
    }

    println!("Found count: {}", count);
}
