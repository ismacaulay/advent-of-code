use crate::utils;
use std::io::{self};

fn read_problem_data() -> io::Result<Vec<Vec<char>>> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day3.txt") {
        for line in lines {
            if let Ok(s) = line {
                result.push(s.chars().collect());
            }
        }
    }

    Ok(result)
}

fn traverse_slope(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let map_width = map[0].len();

    let mut i = 0;
    let mut j = 0;
    while j < map.len() {
        if map[j][i] == '#' {
            trees += 1;
        }

        i += right;
        i = i % map_width;
        j += down;
    }

    return trees;
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 3.1:");
    let map = read_problem_data().unwrap();
    println!("Trees encountered: {}", traverse_slope(&map, 3, 1));
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 3.2:");
    let map = read_problem_data().unwrap();
    let result = traverse_slope(&map, 1, 1)
        * traverse_slope(&map, 3, 1)
        * traverse_slope(&map, 5, 1)
        * traverse_slope(&map, 7, 1)
        * traverse_slope(&map, 1, 2);
    println!("Product of trees encountered: {}", result);
}
