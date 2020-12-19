use crate::utils;
use std::collections::HashMap;

const TEST_MODE: bool = false;

#[derive(Debug)]
enum Rule {
    Single(Vec<String>),
    Double((Vec<String>, Vec<String>)),
    Sink(char),
}

struct ProblemData {
    rules: HashMap<String, Rule>,
    messages: Vec<String>,
}

fn read_problem_data() -> ProblemData {
    let path = if TEST_MODE {
        "data/day19.test2.txt"
    } else {
        "data/day19.txt"
    };

    let mut processing_rules = true;
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                if processing_rules {
                    if s == "" {
                        processing_rules = false;
                    } else {
                        let parts = s.split(":").collect::<Vec<&str>>();
                        let rule_num = parts[0];

                        let rule: Rule;
                        if parts[1].contains("|") {
                            let parts = parts[1].split("|").collect::<Vec<&str>>();
                            let first: Vec<String> =
                                parts[0].trim().split(" ").map(|s| s.to_string()).collect();
                            let second: Vec<String> =
                                parts[1].trim().split(" ").map(|s| s.to_string()).collect();
                            rule = Rule::Double((first, second));
                        } else if parts[1].contains("\"") {
                            let parts = parts[1].trim().split("\"").collect::<Vec<&str>>();
                            let chars = parts[1].chars().collect::<Vec<char>>();
                            rule = Rule::Sink(chars[0]);
                        } else {
                            let first: Vec<String> =
                                parts[1].trim().split(" ").map(|s| s.to_string()).collect();

                            rule = Rule::Single(first);
                        }

                        rules.insert(rule_num.to_string(), rule);
                    }
                } else {
                    messages.push(s);
                }
            }
        }
    }

    ProblemData {
        rules: rules,
        messages: messages,
    }
}

fn matches_single(
    tokens: &[char],
    idx: usize,
    values: &Vec<String>,
    rules: &HashMap<String, Rule>,
) -> (bool, usize) {
    let mut count = 0;
    for id in values.iter() {
        // println!("[matches_single] rule: {}, idx: {}", id, idx + count);
        let (m, i) = matches_rule(&tokens, idx + count, rules.get(id).unwrap(), rules);
        // println!("[matches_single] m: {}, i: {}", m, i);
        if !m {
            return (false, 0);
        }
        count += i;
    }
    return (true, count);
}

fn matches_rule(
    tokens: &[char],
    idx: usize,
    rule: &Rule,
    rules: &HashMap<String, Rule>,
) -> (bool, usize) {
    // println!(
    //     "[matches_rule] idx: {}, rule: {:?}, tokens: {:?}",
    //     idx, rule, tokens
    // );
    if idx >= tokens.len() {
        return (false, 0);
    }

    match rule {
        Rule::Single(values) => {
            return matches_single(tokens, idx, values, rules);
        }
        Rule::Double((lhs, rhs)) => {
            let (m, i) = matches_single(tokens, idx, lhs, rules);
            if m {
                return (true, i);
            }

            let (m, i) = matches_single(tokens, idx, rhs, rules);
            if m {
                return (true, i);
            }

            return (false, 0);
        }
        Rule::Sink(c) => {
            return (tokens[idx] == *c, 1);
        }
    }
}

fn is_valid(message: &String, rules: &HashMap<String, Rule>) -> bool {
    let tokens = message.chars().collect::<Vec<char>>();
    let rule_id = String::from("0");
    let (m, i) = matches_rule(&tokens, 0, rules.get(&rule_id).unwrap(), rules);
    return m && i == message.len();
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 19.1:");

    let data = read_problem_data();
    let mut num_valid = 0;
    for m in data.messages.iter() {
        if is_valid(m, &data.rules) {
            num_valid += 1;
        }
    }

    println!("Found {} valid", num_valid);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 19.2:");

    // let mut data = read_problem_data();
    // data.rules.insert(
    //     String::from("8"),
    //     Rule::Double((
    //         vec![String::from("42")],
    //         vec![String::from("42"), String::from("8")],
    //     )),
    // );
    // data.rules.insert(
    //     String::from("11"),
    //     Rule::Double((
    //         vec![String::from("42"), String::from("31")],
    //         vec![String::from("42"), String::from("11"), String::from("31")],
    //     )),
    // );
    //
    // println!("valid: {}", is_valid(&data.messages[2], &data.rules));
    // let mut num_valid = 0;
    // for m in data.messages.iter() {
    //     println!("checking {}", m);
    //     if is_valid(m, &data.rules) {
    //         println!("VALID");
    //         num_valid += 1;
    //     }
    // }
    // println!("Found {} valid", num_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let mut rules: HashMap<String, Rule> = HashMap::new();
        rules.insert(
            String::from("0"),
            Rule::Single(vec![
                String::from("4"),
                String::from("1"),
                String::from("5"),
            ]),
        );
        rules.insert(
            String::from("1"),
            Rule::Double((
                vec![String::from("2"), String::from("3")],
                vec![String::from("3"), String::from("2")],
            )),
        );
        rules.insert(
            String::from("2"),
            Rule::Double((
                vec![String::from("4"), String::from("4")],
                vec![String::from("5"), String::from("5")],
            )),
        );
        rules.insert(
            String::from("3"),
            Rule::Double((
                vec![String::from("4"), String::from("5")],
                vec![String::from("5"), String::from("4")],
            )),
        );
        rules.insert(String::from("4"), Rule::Sink('a'));
        rules.insert(String::from("5"), Rule::Sink('b'));

        let cases = vec![
            ("ababbb", true),
            ("abbbab", true),
            ("bababa", false),
            ("aaabbb", false),
            ("aaaabbb", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_valid(&String::from(input), &rules),
                expected,
                "{} did not match expected",
                input
            );
        }
    }
}
