use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn is_numeric_string_in_range(s: &str, min: usize, max: usize) -> bool {
    if let Ok(num) = s.parse::<usize>() {
        return min <= num && num <= max;
    }
    return false;
}
