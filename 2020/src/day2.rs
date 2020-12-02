use crate::utils;
use std::io;

#[derive(Debug)]
struct Entry {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

fn read_problem_data() -> io::Result<Vec<Entry>> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day2.txt") {
        for line in lines {
            if let Ok(s) = line {
                let data: Vec<&str> = s.split(' ').collect();
                let min_max: Vec<&str> = data[0].split("-").collect();
                result.push(Entry {
                    min: min_max[0].parse::<i32>().unwrap(),
                    max: min_max[1].parse::<i32>().unwrap(),
                    letter: data[1].as_bytes()[0] as char,
                    password: String::from(data[2]),
                });
            }
        }
    }

    Ok(result)
}

pub fn problem_2a() {
    println!("running problem 2a:");

    fn is_entry_valid(entry: &Entry) -> bool {
        let mut count = 0;
        for c in entry.password.chars() {
            if c == entry.letter {
                count += 1;
            }
        }

        return entry.min <= count && count <= entry.max;
    }

    let mut num_valid = 0;
    if let Ok(entries) = read_problem_data() {
        for entry in entries {
            if is_entry_valid(&entry) {
                num_valid += 1;
            }
        }
    }

    println!("Found {} valid entries", num_valid);
}

#[allow(dead_code)]
pub fn problem_2b() {
    println!("running problem 2b:");
    fn is_entry_valid(entry: &Entry) -> bool {
        let chars: Vec<char> = entry.password.chars().collect();
        let i = (entry.min - 1) as usize;
        let j = (entry.max - 1) as usize;

        if chars[i] == entry.letter {
            return chars[j] != entry.letter;
        }

        if chars[i] != entry.letter {
            return chars[j] == entry.letter;
        }

        return false;
    }

    let mut num_valid = 0;
    if let Ok(entries) = read_problem_data() {
        for entry in entries {
            if is_entry_valid(&entry) {
                num_valid += 1;
            }
        }
    }

    println!("Found {} valid entries", num_valid);
}
