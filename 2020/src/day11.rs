use crate::utils;

const TEST_MODE: bool = false;

fn read_problem_data() -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let path = if TEST_MODE {
        "data/day11.test.txt"
    } else {
        "data/day11.txt"
    };

    if let Ok(lines) = utils::read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                let chars: Vec<char> = s.chars().collect();
                result.push(chars);
            }
        }
    }

    result
}

fn num_adjacent_occupied(layout: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    if y > 0 {
        if x > 0 && layout[y - 1][x - 1] == '#' {
            count += 1;
        }

        if layout[y - 1][x] == '#' {
            count += 1;
        }

        if x < layout[y - 1].len() - 1 && layout[y - 1][x + 1] == '#' {
            count += 1;
        }
    }

    if x > 0 && layout[y][x - 1] == '#' {
        count += 1;
    }

    if x < layout[y].len() - 1 && layout[y][x + 1] == '#' {
        count += 1;
    }

    if y < layout.len() - 1 {
        if x > 0 && layout[y + 1][x - 1] == '#' {
            count += 1;
        }

        if layout[y + 1][x] == '#' {
            count += 1;
        }

        if x < layout[y + 1].len() - 1 && layout[y + 1][x + 1] == '#' {
            count += 1;
        }
    }

    count
}

fn is_occupied(layout: &Vec<Vec<char>>, pos: (usize, usize), offset: (i8, i8)) -> bool {
    let (x, y) = pos;
    if y == layout.len() {
        return false;
    }

    if x == layout[y].len() {
        return false;
    }

    let c = layout[y][x];

    if c == '#' {
        return true;
    }

    if c == 'L' {
        return false;
    }

    let (x_offset, y_offset) = offset;
    if y == 0 && y_offset == -1 {
        return false;
    }

    if x == 0 && x_offset == -1 {
        return false;
    }

    return is_occupied(
        layout,
        (
            ((x as i8) + x_offset) as usize,
            ((y as i8) + y_offset) as usize,
        ),
        offset,
    );
}

fn num_diag_occupied(layout: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    if y > 0 {
        if x > 0 && is_occupied(layout, (x - 1, y - 1), (-1, -1)) {
            count += 1;
        }

        if is_occupied(layout, (x, y - 1), (0, -1)) {
            count += 1;
        }

        if x < layout[y - 1].len() && is_occupied(layout, (x + 1, y - 1), (1, -1)) {
            count += 1;
        }
    }

    if x > 0 && is_occupied(layout, (x - 1, y), (-1, 0)) {
        count += 1;
    }

    if x < layout[y].len() - 1 && is_occupied(layout, (x + 1, y), (1, 0)) {
        count += 1;
    }

    if y < layout.len() - 1 {
        if x > 0 && is_occupied(layout, (x - 1, y + 1), (-1, 1)) {
            count += 1;
        }

        if is_occupied(layout, (x, y + 1), (0, 1)) {
            count += 1;
        }

        if x < layout[y + 1].len() - 1 && is_occupied(layout, (x + 1, y + 1), (1, 1)) {
            count += 1;
        }
    }

    count
}

fn count_occupied(layout: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in layout {
        count += row.iter().filter(|&c| *c == '#').count();
    }
    count
}

fn find_next_state1(layout: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let c = layout[y][x];
    let n = num_adjacent_occupied(layout, x, y);
    if c == 'L' && n == 0 {
        return '#';
    } else if c == '#' && n >= 4 {
        return 'L';
    }

    return c;
}

fn find_next_state2(layout: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let c = layout[y][x];
    let n = num_diag_occupied(layout, x, y);
    if c == 'L' && n == 0 {
        return '#';
    } else if c == '#' && n >= 5 {
        return 'L';
    }

    return c;
}

fn run_round(
    layout: &Vec<Vec<char>>,
    next_state_fn: fn(&Vec<Vec<char>>, usize, usize) -> char,
) -> (Vec<Vec<char>>, bool) {
    let mut next_layout: Vec<Vec<char>> = Vec::new();
    let mut has_changed = false;
    for y in 0..layout.len() {
        let mut next_row: Vec<char> = Vec::new();
        for x in 0..layout[y].len() {
            let next = next_state_fn(layout, x, y);
            if next != layout[y][x] {
                has_changed = true;
            }
            next_row.push(next);
        }
        next_layout.push(next_row);
    }
    (next_layout, has_changed)
}

#[allow(dead_code)]
fn print_layout(layout: &Vec<Vec<char>>) {
    for row in layout {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

fn run_problem(next_state_fn: fn(&Vec<Vec<char>>, usize, usize) -> char) {
    let mut layout = read_problem_data();
    loop {
        let (l, changed) = run_round(&layout, next_state_fn);

        if !changed {
            break;
        }
        // print_layout(&l);
        // println!("");
        layout = l;
    }
    println!("Found occupied: {}", count_occupied(&layout));
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 11.1:");
    run_problem(find_next_state1);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 11.2:");
    run_problem(find_next_state2);
}
