use std::collections::HashMap;

fn compute_spoken_number(starting: &Vec<usize>, turn: usize) -> usize {
    let mut history = HashMap::new();

    let mut spoken = starting[0];
    let mut next_spoken;
    for i in 1..turn {
        if i < starting.len() {
            next_spoken = starting[i];
        } else {
            if let Some(last) = history.get(&spoken) {
                next_spoken = i - *last;
            } else {
                next_spoken = 0;
            }
        }

        history.insert(spoken, i);
        spoken = next_spoken;
    }

    return spoken;
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 15.1:");
    println!(
        "2020th spoken: {}",
        compute_spoken_number(&vec![1, 2, 16, 19, 18, 0], 2020)
    );
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 15.2:");
    println!(
        "30000000th spoken: {}",
        compute_spoken_number(&vec![1, 2, 16, 19, 18, 0], 30000000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_spoken_number() {
        assert_eq!(compute_spoken_number(&vec![0], 1), 0);
        assert_eq!(compute_spoken_number(&vec![0, 3], 2), 3);
        assert_eq!(compute_spoken_number(&vec![0, 3, 6], 3), 6);
        assert_eq!(compute_spoken_number(&vec![0, 3, 6], 4), 0);
        assert_eq!(compute_spoken_number(&vec![0, 3, 6], 5), 3);
        assert_eq!(compute_spoken_number(&vec![0, 3, 6], 2020), 436);
    }
}
