use crate::utils;

const TEST_MODE: bool = true;

fn read_problem_data() -> Vec<String> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day13.test.txt"
    } else {
        "data/day13.txt"
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

fn compute_wait_time(estimate: u32, bus_id: u32) -> u32 {
    ((estimate as f32 / bus_id as f32).ceil() as u32 * bus_id) - estimate
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 13.1:");
    let data = read_problem_data();
    let estimate = data[0].parse::<u32>().unwrap();
    let mut bus_ids = Vec::new();
    for id in data[1].split(",") {
        if id != "x" {
            bus_ids.push(id.parse::<u32>().unwrap());
        }
    }

    let mut bus_id = bus_ids[0];
    let mut min_wait = compute_wait_time(estimate, bus_id);
    for id in bus_ids[1..].iter() {
        let wait = compute_wait_time(estimate, *id);
        if wait < min_wait {
            min_wait = wait;
            bus_id = *id;
        }
    }

    println!(
        "Found bus: {}, wait: {}, sol: {}",
        bus_id,
        min_wait,
        bus_id * min_wait
    );
}

#[derive(Debug)]
struct Bus {
    id: u64,
    t: u64,
}

fn parse_schedule(s: &String) -> Vec<Bus> {
    let mut buses = Vec::new();
    for (idx, bus) in s.split(",").enumerate() {
        if bus != "x" {
            buses.push(Bus {
                id: bus.parse::<u64>().unwrap(),
                t: idx as u64,
            });
        }
    }
    buses
}

// Needed help on this one, was way to hard for me to figure out on my own!
// Saw lots of solutions related to the chinese remainder theorem (CRT), but I don't
// think this is that, but I was not in the mood to re-learn the CRT. Found this
// solution on reddit, but don't understand the math behind it and also not in the
// mood to figure it out. ¯\_(ツ)_/¯
fn compute_timestamp(schedule: &Vec<Bus>) -> u64 {
    let mut t: u64 = 0;
    let mut step: u64 = 1;

    for b in schedule {
        while (t + b.t) % b.id != 0 {
            t += step;
        }
        step *= b.id;
    }
    return t;
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 13.2:");

    println!("tests");
    assert_eq!(
        compute_timestamp(&parse_schedule(&String::from("17,x,13,19"))),
        3417
    );
    assert_eq!(
        compute_timestamp(&parse_schedule(&String::from("1789,37,47,1889"))),
        1202161486
    );
    assert_eq!(
        compute_timestamp(&parse_schedule(&String::from("67,7,59,61"))),
        754018
    );

    let data = read_problem_data();
    let schedule = parse_schedule(&data[1]);
    println!("{:?}", schedule);
    println!("Computed: {}", compute_timestamp(&schedule));
}
