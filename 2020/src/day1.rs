use crate::utils;
use std::collections::HashMap;
use std::io::{self};

fn read_problem1_data() -> io::Result<Vec<i32>> {
    let mut result = Vec::new();

    if let Ok(lines) = utils::read_lines("data/day1.txt") {
        for line in lines {
            if let Ok(s) = line {
                let v = s.parse::<i32>().unwrap();
                result.push(v);
            }
        }
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn problem_1a() {
    println!("running problem 1a:");

    let mut map = HashMap::new();
    if let Ok(data) = read_problem1_data() {
        for a in data {
            match map.get(&a) {
                Some(b) => {
                    println!(
                        "found a: {}, b: {}, sum: {}, product: {}",
                        a,
                        b,
                        a + b,
                        a * b
                    );
                    return;
                }
                None => {
                    let b = 2020 - a;
                    map.insert(b, a);
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn problem_1b() {
    println!("running problem 1b:");

    let mut map = HashMap::new();
    if let Ok(data) = read_problem1_data() {
        for a in data.iter() {
            let b = 2020 - a;
            map.insert(b, *a);
        }

        for i in 0..data.len() {
            for j in 1..data.len() {
                let b = data[i];
                let c = data[j];
                if let Some(a) = map.get(&(b + c)) {
                    println!(
                        "found a: {}, b: {}, c: {}, sum: {}, product: {}",
                        a,
                        b,
                        c,
                        a + b + c,
                        a * b * c,
                    );
                    return;
                }
            }
        }
    }
}
