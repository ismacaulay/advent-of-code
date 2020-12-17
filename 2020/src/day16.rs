use crate::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const TEST_MODE: bool = false;

enum ParseState {
    Rules,
    YourTicket,
    NearbyTickets,
}

#[derive(Debug)]
struct ProblemData {
    rules: HashMap<String, ((usize, usize), (usize, usize))>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn read_problem_data() -> ProblemData {
    let path = if TEST_MODE {
        "data/day16.test2.txt"
    } else {
        "data/day16.txt"
    };

    let mut state = ParseState::Rules;
    let mut rules = HashMap::new();
    let mut your_ticket: Vec<usize> = Vec::new();
    let mut nearby_tickets = Vec::new();
    let re = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                match s.as_str() {
                    "your ticket:" => {
                        state = ParseState::YourTicket;
                    }
                    "nearby tickets:" => {
                        state = ParseState::NearbyTickets;
                    }
                    "" => {}
                    _ => match state {
                        ParseState::Rules => {
                            for cap in re.captures_iter(&s) {
                                rules.insert(
                                    String::from(&cap[1]),
                                    (
                                        (
                                            cap[2].parse::<usize>().unwrap(),
                                            cap[3].parse::<usize>().unwrap(),
                                        ),
                                        (
                                            cap[4].parse::<usize>().unwrap(),
                                            cap[5].parse::<usize>().unwrap(),
                                        ),
                                    ),
                                );
                            }
                        }
                        ParseState::YourTicket => {
                            let v: Vec<usize> =
                                s.split(',').map(|v| v.parse::<usize>().unwrap()).collect();
                            your_ticket.extend(v);
                        }
                        ParseState::NearbyTickets => {
                            nearby_tickets
                                .push(s.split(',').map(|v| v.parse::<usize>().unwrap()).collect());
                        }
                    },
                }
            }
        }
    }

    ProblemData {
        rules,
        your_ticket,
        nearby_tickets,
    }
}

fn is_valid(rules: &HashMap<String, ((usize, usize), (usize, usize))>, value: usize) -> bool {
    for r in rules.values() {
        if in_rule(*r, value) {
            return true;
        }
    }

    return false;
}

fn in_rule(rule: ((usize, usize), (usize, usize)), value: usize) -> bool {
    let (r1, r2) = rule;
    if r1.0 <= value && value <= r1.1 {
        return true;
    }

    if r2.0 <= value && value <= r2.1 {
        return true;
    }

    return false;
}
#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 16.1:");
    let data = read_problem_data();
    let mut error_rate = 0;
    let mut cache = HashMap::new();
    for t in data.nearby_tickets {
        for v in t {
            if let Some(valid) = cache.get(&v) {
                if !valid {
                    error_rate += v;
                }
            } else if !is_valid(&data.rules, v) {
                error_rate += v;
                cache.insert(v, false);
            } else {
                cache.insert(v, true);
            }
        }
    }
    println!("Error rate: {:?}", error_rate);
}

fn remove_field(
    i: usize,
    k: &String,
    fields: &mut Vec<HashSet<String>>,
    order: &mut HashMap<usize, String>,
) {
    fields[i].remove(k);

    let mut fringe = Vec::new();
    if fields[i].len() == 1 {
        let v = fields[i].iter().collect::<Vec<&String>>()[0];
        fringe.push(v.clone());
        order.insert(i, v.clone());
    }

    while !fringe.is_empty() {
        let k = fringe.pop().unwrap();
        for j in 0..fields.len() {
            fields[j].remove(&k);
            if fields[j].len() == 1 {
                for f in fields[j].iter() {
                    fringe.push(f.clone());
                    order.insert(j, f.clone());
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 16.2:");
    let data = read_problem_data();
    let mut cache = HashMap::new();
    let mut valid_nearby_tickets = Vec::new();

    for t in data.nearby_tickets.iter() {
        let mut ticket_valid = true;
        for v in t {
            if let Some(valid) = cache.get(&v) {
                if !valid {
                    ticket_valid = false;
                    break;
                }
            } else if !is_valid(&data.rules, *v) {
                cache.insert(v, false);
                ticket_valid = false;
                break;
            } else {
                cache.insert(v, true);
            }
        }

        if ticket_valid {
            valid_nearby_tickets.push(t.clone());
        }
    }

    let mut field_set = HashSet::new();
    for k in data.rules.keys() {
        field_set.insert(k.clone());
    }

    let mut fields = Vec::new();
    for _ in 0..field_set.len() {
        fields.push(field_set.clone());
    }

    let mut order = HashMap::new();
    for t in valid_nearby_tickets {
        for i in 0..t.len() {
            if let Some(_) = order.get(&i) {
                continue;
            }

            let v = t[i];
            for (k, r) in data.rules.iter() {
                if !in_rule(*r, v) {
                    remove_field(i, k, &mut fields, &mut order);
                }
            }
        }
    }

    let mut total = 1;
    for (idx, v) in order.iter() {
        if v.starts_with("departure") {
            total *= data.your_ticket[*idx];
        }
    }
    println!("Found total: {:?}", total);
}
