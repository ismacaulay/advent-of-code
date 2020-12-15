use crate::utils;
use std::collections::HashMap;

const TEST_MODE: bool = false;

fn read_problem_data() -> Vec<String> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day14.test.txt"
    } else {
        "data/day14.txt"
    };

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                result.push(s);
            }
        }
    }

    result
}

fn apply_mask(mask: &String, value: usize) -> usize {
    let mut masked_value = value;
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '1' => {
                masked_value = masked_value | (1 << i);
            }
            '0' => {
                masked_value = masked_value & !(1 << i);
            }
            _ => {}
        }
    }
    masked_value
}

fn sum_memory(mem: &HashMap<usize, usize>) -> usize {
    let mut sum = 0;
    for v in mem.values() {
        sum += v;
    }
    return sum;
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 14.1:");
    let mut mem = HashMap::new();
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for line in read_problem_data() {
        if line.starts_with("mask =") {
            mask = String::from(line.strip_prefix("mask = ").unwrap());
        } else if line.starts_with("mem[") {
            let parts: Vec<&str> = line.split(" ").collect();
            let addr = parts[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let value = parts[2].parse::<usize>().unwrap();

            mem.insert(addr, apply_mask(&mask, value));
        }
    }

    println!("Found sum: {}", sum_memory(&mem));
}

fn find_combinations(addr: &String) -> Vec<String> {
    if let Some(idx) = addr.find('X') {
        let mut v = Vec::new();
        let mut chars: Vec<char> = addr.chars().collect();
        chars[idx] = '0';
        v.extend(find_combinations(&chars.clone().into_iter().collect()));
        chars[idx] = '1';
        v.extend(find_combinations(&chars.into_iter().collect()));
        return v;
    }

    return vec![addr.clone()];
}

fn compute_addrs(addr: &String) -> Vec<usize> {
    let mut addrs = Vec::new();
    for c in find_combinations(addr) {
        addrs.push(usize::from_str_radix(&c, 2).unwrap());
    }
    return addrs;
}

fn apply_address_mask(mask: &String, addr: usize) -> Vec<usize> {
    let mut addr_chars: Vec<char> = format!("{:036b}", addr).chars().collect();

    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => {
                addr_chars[i] = '1';
            }
            'X' => {
                addr_chars[i] = 'X';
            }
            _ => {}
        }
    }

    return compute_addrs(&addr_chars.into_iter().collect());
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 14.2:");
    let mut mem = HashMap::new();
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for line in read_problem_data() {
        if line.starts_with("mask =") {
            mask = String::from(line.strip_prefix("mask = ").unwrap());
        } else if line.starts_with("mem[") {
            let parts: Vec<&str> = line.split(" ").collect();
            let addr = parts[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let value = parts[2].parse::<usize>().unwrap();

            for addr in apply_address_mask(&mask, addr) {
                mem.insert(addr, value);
            }
        }
    }

    println!("Found sum: {}", sum_memory(&mem));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask() {
        assert_eq!(
            apply_mask(&String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 11),
            73
        );
    }

    #[test]
    fn test_apply_address_mask() {
        assert_eq!(
            apply_address_mask(&String::from("000000000000000000000000000000X1001X"), 42),
            vec![26, 27, 58, 59]
        );

        assert_eq!(
            apply_address_mask(&String::from("00000000000000000000000000000000X0XX"), 26),
            vec![16, 17, 18, 19, 24, 25, 26, 27]
        );
    }
}
