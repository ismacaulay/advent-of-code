use crate::utils;

const TEST_MODE: bool = false;

#[derive(Debug)]
struct NavInstruction {
    action: String,
    value: i32,
}

fn read_problem_data() -> Vec<NavInstruction> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day12.test.txt"
    } else {
        "data/day12.txt"
    };

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                let action = &s[0..1];
                let value = &s[1..].parse::<i32>().unwrap();

                result.push(NavInstruction {
                    action: String::from(action),
                    value: *value,
                });
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 12.1:");

    let mut north = 0;
    let mut east = 0;
    let mut dir = 0;

    for ins in read_problem_data() {
        match ins.action.as_str() {
            "N" => {
                north += ins.value;
            }
            "S" => {
                north -= ins.value;
            }
            "E" => {
                east += ins.value;
            }
            "W" => {
                east -= ins.value;
            }
            "L" => {
                dir += ins.value;
                while dir >= 360 {
                    dir -= 360;
                }
            }
            "R" => {
                dir -= ins.value;
                while dir < 0 {
                    dir += 360;
                }
            }
            "F" => match dir {
                0 => {
                    east += ins.value;
                }
                90 => {
                    north += ins.value;
                }
                180 => {
                    east -= ins.value;
                }
                270 => {
                    north -= ins.value;
                }
                _ => {
                    panic!("Unknown dir: {}", dir);
                }
            },
            _ => {}
        }
    }

    println!(
        "N: {}, E: {}, Manhattan dist: {}",
        north,
        east,
        north.abs() + east.abs()
    );
}

pub fn deg2rad(deg: i32) -> f32 {
    return (deg as f32) * std::f32::consts::PI / 180.0;
}

pub fn rotate(p: (i32, i32), deg: i32) -> (i32, i32) {
    let cos = deg2rad(deg).cos();
    let sin = deg2rad(deg).sin();

    let x = p.0 as f32;
    let y = p.1 as f32;

    return (
        (x * cos - y * sin).round() as i32,
        (x * sin + y * cos).round() as i32,
    );
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 12.2:");

    let mut wp_north = 1;
    let mut wp_east = 10;
    let mut ship_north = 0;
    let mut ship_east = 0;

    for ins in read_problem_data() {
        match ins.action.as_str() {
            "N" => {
                wp_north += ins.value;
            }
            "S" => {
                wp_north -= ins.value;
            }
            "E" => {
                wp_east += ins.value;
            }
            "W" => {
                wp_east -= ins.value;
            }
            "L" => {
                let (e, n) = rotate((wp_east, wp_north), ins.value);
                wp_north = n;
                wp_east = e;
            }
            "R" => {
                let (e, n) = rotate((wp_east, wp_north), -ins.value);
                wp_north = n;
                wp_east = e;
            }
            "F" => {
                ship_north += ins.value * wp_north;
                ship_east += ins.value * wp_east;
            }
            _ => {}
        }
    }
    println!(
        "N: {}, E: {}, Manhattan dist: {}",
        ship_north,
        ship_east,
        ship_north.abs() + ship_east.abs()
    );
}
