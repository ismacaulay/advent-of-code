use crate::utils;
use std::collections::HashMap;

const TEST_MODE: bool = false;

fn read_problem_data() -> Vec<usize> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day10.test.txt"
    } else {
        "data/day10.txt"
    };

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                result.push(s.parse::<usize>().unwrap());
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 10.1:");

    let mut data = read_problem_data();
    data.sort();

    let mut one_diff = 0;
    let mut three_diff = 1;
    let mut current = 0;
    for adapter in data {
        let diff = adapter - current;
        if diff == 1 {
            one_diff += 1;
        } else if diff == 3 {
            three_diff += 1;
        }

        current = adapter;
    }

    println!(
        "Found 1: {}, 3: {}, 1x3: {}",
        one_diff,
        three_diff,
        one_diff * three_diff
    );
}

fn count_paths(data: &[usize], current: usize, path_cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(c) = path_cache.get(&current) {
        return *c;
    }

    if data.len() == 0 {
        return 1;
    }

    let mut paths = 0;
    for (idx, adapter) in data.iter().enumerate() {
        if adapter - current > 3 {
            break;
        }

        let found_paths = count_paths(&data[(idx + 1)..], data[idx], path_cache);
        if found_paths > 0 {
            paths += found_paths;
        }
    }

    path_cache.insert(current, paths);
    return paths;
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 10.2:");
    let mut data = read_problem_data();
    data.sort();

    let mut path_cache: HashMap<usize, usize> = HashMap::new();
    println!("Found paths: {}", count_paths(&data, 0, &mut path_cache));
}
