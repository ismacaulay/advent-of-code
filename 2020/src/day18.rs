use crate::utils;

fn read_problem_data() -> Vec<String> {
    let path = "data/day18.txt";

    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                result.push(s);
            }
        }
    }

    result
}

fn tokenize(expression: &String) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in expression.split(" ") {
        match token {
            "+" => tokens.push(String::from("+")),
            "*" => tokens.push(String::from("*")),
            _ => {
                if token.starts_with("(") {
                    let mut t = token;
                    while t.starts_with("(") {
                        tokens.push(String::from("("));
                        t = t.strip_prefix("(").unwrap();
                    }

                    tokens.push(String::from(t));
                } else if token.ends_with(")") {
                    let mut t = token;
                    let mut num = 0;
                    while t.ends_with(")") {
                        t = t.strip_suffix(")").unwrap();
                        num += 1;
                    }

                    tokens.push(String::from(t));
                    for _ in 0..num {
                        tokens.push(String::from(")"));
                    }
                } else {
                    tokens.push(String::from(token));
                }
            }
        }
    }
    tokens
}

fn compute(lhs: &Option<usize>, rhs: usize, op: &Option<String>) -> Option<usize> {
    if let Some(lhs) = lhs {
        if let Some(op) = op {
            if op == "+" {
                return Some(lhs + rhs);
            }

            return Some(lhs * rhs);
        }
    }
    return Some(rhs);
}

fn solve(expression: &String) -> usize {
    let mut lhs = None;
    let mut op = None;
    let mut stack = Vec::new();

    for token in tokenize(expression) {
        match token.as_str() {
            "+" | "*" => op = Some(token),
            "(" => {
                stack.push((lhs, op));
                lhs = None;
                op = None;
            }
            ")" => {
                let (l, o) = stack.pop().unwrap();
                lhs = compute(&l, lhs.unwrap(), &o);
            }
            _ => {
                lhs = compute(&lhs, token.parse::<usize>().unwrap(), &op);
            }
        }
    }

    lhs.unwrap()
}

fn multiply(values: &Vec<Option<usize>>) -> usize {
    let mut result = 1;
    for v in values {
        if let Some(v) = v {
            result *= v;
        }
    }
    result
}

fn solve2(expression: &String) -> usize {
    let mut lhs = None;
    let mut op = None;
    let mut stack = Vec::new();
    let mut mul = Vec::new();

    for token in tokenize(expression) {
        match token.as_str() {
            "+" => op = Some(token),
            "*" => {
                mul.push(lhs);
                lhs = None;
            }

            "(" => {
                stack.push((lhs, op, mul));
                lhs = None;
                op = None;
                mul = Vec::new();
            }
            ")" => {
                mul.push(lhs);
                let (l, o, m) = stack.pop().unwrap();
                lhs = compute(&l, multiply(&mul), &o);
                mul = m;
            }
            _ => {
                lhs = compute(&lhs, token.parse::<usize>().unwrap(), &op);
            }
        }
    }

    mul.push(lhs);
    multiply(&mul)
}
#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 18.1:");
    let mut total = 0;
    for expression in read_problem_data() {
        total += solve(&expression);
    }

    println!("Total: {}", total);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 18.2:");
    let mut total = 0;
    for expression in read_problem_data() {
        total += solve2(&expression);
    }

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        assert_eq!(solve(&String::from("1 + 2 * 3 + 4 * 5 + 6")), 71);
        assert_eq!(solve(&String::from("1 + (2 * 3) + (4 * (5 + 6))")), 51);
        assert_eq!(solve(&String::from("2 * 3 + (4 * 5)")), 26);
        assert_eq!(solve(&String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 437);
        assert_eq!(
            solve(&String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            12240
        );
        assert_eq!(
            solve(&String::from(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            13632
        );
    }

    #[test]
    fn test_solver2() {
        assert_eq!(solve2(&String::from("1 + 2 * 3 + 4 * 5 + 6")), 231);
        assert_eq!(solve2(&String::from("1 + (2 * 3) + (4 * (5 + 6))")), 51);
        assert_eq!(solve2(&String::from("2 * 3 + (4 * 5)")), 46);
        assert_eq!(solve2(&String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 1445);
        assert_eq!(
            solve2(&String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669060
        );
        assert_eq!(
            solve2(&String::from(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            23340
        );
    }
}
