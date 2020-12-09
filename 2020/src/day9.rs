use crate::utils;
use std::collections::HashMap;

const TEST_MODE: bool = false;

fn read_problem_data() -> Vec<i64> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day9.test.txt"
    } else {
        "data/day9.txt"
    };

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                result.push(s.parse::<i64>().unwrap());
            }
        }
    }

    result
}

fn has_sum(data: &[i64], sum: i64) -> bool {
    let mut map = HashMap::new();

    for v in data {
        if let Some(_) = map.get(v) {
            return true;
        }

        let diff = i64::abs(sum - v);
        map.insert(diff, v);
    }

    return false;
}

fn find_first_without_sum(data: &Vec<i64>, preamble_len: usize) -> i64 {
    let mut start = 0;
    let mut idx = preamble_len;

    while idx < data.len() {
        let sum = data[idx];

        if !has_sum(&data[start..start + preamble_len], sum) {
            return sum;
        }

        start += 1;
        idx += 1;
    }

    return 0;
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 9.1:");

    let data = read_problem_data();
    if TEST_MODE {
        println!("Found: {}", find_first_without_sum(&data, 5));
    } else {
        println!("Found: {}", find_first_without_sum(&data, 25));
    }
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 9.2:");
    let invalid_num = if TEST_MODE { 127 } else { 14360655 };
    // let invalid_num = 14360655;
    let data = read_problem_data();

    let mut start = 0;
    let mut end = 1;
    while end < data.len() {
        let sum: i64 = data[start..end].iter().sum();
        if sum == invalid_num {
            let mut sorted = data[start..end].to_vec();
            sorted.sort();
            println!(
                "Found {}, {}; sum: {}",
                sorted.first().unwrap(),
                sorted.last().unwrap(),
                sorted.first().unwrap() + sorted.last().unwrap(),
            );
            break;
        } else if sum < invalid_num {
            end += 1;
        } else {
            start += 1;
            end = start;
        }
    }
}
