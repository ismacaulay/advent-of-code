use crate::utils;
use std::iter::FromIterator;

const TEST_MODE: bool = false;

fn read_problem_data() -> Vec<Vec<char>> {
    let path = if TEST_MODE {
        "data/day17.test.txt"
    } else {
        "data/day17.txt"
    };

    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                result.push(s.chars().collect());
            }
        }
    }

    result
}

fn get_cube_state(idx: (isize, isize, isize), dimension: &Vec<Vec<Vec<char>>>) -> char {
    let (x, y, z) = idx;

    if 0 <= z && z < dimension.len() as isize {
        let z = z as usize;
        if 0 <= y && y < dimension[z].len() as isize {
            let y = y as usize;
            if 0 <= x && x < dimension[z][y].len() as isize {
                return dimension[z][y][x as usize];
            }
        }
    }

    return '.';
}

fn num_active_neighbours(idx: (isize, isize, isize), dimension: &Vec<Vec<Vec<char>>>) -> isize {
    let mut num_active = 0;
    for z in -1..2 {
        for y in -1..2 {
            for x in -1..2 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                if get_cube_state((idx.0 + x, idx.1 + y, idx.2 + z), dimension) == '#' {
                    num_active += 1;
                }
            }
        }
    }
    num_active
}

fn compute_cube_state(idx: (isize, isize, isize), dimension: &Vec<Vec<Vec<char>>>) -> char {
    let current = get_cube_state(idx, dimension);
    let active_neighbors = num_active_neighbours(idx, dimension);

    if current == '#' {
        if active_neighbors == 2 || active_neighbors == 3 {
            return '#';
        }
    } else {
        if active_neighbors == 3 {
            return '#';
        }
    }

    return '.';
}

fn run_cycle(dimension: &Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut new_dimension = Vec::new();

    let z_len = (dimension.len() + 1) as isize;
    let y_len = (dimension[0].len() + 1) as isize;
    let x_len = (dimension[0][0].len() + 1) as isize;

    for z in -1..z_len {
        let mut y_row = Vec::new();
        for y in -1..y_len {
            let mut x_row = Vec::new();
            for x in -1..x_len {
                x_row.push(compute_cube_state((x, y, z), dimension));
            }
            y_row.push(x_row);
        }
        new_dimension.push(y_row);
    }

    new_dimension
}

fn print(dimension: &Vec<Vec<Vec<char>>>) {
    for (i, z) in dimension.iter().enumerate() {
        println!("z = {}", i);
        for y in z {
            println!("{}", String::from_iter(y));
        }
        println!("");
    }
}

fn count_active(dimension: &Vec<Vec<Vec<char>>>) -> usize {
    let mut active = 0;
    for z in dimension {
        for y in z {
            for x in y {
                if *x == '#' {
                    active += 1;
                }
            }
        }
    }
    active
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 17.1:");

    let data = read_problem_data();
    let mut dimension = vec![data];
    for _ in 0..6 {
        dimension = run_cycle(&dimension);
    }

    println!("Num active: {}", count_active(&dimension));
}

fn get_cube_state_4d(
    idx: (isize, isize, isize, isize),
    dimension: &Vec<Vec<Vec<Vec<char>>>>,
) -> char {
    let (x, y, z, w) = idx;

    if 0 <= w && w < dimension.len() as isize {
        let w = w as usize;
        if 0 <= z && z < dimension[w].len() as isize {
            let z = z as usize;
            if 0 <= y && y < dimension[w][z].len() as isize {
                let y = y as usize;
                if 0 <= x && x < dimension[w][z][y].len() as isize {
                    return dimension[w][z][y][x as usize];
                }
            }
        }
    }

    return '.';
}

fn num_active_neighbours_4d(
    idx: (isize, isize, isize, isize),
    dimension: &Vec<Vec<Vec<Vec<char>>>>,
) -> isize {
    let mut num_active = 0;
    for w in -1..2 {
        for z in -1..2 {
            for y in -1..2 {
                for x in -1..2 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }

                    if get_cube_state_4d((idx.0 + x, idx.1 + y, idx.2 + z, idx.3 + w), dimension)
                        == '#'
                    {
                        num_active += 1;
                    }
                }
            }
        }
    }
    num_active
}
fn compute_cube_state_4d(
    idx: (isize, isize, isize, isize),
    dimension: &Vec<Vec<Vec<Vec<char>>>>,
) -> char {
    let current = get_cube_state_4d(idx, dimension);
    let active_neighbors = num_active_neighbours_4d(idx, dimension);

    if current == '#' {
        if active_neighbors == 2 || active_neighbors == 3 {
            return '#';
        }
    } else {
        if active_neighbors == 3 {
            return '#';
        }
    }

    return '.';
}

fn run_cycle_4d(dimension: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut new_dimension = Vec::new();

    let w_len = (dimension.len() + 1) as isize;
    let z_len = (dimension[0].len() + 1) as isize;
    let y_len = (dimension[0][0].len() + 1) as isize;
    let x_len = (dimension[0][0][0].len() + 1) as isize;

    for w in -1..w_len {
        let mut z_row = Vec::new();
        for z in -1..z_len {
            let mut y_row = Vec::new();
            for y in -1..y_len {
                let mut x_row = Vec::new();
                for x in -1..x_len {
                    x_row.push(compute_cube_state_4d((x, y, z, w), dimension));
                }
                y_row.push(x_row);
            }
            z_row.push(y_row);
        }
        new_dimension.push(z_row);
    }

    new_dimension
}

fn count_active_4d(dimension: &Vec<Vec<Vec<Vec<char>>>>) -> usize {
    let mut active = 0;
    for w in dimension {
        for z in w {
            for y in z {
                for x in y {
                    if *x == '#' {
                        active += 1;
                    }
                }
            }
        }
    }
    active
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 17.2:");

    let data = read_problem_data();
    let mut dimension = vec![vec![data]];
    for _ in 0..6 {
        dimension = run_cycle_4d(&dimension);
    }

    println!("Num active: {}", count_active_4d(&dimension));
}
