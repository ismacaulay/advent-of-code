use crate::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn read_problem_data() -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day7.txt") {
        for line in lines {
            if let Ok(s) = line {
                result.push(s);
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 7.1:");
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let bag_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    for line in read_problem_data() {
        if line.ends_with("no other bags.") {
            continue;
        }

        let data: Vec<&str> = line.split(" bags contain").collect();
        let internal_bags = data[1];
        for bag in internal_bags.split(",") {
            for cap in bag_re.captures_iter(bag) {
                if let Some(cur) = map.get_mut(&cap[2]) {
                    cur.push(String::from(data[0]));
                } else {
                    let cur = vec![String::from(data[0])];
                    map.insert(String::from(&cap[2]), cur);
                }
            }
        }
    }

    let mut fringe = vec![String::from("shiny gold")];
    let mut set = HashSet::new();
    while fringe.len() > 0 {
        if let Some(v) = fringe.pop() {
            if let Some(colors) = map.get(&v) {
                for c in colors {
                    set.insert(c.clone());
                    fringe.push(c.clone());
                }
            }
        }
    }

    println!("Found: {}", set.len());
}

fn recursive_search(map: &HashMap<String, Vec<(String, String)>>, key: &String) -> usize {
    let mut count = 0;
    if let Some(values) = map.get(key) {
        for (c, v) in values {
            let num_bags = c.parse::<usize>().unwrap();
            let num_bags_in_bag = recursive_search(&map, v);
            count += num_bags + (num_bags * num_bags_in_bag);
        }
    }
    return count;
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 7.2:");
    let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    let bag_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    for line in read_problem_data() {
        if line.ends_with("no other bags.") {
            continue;
        }

        let data: Vec<&str> = line.split(" bags contain").collect();
        let mut internal_bags = Vec::new();
        for bag in data[1].split(",") {
            for cap in bag_re.captures_iter(bag) {
                internal_bags.push((String::from(&cap[1]), String::from(&cap[2])));
            }
        }
        map.insert(String::from(data[0]), internal_bags);
    }

    let count = recursive_search(&map, &String::from("shiny gold"));
    println!("Found: {}", count);
}
