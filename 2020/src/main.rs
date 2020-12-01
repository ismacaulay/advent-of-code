use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_problem1_data() -> io::Result<Vec<i32>> {
    let mut result = Vec::new();

    if let Ok(lines) = read_lines("data/problem_1.txt") {
        for line in lines {
            if let Ok(s) = line {
                let v = s.parse::<i32>().unwrap();
                result.push(v);
            }
        }
    }

    Ok(result)
}

fn problem_1a() {
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

fn problem_1b() {
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

fn main() {
    // problem_1a();
    problem_1b();
}
